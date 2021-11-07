//! Definition of db schema

/* std use */

/* crate use */

/* project use */

diesel::table! {
    commands {
    id -> Integer,
    name -> Text,
    value -> Text,
    activate -> Bool,
    }
}

diesel::table! {
    timers {
    id -> Integer,
    name -> Text,
    value -> Text,
    time -> Integer,
    activate -> Bool,
    }
}

diesel::table! {
    twitch {
    id -> Integer,
    token -> Text,
    refresh_token -> Text,
    expire_in -> Integer,
    generation_date -> Date,
    }
}
