use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Subcommands {
    Update {
        #[arg()]
        /// Path to the i18n export .json file.
        path: String,

        #[arg()]
        /// List of languages to be derived from i18n export .json file.
        languages: Vec<String>,

        #[arg(long)]
        /// Do not sort the source .json file.
        no_sort: bool
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Subcommands
}