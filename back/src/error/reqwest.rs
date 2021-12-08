//! Reqwest error

/* std use */

/* crate use */

/* std use */

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Reqwest {
    #[error("Error durring request twitch authentification: {error}")]
    Twitch { error: reqwest::Error },

    #[error("Twitch authentification not return Ok code: {code}")]
    Authorize { code: reqwest::StatusCode },

    #[error("Twitch token not return Ok code: {code}")]
    Token { code: reqwest::StatusCode },

    #[error("Error durring deserialization of twitch token answer {error}")]
    DeserilizeToken { error: reqwest::Error },
}
