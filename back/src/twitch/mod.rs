//! Twitch relate function

/* std use */

/* crate use */

/* project use */

pub fn new_connexion() -> crate::db::twitch::Twitch {
    crate::db::twitch::Twitch {
        id: 0,
        token: "".to_string(),
        refresh_token: "".to_string(),
        expire_in: 0,
        generation_date: chrono::Utc::now().naive_utc().date(),
    }
}
