table! {
    commands (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
        activate -> Bool,
    }
}

table! {
    timers (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
        time_th -> BigInt,
        message_th -> BigInt,
        activate -> Bool,
    }
}

table! {
    twitch (id) {
        id -> Nullable<Integer>,
        token -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        expire_in -> Nullable<Integer>,
        generation_date -> Nullable<Date>,
    }
}

allow_tables_to_appear_in_same_query!(commands, timers, twitch,);
