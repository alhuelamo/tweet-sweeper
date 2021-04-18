use std::process;

use egg_mode::error::Error;
use tweet_sweeper::{config, report::Report};

use env_logger;

fn main() {
    env_logger::init();

    let config = config::load_default_configuration().unwrap_or_else(|err| {
        eprintln!("Error reading configuration: {}", err);
        process::exit(1);
    });

    match tweet_sweeper::run(config) {
        Ok(report) => process_success(report),
        Err(egg_error) => process_error(egg_error),
    }

    log::info!("DONE!");
}

fn process_success(report: Report) {
    match report.get_number_of_removed_tweets() {
        0 => log::info!("No tweets removed!"),
        n_removed => log::info!("Successfully removed {} tweets!", n_removed),
    }
}

fn process_error(egg_error: Error) {
    log::error!("Something went wrong...");
    log::error!("{}", egg_error);
    process::exit(1);
}
