//! Main cam_bot binary

/* std use */

/* crate use */
use clap::Parser;
#[macro_use]
extern crate diesel_migrations;

use figment::providers::Format as _;

/* project use */
use cam_bot::*;

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "cam_bot",
    version = "0.1",
    author = "Pierre Marijon <pierre@marijon.fr>",
    about = "A twitch bot"
)]
struct Command {
    #[clap(short = 'c', long = "config", about = "Path to configuration file")]
    pub config: std::path::PathBuf,

    #[clap(
        short = 't',
        long = "twitch",
        about = "Path to twitch configuration file"
    )]
    pub twitch: std::path::PathBuf,

    #[clap(
        short = 'v',
        long = "verbosity",
        parse(from_occurrences),
        about = "verbosity level also control by environment variable RUSTYREAD_LOG if flag is set RUSTYREAD_LOG value is ignored"
    )]
    pub verbosity: i8,
}

/// Convert verbosity level (number of v) is log::Level
pub fn i82level(level: i8) -> Option<log::Level> {
    match level {
        std::i8::MIN..=0 => None,
        1 => Some(log::Level::Error),
        2 => Some(log::Level::Warn),
        3 => Some(log::Level::Info),
        4 => Some(log::Level::Debug),
        5..=std::i8::MAX => Some(log::Level::Trace),
    }
}

async fn run_migration(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    diesel_migrations::embed_migrations!();

    let conn = (cam_bot::Dbconn::get_one(&rocket))
        .await
        .ok_or(error::Db::Connection)
        .unwrap();
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("can run migrations");

    rocket
}

#[rocket::main]
async fn main() -> error::Result<()> {
    let args = Command::parse();

    /* load rocket config */
    let config = figment::Figment::from(rocket::Config::default())
        .merge(figment::providers::Toml::file(args.config).nested());

    /* create rocket object */
    let server = rocket::custom(config)
        .attach(Dbconn::fairing())
        .attach(rocket_dyn_templates::Template::fairing())
        .attach(rocket::fairing::AdHoc::on_ignite(
            "Run Migrations",
            run_migration,
        ))
        .mount("/commands", rocket::routes![cam_bot::server::commands::index, cam_bot::server::commands::get, cam_bot::server::commands::delete]);

    /* launch server */
    server
        .launch()
        .await
        .map_err(|error| error::Error::Server(error::Server::Execution { error }))
}
