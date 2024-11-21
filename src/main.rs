use clap::Parser;
use std::process::ExitCode;
use crate::core::merger::{Merger, MergerConfig};
use crate::core::utils::print_error;
use crate::types::args::{Args, Subcommands};

mod core;
mod types;

fn main() -> ExitCode {
    let args = Args::parse();

    match args.command {
        Subcommands::Update { path, languages, no_sort } => {
            let merger = Merger::new(MergerConfig {
                source_path: path,
                target_languages: languages,
                no_sort
            });

            let mut merger = match merger {
                Ok(merger) => merger,
                Err(err) => {
                    print_error(&err);
                    return ExitCode::FAILURE;
                }
            };

            if let Err(err) = merger.merge() {
                print_error(&err);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}