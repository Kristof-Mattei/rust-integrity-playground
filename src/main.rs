#![cfg_attr(not(debug_assertions), deny(warnings))]

use std::path::PathBuf;

use env_logger::Env;
use futures::{stream::FuturesUnordered, StreamExt};
use log::info;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    // name, // from Cargo.toml,
    about, // needed otherwise it doesn't show description from Cargo.toml,
    author // needed otherwise it doesn't show author from Cargo.toml
)]
struct Opt {
    #[structopt(
        // verbatim_doc_comment,
        help = "Some help",
        parse(from_os_str)
    )]
    some_value: PathBuf,
}

async fn something_to_await(_: PathBuf) {
    todo!("TODO");
}

async fn run_app() {
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();

    let Opt { some_value } = Opt::from_args();

    let mut tasks = FuturesUnordered::new();

    tasks.push(Box::pin(something_to_await(some_value)));

    loop {
        match tasks.next().await {
            None => {
                info!("Done!");
                return;
            }
            _ => {
                info!("Waiting...")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run_app().await;
}
