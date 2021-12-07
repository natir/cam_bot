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
        id -> Integer,
        token -> Text,
        refresh_token -> Text,
        expire_in -> Integer,
        generation_date -> Date,
    }
}
