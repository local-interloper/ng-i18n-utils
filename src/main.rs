use clap::Parser;
use std::process::ExitCode;
use crate::core::updater::{Updater, UpdaterConfig};
use crate::core::utils::print_error;
use crate::types::args::{Args, Subcommands};

mod core;
mod types;

fn main() -> ExitCode {
    let args = Args::parse();

    match args.command {
        Subcommands::Update { path, languages, no_sort } => {
            let updater = Updater::new(UpdaterConfig {
                source_path: path,
                target_languages: languages,
                no_sort
            });

            let mut updater = match updater {
                Ok(updater) => updater,
                Err(err) => {
                    print_error(&err);
                    return ExitCode::FAILURE;
                }
            };

            if let Err(err) = updater.update() {
                print_error(&err);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}