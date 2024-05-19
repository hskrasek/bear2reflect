// @generated automatically by Diesel CLI.
#![allow(clippy::all)]
#![allow(non_snake_case)]

use diesel::joinable;

diesel::table! {
    ZSFNOTE (Z_PK) {
        Z_PK -> Integer,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZARCHIVED -> Nullable<Integer>,
        ZENCRYPTED -> Nullable<Integer>,
        ZHASFILES -> Nullable<Integer>,
        ZHASIMAGES -> Nullable<Integer>,
        ZHASSOURCECODE -> Nullable<Integer>,
        ZLOCKED -> Nullable<Integer>,
        ZORDER -> Nullable<Integer>,
        ZPERMANENTLYDELETED -> Nullable<Integer>,
        ZPINNED -> Nullable<Integer>,
        ZSHOWNINTODAYWIDGET -> Nullable<Integer>,
        ZSKIPSYNC -> Nullable<Integer>,
        ZTODOCOMPLETED -> Nullable<Integer>,
        ZTODOINCOMPLETED -> Nullable<Integer>,
        ZTRASHED -> Nullable<Integer>,
        ZVERSION -> Nullable<Integer>,
        ZPASSWORD -> Nullable<Integer>,
        ZSERVERDATA -> Nullable<Integer>,
        ZARCHIVEDDATE -> Nullable<Timestamp>,
        ZCONFLICTUNIQUEIDENTIFIERDATE -> Nullable<Timestamp>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZLOCKEDDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZORDERDATE -> Nullable<Timestamp>,
        ZPINNEDDATE -> Nullable<Timestamp>,
        ZTRASHEDDATE -> Nullable<Timestamp>,
        ZCONFLICTUNIQUEIDENTIFIER -> Nullable<Text>,
        ZENCRYPTIONUNIQUEIDENTIFIER -> Nullable<Text>,
        ZLASTEDITINGDEVICE -> Nullable<Text>,
        ZSUBTITLE -> Nullable<Text>,
        ZTEXT -> Nullable<Text>,
        ZTITLE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
        ZENCRYPTEDDATA -> Nullable<Binary>,
        ZVECTORCLOCK -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFNOTETAG (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZISROOT -> Nullable<Integer>,
        ZPINNED -> Nullable<Integer>,
        ZSORTING -> Nullable<Integer>,
        ZSORTINGDIRECTION -> Nullable<Integer>,
        ZVERSION -> Nullable<Integer>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZPINNEDDATE -> Nullable<Timestamp>,
        ZSORTINGDATE -> Nullable<Timestamp>,
        ZSORTINGDIRECTIONDATE -> Nullable<Timestamp>,
        ZTAGCONDATE -> Nullable<Timestamp>,
        ZTAGCON -> Nullable<Text>,
        ZTITLE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
        ZSERVERDATA -> Nullable<Binary>,
    }
}

diesel::table! {
    Z_5TAGS (Z_5NOTES, Z_13TAGS) {
        Z_5NOTES -> Nullable<Integer>,
        Z_13TAGS -> Nullable<Integer>,
    }
}

joinable!(Z_5TAGS -> ZSFNOTE (Z_5NOTES));
joinable!(Z_5TAGS -> ZSFNOTETAG (Z_13TAGS));

diesel::allow_tables_to_appear_in_same_query!(ZSFNOTE, ZSFNOTETAG, Z_5TAGS,);
