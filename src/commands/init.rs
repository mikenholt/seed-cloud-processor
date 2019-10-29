// mod utils;

use clap::{ArgMatches};
use crate::utils::docker;


pub fn run_init(args: &ArgMatches, logger: &slog::Logger) -> Result<(), String> {
    info!(logger, "run_init");
    let docker_image = args.value_of("seed-image").unwrap().to_string();
    docker::get_manifest(&docker_image, logger);
    Ok(())
}