table! {
    commands (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        value -> Nullable<Text>,
        activate -> Nullable<Bool>,
    }
}

table! {
    timers (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        value -> Nullable<Text>,
        time -> Nullable<Integer>,
        activate -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(
    commands,
    timers,
);
