//! Define error type in cam_bot

/* std use */

/* crate use */

/* std use */

/* mod declaration */

/* pub use */

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Back(#[from] back::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
