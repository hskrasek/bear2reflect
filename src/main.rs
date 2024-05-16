extern crate diesel;
extern crate dotenv;

use std::collections::BTreeMap;
use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use inflector::Inflector;
use regex::Regex;

use crate::models::{Note, NoteTag, Tag};
use crate::schema::ZSFNOTE::dsl::*;
use crate::schema::ZSFNOTE::ZTRASHED;

mod schema;
mod models;

// Paths for building out the notes with full files
// # Paths to various files
// approot = os.path.expanduser("~/Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear")
// dbpath = os.path.join(approot, "Application Data/database.sqlite")
// assetpath = os.path.join(approot, "Application Data/Local Files")
// imagepath = os.path.join(assetpath, "Note Images")
// filepath = os.path.join(assetpath, "Note Files")

pub async fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub async fn fetch_active_notes(connection: &mut SqliteConnection) -> Vec<Note> {
    ZSFNOTE
        .filter(ZTRASHED.eq(0))
        .select(Note::as_select())
        .load(connection)
        .expect("Error loading notes")
}

pub async fn fetch_tags_for_notes(connection: &mut SqliteConnection, notes: &Vec<Note>) -> Vec<(Tag, Option<i32>)> {
    NoteTag::belonging_to(notes)
        .inner_join(schema::ZSFNOTETAG::table)
        .select((Tag::as_select(), schema::Z_5TAGS::Z_5NOTES.nullable()))
        .load::<(Tag, Option<i32>)>(connection)
        .expect("Failed to load tags")
}

pub async fn substitute_tags_for_backlinks<'a>(note: &'a mut Note, tags: Vec<&Tag>) -> &'a mut Note {
    let re = Regex::new(r"#([a-zA-Z0-9]+(\s[a-zA-Z0-9]+)?)#?").unwrap();

    for tag in tags {
        if let Some(is_root) = tag.is_root {
            if is_root != 1 {
                println!("Skipping non-root tag: {}", tag.title.clone().unwrap_or_default());
                continue;
            }

            if let Some(ref mut text) = note.ZTEXT {
                let new_text = re.replace_all(
                    text,
                    &format!(r"[[{}]]", tag.title.clone().unwrap_or_default().to_title_case())
                );
                dbg!(&new_text);
                *text = new_text.to_string();
            }
        } else {
            println!("Tag is missing 'is_root': {:?}", tag);
        }
    }

    note
}

#[tokio::main]
async fn main() {
    let connection: &mut SqliteConnection = &mut establish_connection().await;

    let mut notes: Vec<Note> = fetch_active_notes(connection).await;

    let tags: Vec<(Tag, Option<i32>)> = fetch_tags_for_notes(connection, &notes).await;
    // Group tags by the i32 in the tuple
    let mut grouped_tags_by_note = BTreeMap::<i32, Vec<&Tag>>::new();

    for tag_note_tuple in tags.iter() {
        grouped_tags_by_note
            .entry(tag_note_tuple.1.unwrap_or(0))
            .or_default()
            .push(&tag_note_tuple.0);
    }

    // Drop this
    // Iterate through the notes, converting tags into backlinks in the note text
    for note in &mut notes {
        let tags = {
            let z_pk = note.Z_PK.clone();
            let empty_vec: Vec<&Tag> = Vec::new();
            grouped_tags_by_note.get(&z_pk).unwrap_or(&empty_vec).to_vec()
        };

        substitute_tags_for_backlinks(note, tags).await;
    }

    // println!("{:?}", notes.first().unwrap());

    // let tag: &(Tag, Option<i32>) = tags.first().unwrap();
    // dbg!(tag);

    // let printable_note = note.unwrap();
    //
    // let note_id = printable_note.Z_PK;
    // let note_title = printable_note.ZTITLE.unwrap();
    // let note_text = printable_note.ZTEXT.unwrap();
    //
    // println!("{}: {}", note_id, note_title);
    // println!("-----------\n");
    // println!("Displaying {} tags", tags.len());
    // for tag in tags {
    //     println!("{}", tag.title.unwrap());
    // }

    // println!("Preparing note for Reflect API...");
    // let note_json = json!({
    //     "subject": note_title,
    //     "content_markdown": note_text,
    //     "tags": tags.into_iter().map(|tag| tag.title.clone().unwrap()).collect::<Vec<String>>(),
    // });
    //
    // println!("{}", note_json.to_string());
}
