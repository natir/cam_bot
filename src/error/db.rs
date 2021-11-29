//! Db error

/* std use */

/* crate use */

/* std use */

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Db {
    #[error("Failled to create connection with database")]
    Connection,
}
