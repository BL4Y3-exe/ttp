mod app;
mod command;
mod core;
mod event;
mod storage;
mod theme;
mod ui;

use anyhow::Result;
use clap::Parser;

use crate::app::App;

#[derive(Debug, Parser)]
#[command(name = "ttp", version, about = "A terminal typing practice app")]
struct Cli;

fn main() -> Result<()> {
    let _cli = Cli::parse();
    let app = App::new();

    event::run(app)
}
