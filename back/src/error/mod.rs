//! Define error type in cam_bot backend

/* std use */

/* crate use */

/* std use */

/* mod declaration */
pub mod db;
pub mod irc;
pub mod obs;
pub mod reqwest;
pub mod rocket;
pub mod twitch;

/* pub use */
pub use self::db::*;
pub use self::irc::*;
pub use self::obs::*;
pub use self::reqwest::*;
pub use self::rocket::*;
pub use self::twitch::*;

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] Db),

    #[error(transparent)]
    Rocket(#[from] Box<Rocket>),

    #[error(transparent)]
    Reqwest(#[from] Reqwest),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
