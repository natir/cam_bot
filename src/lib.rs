//! A twitch bot

/* std use */

/* crate use */
#[macro_use]
extern crate rocket; /* I don't try to understand but rocket need this */
#[macro_use]
extern crate rocket_sync_db_pools; /* I don't try to understand but rocket need this */
#[macro_use]
extern crate diesel; /* I don't try to understand but diesel need this */
#[macro_use]
extern crate diesel_migrations; /* I don't try to understand but diesel need this */

/* std use */

/* mod declaration */
pub mod db;
pub mod error;
pub mod irc;
pub mod obs;
pub mod server;
pub mod twitch;

#[database("sqlite_db")]
pub struct Dbconn(diesel::SqliteConnection);
