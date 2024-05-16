use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::ZSFNOTE)]
#[diesel(primary_key(Z_PK))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub Z_PK: i32,
    pub Z_ENT: Option<i32>,
    pub Z_OPT: Option<i32>,
    pub ZARCHIVED: Option<i32>,
    pub ZENCRYPTED: Option<i32>,
    pub ZHASFILES: Option<i32>,
    pub ZHASIMAGES: Option<i32>,
    pub ZHASSOURCECODE: Option<i32>,
    pub ZLOCKED: Option<i32>,
    pub ZORDER: Option<i32>,
    pub ZPERMANENTLYDELETED: Option<i32>,
    pub ZPINNED: Option<i32>,
    pub ZSHOWNINTODAYWIDGET: Option<i32>,
    pub ZSKIPSYNC: Option<i32>,
    pub ZTODOCOMPLETED: Option<i32>,
    pub ZTODOINCOMPLETED: Option<i32>,
    pub ZTRASHED: Option<i32>,
    pub ZVERSION: Option<i32>,
    pub ZPASSWORD: Option<i32>,
    pub ZSERVERDATA: Option<i32>,
    pub ZCONFLICTUNIQUEIDENTIFIER: Option<String>,
    pub ZENCRYPTIONUNIQUEIDENTIFIER: Option<String>,
    pub ZLASTEDITINGDEVICE: Option<String>,
    pub ZSUBTITLE: Option<String>,
    pub ZTEXT: Option<String>,
    pub ZTITLE: Option<String>,
    pub ZUNIQUEIDENTIFIER: Option<String>,
    pub ZENCRYPTEDDATA: Option<Vec<u8>>,
    pub ZVECTORCLOCK: Option<Vec<u8>>,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
#[diesel(table_name = crate::schema::ZSFNOTETAG)]
#[diesel(primary_key(Z_PK))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    #[diesel(column_name = "Z_PK")]
    pub id: Option<i32>,
    #[diesel(column_name = "ZISROOT")]
    pub is_root: Option<i32>,
    #[diesel(column_name = "ZPINNED")]
    pub pinned: Option<i32>,
    #[diesel(column_name = "ZSORTING")]
    pub sorting: Option<i32>,
    #[diesel(column_name = "ZSORTINGDIRECTION")]
    pub sorting_direction: Option<i32>,
    #[diesel(column_name = "ZVERSION")]
    pub version: Option<i32>,
    #[diesel(column_name = "ZTAGCON")]
    pub tagcon: Option<String>,
    #[diesel(column_name = "ZTITLE")]
    pub title: Option<String>,
    #[diesel(column_name = "ZUNIQUEIDENTIFIER")]
    pub unique_identifier: Option<String>,
    #[diesel(column_name = "ZSERVERDATA")]
    pub server_data: Option<Vec<u8>>,
}

#[derive(Selectable, Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::Z_5TAGS, primary_key(Z_5NOTES, Z_13TAGS))]
#[diesel(belongs_to(Note, foreign_key = Z_5NOTES))]
#[diesel(belongs_to(Tag, foreign_key = Z_13TAGS))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoteTag {
    #[diesel(column_name = "Z_5NOTES")]
    pub note_id: Option<i32>,
    #[diesel(column_name = "Z_13TAGS")]
    pub tag_id: Option<i32>,
}
