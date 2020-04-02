extern crate clap;

use env_logger::Env;
use std::error::Error;
use tokio::task::JoinHandle;
use tokio;
use futures::future::join_all;

mod cli;
mod config;
mod download;
mod process;
mod save;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tasks: Vec<JoinHandle<Result<(), ()>>>= vec![];

    let matches = cli::cli();
    info!("ESR lexicon 🕮");

    if !matches.is_present("silent") {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
    }

    let sources = if matches.is_present("config") {
        let config = matches.value_of("config").unwrap_or("config.json");
        config::parse_config(config::open_file(config)).unwrap()
    } else {
        vec!(config::Source {
            url: matches.value_of("url").expect("--url is required in this context").into(),
            jq: matches.value_of("jq").expect("--jq is required in this context").into(),
            output: matches.value_of("output").expect("--output is required in this context").into(),
        })
    };

    for source in sources {
        tasks.push(tokio::spawn(async move {
            process::run(source).await.expect("Something went wrong while processing data");
            Ok(())
        }));
    }

    join_all(tasks).await;

    Ok(())
}
