// @generated automatically by Diesel CLI.

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
    ZSFNOTEBACKLINK (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZLOCATION -> Nullable<Integer>,
        ZVERSION -> Nullable<Integer>,
        ZLINKEDBY -> Nullable<Integer>,
        ZLINKINGTO -> Nullable<Integer>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZTITLE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
        ZSERVERDATA -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFNOTEFILE (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZDOWNLOADED -> Nullable<Integer>,
        ZFILESIZE -> Nullable<Integer>,
        ZINDEX -> Nullable<Integer>,
        ZPERMANENTLYDELETED -> Nullable<Integer>,
        ZSKIPSYNC -> Nullable<Integer>,
        ZUNUSED -> Nullable<Integer>,
        ZUPLOADED -> Nullable<Integer>,
        ZVERSION -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZSERVERDATA -> Nullable<Integer>,
        ZANIMATED -> Nullable<Integer>,
        ZHEIGHT -> Nullable<Integer>,
        ZWIDTH -> Nullable<Integer>,
        ZDURATION -> Nullable<Integer>,
        ZHEIGHT1 -> Nullable<Integer>,
        ZWIDTH1 -> Nullable<Integer>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZINSERTIONDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZSEARCHTEXTDATE -> Nullable<Timestamp>,
        ZUNUSEDDATE -> Nullable<Timestamp>,
        ZUPLOADEDDATE -> Nullable<Timestamp>,
        ZFILENAME -> Nullable<Text>,
        ZLASTEDITINGDEVICE -> Nullable<Text>,
        ZNORMALIZEDFILEEXTENSION -> Nullable<Text>,
        ZSEARCHTEXT -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFNOTEFILESERVERDATA (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZFILE -> Nullable<Integer>,
        Z7_FILE -> Nullable<Integer>,
        ZSYSTEMFIELDS -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFNOTESERVERDATA (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZSYSTEMFIELDS -> Nullable<Binary>,
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

diesel::allow_tables_to_appear_in_same_query!(
    ZSFNOTE,
    ZSFNOTEBACKLINK,
    ZSFNOTEFILE,
    ZSFNOTEFILESERVERDATA,
    ZSFNOTESERVERDATA,
    ZSFNOTETAG,
    Z_5TAGS,
);
