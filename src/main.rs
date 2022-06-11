use anyhow::Result;
use clap::Parser;

mod data;
mod run;

use crate::data::Data;
use crate::run::run;

/// Choose who is gonna be the next meeting chair.
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /// List members of the meeting.
    ///
    /// This option will also print the last meeting chair and the hidden member(s).
    #[clap(long, short = 'l')]
    list: bool,
    /// Specify who was the last meeting chair.
    #[clap(long)]
    last_chair: Option<String>,
    /// Add member(s) to the team.
    #[clap(long, short = 'a')]
    add_members: Vec<String>,
    /// Remove temporarily member(s) of the team.
    #[clap(long, short = 'h', requires = "input")]
    hide_members: Vec<String>,
    /// Remove member(s) of the team.
    #[clap(long, short = 'r')]
    remove_members: Vec<String>,
    /// Roll the dice.
    ///
    /// All options given will be executed before the run.
    #[clap(long)]
    run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data = Data::get_or_create()?;

    run(cli, data)?;

    Ok(())
}
