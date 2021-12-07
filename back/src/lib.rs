//! A backend for cam_bot

/* std use */

/* crate use */
#[macro_use]
extern crate rocket_sync_db_pools; /* I don't try to understand but rocket need this */
#[macro_use]
extern crate diesel; /* I don't try to understand but diesel need this */

/* std use */

/* mod declaration */
pub mod api;
pub mod db;
pub mod front;
pub mod irc;
pub mod obs;
pub mod twitch;

#[database("sqlite_db")]
pub struct Dbconn(diesel::SqliteConnection);