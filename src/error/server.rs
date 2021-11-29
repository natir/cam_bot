//! Rocket error

/* std use */

/* crate use */

/* std use */

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Server {
    #[error("Error durring rocket execution: {error}")]
    Execution { error: rocket::Error },
}
