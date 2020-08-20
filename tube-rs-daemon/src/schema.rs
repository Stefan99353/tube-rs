table! {
    default_options (id) {
        id -> Integer,
        flag -> Text,
        val -> Nullable<Text>,
        group_id -> Integer,
    }
}

table! {
    flag_values (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
        youtube_flag_id -> Integer,
    }
}

table! {
    option_groups (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    video_options (id) {
        id -> Integer,
        flag -> Text,
        val -> Nullable<Text>,
        video_id -> Text,
    }
}

table! {
    videos (id) {
        id -> Text,
        title -> Text,
        added_at -> Text,
        downloaded -> Bool,
        path -> Text,
        quality -> Text,
    }
}

table! {
    youtube_flags (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        flag -> Text,
    }
}

joinable!(default_options -> option_groups (group_id));
joinable!(flag_values -> youtube_flags (youtube_flag_id));
joinable!(video_options -> videos (video_id));

allow_tables_to_appear_in_same_query!(
    default_options,
    flag_values,
    option_groups,
    video_options,
    videos,
    youtube_flags,
);
