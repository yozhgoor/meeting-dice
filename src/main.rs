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
    #[clap(long)]
    list: bool,
    /// Specify who was the last meeting chair.
    #[clap(long)]
    last_chair: Option<String>,
    /// Add a new member to the team.
    ///
    /// Cannot be used with `--add-members`.
    #[clap(long, conflicts_with = "add-members")]
    add_member: Option<String>,
    /// Add members to the team.
    ///
    /// Cannot be used with `--add-member`.
    #[clap(long, conflicts_with = "add-member")]
    add_members: Vec<String>,
    /// Remove temporarily a member of the team.
    ///
    /// Cannot be used with `--hide-members`.
    #[clap(long, conflicts_with = "hide-members")]
    hide_member: Option<String>,
    /// Remove temporarily members of the team.
    ///
    /// Cannot be used with `--hide-member`.
    #[clap(long, conflicts_with = "hide-member")]
    hide_members: Vec<String>,
    /// Remove a member of the team.
    ///
    /// Cannot be used with `--remove-member`
    #[clap(long, conflicts_with = "remove-members")]
    remove_member: Option<String>,
    /// Remove members of the team.
    ///
    /// Cannot be used with `--remove-members`.
    #[clap(long, conflicts_with = "remove-member")]
    remove_members: Vec<String>,
    /// Roll the dice.
    ///
    /// All options given will be executed before selecting the next meeting chair.
    #[clap(long)]
    run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data = Data::get_or_create()?;

    run(cli, data)?;

    Ok(())
}
