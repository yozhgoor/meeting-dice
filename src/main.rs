use anyhow::{Context, Result};
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
    /// This option will also print the last meeting chair, the last note taker and the
    /// hidden member(s).
    #[clap(long, short = 'l')]
    list: bool,
    /// Specify who was the last meeting chair.
    #[clap(long)]
    last_chair: Option<String>,
    /// Specify who was the last note taker.
    #[clap(long)]
    last_note_taker: Option<String>,
    /// Add member(s) to the team.
    #[clap(long, short = 'a')]
    add_members: Vec<String>,
    /// Remove temporarily member(s) of the team.
    #[clap(long, short = 'h', requires = "run")]
    hide_members: Vec<String>,
    /// Remove member(s) of the team.
    ///
    /// If a removed member was the last meeting chair or the last note taker, these values will be
    /// set to `None`.
    #[clap(long, short = 'r')]
    remove_members: Vec<String>,
    /// Specify if this meeting needs a note taker.
    ///
    /// Note that the last note taker will remain the same if it is set to false.
    #[clap(long, short = 'n')]
    note_taker: bool,
    /// Roll the dice.
    ///
    /// All the options given will be executed before the run.
    #[clap(long)]
    run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data = Data::get_or_create().context("cannot get or create data file")?;

    run(cli, data)?;

    Ok(())
}
