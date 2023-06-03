
extern crate dotenv;
use dotenv::dotenv;

mod app;
mod ui;

mod crossterm;
mod screens;
mod global_state;
mod global_key_handler;

use crate::crossterm::run;

use argh::FromArgs;
use std::{error::Error, time::Duration};

#[derive(Debug, FromArgs)]
#[argh(description = "Client for ratatui")]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics)?;
    Ok(())
}
