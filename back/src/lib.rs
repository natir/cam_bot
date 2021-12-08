//! A backend for cam_bot

/* std use */

/* crate use */
#[macro_use]
extern crate rocket_sync_db_pools; /* I don't try to understand but rocket need this */
#[macro_use]
extern crate diesel; /* I don't try to understand but diesel need this */
#[macro_use]
extern crate diesel_migrations;

use figment::providers::Format as _;

/* project use */

/* mod declaration */
pub mod api;
pub mod db;
pub mod error;
pub mod front;
pub mod irc;
pub mod obs;
pub mod twitch;

/* pub use */
pub use self::error::Error;

/* Global variable */
#[database("sqlite_db")]
pub struct Dbconn(diesel::SqliteConnection);

async fn run_migration(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    diesel_migrations::embed_migrations!();

    let conn = (Dbconn::get_one(&rocket))
        .await
        .ok_or(error::Db::Connection)
        .unwrap();
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("can run migrations");

    rocket
}

pub async fn run(rocket_config: std::path::PathBuf) -> std::result::Result<(), error::Error> {
    /* load rocket config */
    let config = figment::Figment::from(rocket::Config::default())
        .merge(figment::providers::Toml::file(rocket_config).nested());

    /* create rocket object */
    let server = rocket::custom(config)
        .attach(rocket_dyn_templates::Template::fairing())
        .attach(Dbconn::fairing())
        .attach(rocket::fairing::AdHoc::on_ignite(
            "Run Migrations",
            run_migration,
        ))
        .mount("/", rocket::routes![front::file])
        .mount("/api/commands", api::commands::routes())
        .mount("/api/timers", api::timers::routes())
        .mount("/api/twitch", api::twitch::routes())
        .mount("/front", rocket::fs::FileServer::from("front"));

    /* launch server */
    server
        .launch()
        .await
        .map_err(|error| Error::Rocket(Box::new(error::Rocket::Execution { error })))
}
