//! Reqwest error

/* std use */

/* crate use */

/* std use */

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Reqwest {
    #[error("Error durring request twitch authentification: {error}")]
    Twitch { error: reqwest::Error },
}
