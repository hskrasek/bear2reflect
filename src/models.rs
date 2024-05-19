use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::ZSFNOTE)]
#[diesel(primary_key(Z_PK))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    #[diesel(column_name = Z_PK)]
    pub id: i32,
    #[diesel(column_name = Z_ENT)]
    pub ent: Option<i32>,
    #[diesel(column_name = Z_OPT)]
    pub opt: Option<i32>,
    #[diesel(column_name = ZARCHIVED)]
    pub archived: Option<i32>,
    #[diesel(column_name = ZENCRYPTED)]
    pub encrypted: Option<i32>,
    #[diesel(column_name = ZHASFILES)]
    pub has_files: Option<i32>,
    #[diesel(column_name = ZHASIMAGES)]
    pub has_images: Option<i32>,
    #[diesel(column_name = ZHASSOURCECODE)]
    pub has_source_code: Option<i32>,
    #[diesel(column_name = ZLOCKED)]
    pub locked: Option<i32>,
    #[diesel(column_name = ZORDER)]
    pub order: Option<i32>,
    #[diesel(column_name = ZPERMANENTLYDELETED)]
    pub permanently_deleted: Option<i32>,
    #[diesel(column_name = ZPINNED)]
    pub pinned: Option<i32>,
    #[diesel(column_name = ZSHOWNINTODAYWIDGET)]
    pub shown_in_today_widget: Option<i32>,
    #[diesel(column_name = ZSKIPSYNC)]
    pub skip_sync: Option<i32>,
    #[diesel(column_name = ZTODOCOMPLETED)]
    pub todo_completed: Option<i32>,
    #[diesel(column_name = ZTODOINCOMPLETED)]
    pub todo_incompleted: Option<i32>,
    #[diesel(column_name = ZTRASHED)]
    pub trashed: Option<i32>,
    #[diesel(column_name = ZVERSION)]
    pub version: Option<i32>,
    #[diesel(column_name = ZPASSWORD)]
    pub password: Option<i32>,
    #[diesel(column_name = ZSERVERDATA)]
    pub server_data: Option<i32>,
    #[diesel(column_name = ZCONFLICTUNIQUEIDENTIFIER)]
    pub conflict_unique_identifier: Option<String>,
    #[diesel(column_name = ZENCRYPTIONUNIQUEIDENTIFIER)]
    pub encryption_unique_identifier: Option<String>,
    #[diesel(column_name = ZLASTEDITINGDEVICE)]
    pub last_editing_device: Option<String>,
    #[diesel(column_name = ZSUBTITLE)]
    pub subtitle: Option<String>,
    #[diesel(column_name = ZTEXT)]
    pub text: Option<String>,
    #[diesel(column_name = ZTITLE)]
    pub title: Option<String>,
    #[diesel(column_name = ZUNIQUEIDENTIFIER)]
    pub unique_identifier: Option<String>,
    #[diesel(column_name = ZENCRYPTEDDATA)]
    pub encrypted_data: Option<Vec<u8>>,
    #[diesel(column_name = ZVECTORCLOCK)]
    pub vector_clock: Option<Vec<u8>>,
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
