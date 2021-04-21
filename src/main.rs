use std::process;

use tweet_sweeper::config;

use env_logger;

fn main() {
    env_logger::init();

    let config = config::load_default_configuration().unwrap_or_else(|err| {
        eprintln!("Error reading configuration: {}", err);
        process::exit(1);
    });

    tweet_sweeper::run(config);

    log::info!("DONE!");
}
