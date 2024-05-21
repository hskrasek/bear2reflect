extern crate clap;
extern crate diesel;

use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::string::ToString;

use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use futures::future::join_all;
use inflector::Inflector;
use log::{debug, info};
use regex::Regex;
use serde_json::json;
use tokio::task;

use self::models::{Note, NoteTag, Tag};
use self::schema::ZSFNOTE::dsl::*;

mod models;
mod schema;

const BEAR_DB_PATH: &str =
    "~/Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear/Application Data/database.sqlite";

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Bear2Reflect {
    #[arg(value_name = "bear_db_path")]
    #[arg(default_value = BEAR_DB_PATH)]
    bear_db: Option<PathBuf>,

    #[command(flatten)]
    verbose: Verbosity,

    #[arg(short, long, help = "Dry run the migration without writing to the Reflect API")]
    dry_run: bool,
}

// TODO: Utilize the following paths to include note media in the migration, when supported.
// Source: https://github.com/mivok/bear_backup/blob/master/bear_backup.py
// Paths for building out the notes with full files
// approot = os.path.expanduser("~/Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear")
// dbpath = os.path.join(approot, "Application Data/database.sqlite")
// assetpath = os.path.join(approot, "Application Data/Local Files")
// imagepath = os.path.join(assetpath, "Note Images")
// filepath = os.path.join(assetpath, "Note Files")

async fn establish_connection(
    database_path: PathBuf,
) -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url: String = shellexpand::tilde(&database_path.into_os_string().into_string().unwrap()).to_string();
    let database_url_ref: &str = &database_url;
    let manager: ConnectionManager<SqliteConnection> =
        ConnectionManager::<SqliteConnection>::new(database_url_ref);

    info!(
        "Creating a pool of database collections connected to {}",
        database_url
    );

    Ok(r2d2::Pool::builder()
        .build(manager)
        .with_context(|| format!("Failed to establish database pool on {}.", database_url))?)
}

async fn fetch_active_notes(pool: DbPool) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let mut connection = pool
        .get()
        .with_context(|| "Failed to fetch a connection from the database pool.")?;

    Ok(task::spawn_blocking(move || {
        info!("Spawning task to fetch notes from the Bear database.");

        ZSFNOTE
            .filter(ZTRASHED.eq(0))
            .filter(ZARCHIVED.eq(0))
            .select(Note::as_select())
            .load(&mut connection)
            .expect("Failed to load notes from the Bear database.")
    })
    .await?)
}

async fn fetch_tags_for_notes(
    pool: DbPool,
    notes: Vec<Note>,
) -> Result<Vec<(Tag, Option<i32>)>, Box<dyn std::error::Error>> {
    let mut connection = pool
        .get()
        .with_context(|| "Failed to fetch a connection from the database pool.")?;

    debug!(
        "Attempting to load tags for the following notes: {:#?}",
        &notes.clone().into_iter().map(|x| { x.id })
    );

    Ok(task::spawn_blocking(move || {
        info!("Spawning task to fetch associated tags for notes.");

        NoteTag::belonging_to(&notes)
            .inner_join(schema::ZSFNOTETAG::table)
            .select((Tag::as_select(), schema::Z_5TAGS::Z_5NOTES.nullable()))
            .load::<(Tag, Option<i32>)>(&mut connection)
            .expect("Failed to load tags for the selected notes from the Bear database.")
    })
    .await?)
}

async fn collapse_root_tags_with_nested_tags(
    tags: Vec<&Tag>,
) -> Result<Vec<&Tag>, Box<dyn std::error::Error>> {
    info!("Collapsing tags vector to remove root tags, in instances where the root tag and tag nested under the root are present.");

    if tags.len() <= 1 {
        debug!(
            "Skipping tag collapsing due to one or less tags. Total: {}",
            tags.len()
        );

        return Ok(tags);
    }

    // Sort tags by the is_root property in descending order
    let mut sorted_tags = tags.clone();

    sorted_tags.sort_by(|tag_a: &&Tag, tag_b: &&Tag| {
        tag_b.is_root.unwrap_or(0).cmp(&tag_a.is_root.unwrap_or(0))
    });

    let mut i: usize = 0;

    while i < sorted_tags.len() {
        if let Some(is_root) = sorted_tags[i].is_root {
            if is_root == 1 {
                let title = sorted_tags[i].title.as_ref().unwrap();
                let mut found: bool = false;

                for other_tag in sorted_tags.iter().skip(i + 1) {
                    if let Some(ref other_title) = other_tag.title {
                        if other_title.starts_with(title) {
                            found = true;

                            debug!(
                                "Found nested tag {} that starts with root tag {}",
                                other_title, title
                            );

                            break;
                        }
                    }
                }

                if found {
                    debug!("Removing root tag {}", title);

                    sorted_tags.remove(i);

                    continue;
                }
            }
        }

        i += 1;
    }

    Ok(sorted_tags)
}

async fn replace_tags_in_text(
    text: Option<&mut String>,
    tags: Vec<&Tag>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Some(text) = text {
        let mut new_text = text.clone();

        for tag in tags {
            let regex =
                Regex::new(r"#([a-zA-Z0-9]+(\s[a-zA-Z0-9]+)?(/[a-zA-Z0-9]+(\s[a-zA-Z0-9]+)?)*)#?")
                    .unwrap();

            let tag_title = tag.title.clone().unwrap_or_default();
            let backlinks: Vec<String> = tag_title
                .split('/')
                .map(|backlink| backlink.to_title_case())
                .collect();

            let formatted_backlinks = backlinks
                .iter()
                .map(|s| format!("[[{}]]", s))
                .collect::<Vec<_>>()
                .join(" ");

            let replaced_text = &mut regex.replace_all(text, &formatted_backlinks);
            new_text = replaced_text.to_string();
        }

        return Ok(Some(new_text));
    }

    Ok(None)
}

async fn substitute_tags_for_backlinks<'a>(
    note: &'a mut Note,
    tags: Vec<&Tag>,
) -> Result<&'a mut Note, Box<dyn std::error::Error>> {
    info!(
        "Substituting tags for Reflect formatted backlinks in note; Id: {}, Title: {}",
        note.id,
        note.title.clone().unwrap_or_default()
    );

    if let Some(mut title) = note.title.take() {
        debug!("Note title before replacement: {:?}", note.title);
        note.title = replace_tags_in_text(Some(&mut title), tags.clone()).await?;
        debug!("Note title after replacement: {:?}", note.title);
    }

    if let Some(mut subtitle) = note.subtitle.take() {
        debug!("Note subtitle before replacement: {:?}", note.subtitle);
        note.subtitle = replace_tags_in_text(Some(&mut subtitle), tags.clone()).await?;
        debug!("Note subtitle after replacement: {:?}", note.subtitle);
    }

    if let Some(mut text) = note.text.take() {
        debug!("Note text before replacement: {:?}", note.text);
        note.text = replace_tags_in_text(Some(&mut text), tags).await?;
        debug!("Note text after replacement: {:?}", note.text);
    }

    Ok(note)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();

    let app: Bear2Reflect = Bear2Reflect::parse();

    env_logger::Builder::new()
        .filter_level(app.verbose.log_level_filter())
        .init();

    let database_path = app.bear_db.unwrap();

    let pool: DbPool = establish_connection(database_path).await?;

    writeln!(stdout, "Loading notes from Bear internal database...")?;

    let notes: Vec<Note> = fetch_active_notes(pool.clone()).await?;

    writeln!(stdout, "Found {} notes to migrate to Reflect", notes.len())?;

    let tags: Vec<(Tag, Option<i32>)> = fetch_tags_for_notes(pool.clone(), notes.clone()).await?;

    let mut grouped_tags_by_note: BTreeMap<i32, Vec<&Tag>> = BTreeMap::<i32, Vec<&Tag>>::new();

    for tag_note_tuple in tags.iter() {
        grouped_tags_by_note
            .entry(tag_note_tuple.1.unwrap_or(0))
            .or_default()
            .push(&tag_note_tuple.0);
    }

    // Iterate through the notes, converting tags into backlinks in the note text
    let note_futures = notes.into_iter().map(|mut note| {
        let grouped_tags_by_note = &grouped_tags_by_note;

        async move {
            let tags = {
                let z_pk: i32 = note.id;
                let empty_vec: Vec<&Tag> = Vec::new();

                collapse_root_tags_with_nested_tags(
                    grouped_tags_by_note
                        .get(&z_pk)
                        .unwrap_or(&empty_vec)
                        .to_vec(),
                )
                .await
            };

            substitute_tags_for_backlinks(&mut note, tags?).await?;

            Ok::<Note, Box<dyn std::error::Error>>(note)
        }
    });

    writeln!(stdout, "Preparing notes for Reflect...")?;

    let notes: Result<Vec<_>, _> = join_all(note_futures).await.into_iter().collect();

    let notes = match notes {
        Ok(notes) => notes,
        Err(e) => {
            writeln!(stdout, "Failed to prepare notes for Reflect: {}", e)?;

            return Ok(());
        }
    };

    for note in notes {
        let note_json = json!({
            "subject": note.title,
            "content_markdown": note.text,
        });

        debug!("{}", note_json);
    }

    Ok(())
}
