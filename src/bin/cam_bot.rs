//! Main cam_bot binary

/* std use */

/* crate use */
use clap::Parser;

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

#[tokio::main]
pub async fn main() -> error::Result<()> {
    let args = Command::parse();

    /* Setup log level */
    if let Some(level) = i82level(args.verbosity) {
        env_logger::builder()
            .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
            .filter_level(level.to_level_filter())
            .init();
    } else {
        env_logger::Builder::from_env("CAM_BOT_LOG")
            .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
            .init();
    }

    back::run(args.config).await.map_err(error::Error::Back)
}
