// @generated automatically by Diesel CLI.

diesel::table! {
    likes (id) {
        id -> Bigint,
        created_at -> Timestamp,
        tweet_id -> Bigint,
    }
}

diesel::table! {
    tweets (id) {
        id -> Bigint,
        created_at -> Timestamp,
        message -> Text,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));

diesel::allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);
