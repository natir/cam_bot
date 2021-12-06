//! Define error type in cam_bot

/* std use */

/* crate use */

/* std use */

/* mod declaration */
pub mod db;
pub mod irc;
pub mod log;
pub mod obs;
pub mod server;
pub mod twitch;

/* pub use */
pub use self::db::*;
pub use self::irc::*;
pub use self::log::*;
pub use self::obs::*;
pub use self::server::*;
pub use self::twitch::*;

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] Db),

    #[error(transparent)]
    Server(#[from] Box<Server>),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
