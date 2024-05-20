extern crate clap;
extern crate diesel;

use std::collections::BTreeMap;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use futures::future::join_all;
use inflector::Inflector;
use regex::Regex;
use serde_json::json;
use tokio::task;

use self::models::{Note, NoteTag, Tag};
use self::schema::ZSFNOTE::dsl::*;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// TODO: Utilize the following paths to include note media in the migration, when supported.
// Source: https://github.com/mivok/bear_backup/blob/master/bear_backup.py
// Paths for building out the notes with full files
// approot = os.path.expanduser("~/Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear")
// dbpath = os.path.join(approot, "Application Data/database.sqlite")
// assetpath = os.path.join(approot, "Application Data/Local Files")
// imagepath = os.path.join(assetpath, "Note Images")
// filepath = os.path.join(assetpath, "Note Files")

async fn establish_connection() -> DbPool {
    let database_url = "sqlite://bear.sqlite".to_string(); // TODO: Update this to be dynamic/default to Bear location
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to establish database pool with internal Bear database.")
}

async fn fetch_active_notes(pool: DbPool) -> Vec<Note> {
    let mut connection = pool
        .get()
        .expect("Failed to fetch a connection from the database pool.");

    task::spawn_blocking(move || {
        ZSFNOTE
            .filter(ZTRASHED.eq(0))
            .filter(ZARCHIVED.eq(0))
            .select(Note::as_select())
            .load(&mut connection)
            .expect("Error loading notes from the Bear database.")
    })
    .await
    .expect("Task for fetching notes failed.")
}

async fn fetch_tags_for_notes(
    pool: DbPool,
    notes: Vec<Note>,
) -> Vec<(Tag, Option<i32>)> {
    let mut connection = pool
        .get()
        .expect("Failed to fetch a connection from the database pool.");

    task::spawn_blocking(move || {
        NoteTag::belonging_to(&notes)
            .inner_join(schema::ZSFNOTETAG::table)
            .select((Tag::as_select(), schema::Z_5TAGS::Z_5NOTES.nullable()))
            .load::<(Tag, Option<i32>)>(&mut connection)
            .expect("Failed to load tags")
    })
    .await.expect("")
}

async fn collapse_root_tags_with_nested_tags(tags: Vec<&Tag>) -> Vec<&Tag> {
    if tags.len() <= 1 {
        return tags;
    }
    // Sort tags by the is_root property in descending order
    let mut sorted_tags = tags.clone();
    sorted_tags.sort_by(|tag_a, tag_b| tag_b.is_root.unwrap_or(0).cmp(&tag_a.is_root.unwrap_or(0)));
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

                            break;
                        }
                    }
                }

                if found {
                    sorted_tags.remove(i);
                    continue;
                }
            }
        }

        i += 1;
    }

    sorted_tags
}

async fn replace_tags_in_text(text: Option<&mut String>, tags: Vec<&Tag>) -> Option<String> {
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

        return Some(new_text);
    }

    None
}

async fn substitute_tags_for_backlinks<'a>(note: &'a mut Note, tags: Vec<&Tag>) -> &'a mut Note {
    note.title = replace_tags_in_text(note.title.as_mut(), tags.clone()).await;
    note.subtitle = replace_tags_in_text(note.subtitle.as_mut(), tags.clone()).await;
    note.text = replace_tags_in_text(note.text.as_mut(), tags.clone()).await;

    note
}

#[tokio::main]
async fn main() {
    let pool: DbPool = establish_connection().await;

    let notes: Vec<Note> = fetch_active_notes(pool.clone()).await;

    let tags: Vec<(Tag, Option<i32>)> = fetch_tags_for_notes(pool.clone(), notes.clone()).await;

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
                let z_pk = note.id;
                let empty_vec: Vec<&Tag> = Vec::new();

                collapse_root_tags_with_nested_tags(
                    grouped_tags_by_note
                        .get(&z_pk)
                        .unwrap_or(&empty_vec)
                        .to_vec(),
                )
                .await
            };

            substitute_tags_for_backlinks(&mut note, tags).await;

            note
        }
    });

    let notes = join_all(note_futures).await;

    println!("Preparing note for Reflect API...");

    for note in notes {
        let note_json = json!({
            "subject": note.title,
            "content_markdown": note.text,
        });

        println!("{}", note_json);
    }
}
