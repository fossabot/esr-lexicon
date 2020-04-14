extern crate clap;

use env_logger::Env;
use futures::future::join_all;
use std::collections::HashSet;
use std::error::Error;
use tokio;
use tokio::task::JoinHandle;

mod cli;
mod config;
mod download;
mod process;
mod save;
mod utils;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut process_tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];

    let matches = cli::cli();
    info!("ESR lexicon 🕮");

    if !matches.is_present("silent") {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
    }

    let sources: Vec<config::Source> = match (
        matches.value_of("config"),
        matches.value_of("url"),
        matches.value_of("file"),
    ) {
        (Some(config), None, None) => {
            config::parse_config(utils::open_file(config).await?).expect("Failed to parse config")
        }
        (None, Some(url), None) => vec![config::Source {
            source: config::SourceType::Url(url.into()),
            jq: matches
                .value_of("jq")
                .expect("--jq is required in this context")
                .into(),
            output: matches
                .value_of("output")
                .expect("--output is required in this context")
                .into(),
        }],
        (None, None, Some(path)) => vec![config::Source {
            source: config::SourceType::FilePath(path.into()),
            jq: matches
                .value_of("jq")
                .expect("--jq is required in this context")
                .into(),
            output: matches
                .value_of("output")
                .expect("--output is required in this context")
                .into(),
        }],
        _ => vec![],
    };

    for source in sources.clone() {
        process_tasks.push(tokio::spawn(async move {
            process::run(source)
                .await
                .expect("Something went wrong while processing data");
            Ok(())
        }));
    }

    join_all(process_tasks).await;

    let mut dedup_tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];

    let output_files = sources
        .into_iter()
        .map(|source| source.output)
        .collect::<HashSet<String>>();

    for file in output_files {
        dedup_tasks.push(tokio::spawn(async move {
            save::dedup_file(&file).await.expect("Failed to dedup data");
            Ok(())
        }));
    }

    join_all(dedup_tasks).await;

    Ok(())
}
