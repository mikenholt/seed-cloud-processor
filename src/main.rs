#[macro_use]
extern crate slog;
extern crate sloggers;
extern crate clap;

mod commands;
pub mod utils;

use clap::{Arg, ArgMatches, App, SubCommand};
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use std::process;

fn main() {
    let matches = App::new("seed-cloud-processor")
        .version("0.1.0")
        .author("Mike Holt <mike.holt@appliedis.com>")
        .about("Configure and deploy Seed tasks to a distributed cloud system.")
        .long_about("Seed-Batch is a CLI that allows a user to configure and execute batch processing
task on remote distributed systems.  Seed-Batch talk to AWS Batch, Azure Batch,
and Hashicorp Nomad clusters by submitting Seed based tasks and letting those
services schedule and execute the work.

Seed-Batch has several dependencies that are required for it to work correctly:
	1) Proper configuration and access to an AWS Batch, Azure Batch, or Hashicorp Nomad cluster.
	2) A valid Seed image that is hosted in a repo accessible to the cluster.
	3) A set of inputs also accessible to the cluster.
	
Seed-Batch will help you in configuring and validating these three items
before submitting the tasks for execution.")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("enable verbose logging"))
        .subcommand(
            SubCommand::with_name("init")
                .about("help for seed-cloud-processor")
                .arg(Arg::with_name("seed-image")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("docker container uri with tag")
                )
        )
        // .arg(Arg::with_name("SEED IMAGE")
        //          .required(true)
        //          .takes_value(true)
        //          .index(1)
        //          .help("url to download"))
        .get_matches();
    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<(), String> {
    // Configure the logger - instantiate it and assign the verbosity level
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => sloggers::types::Severity::Info,
        1 => sloggers::types::Severity::Debug,
        2 | _ => sloggers::types::Severity::Trace,
    };

    let mut builder = TerminalLoggerBuilder::new();
    builder.level(min_log_level);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();

    // Announce the log level if set
    if matches.occurrences_of("verbose") == 1 {
        debug!(logger, "Debug level logging enabled");
    } else if matches.occurrences_of("verbose") > 1 {
        trace!(logger, "Trace level logging enabled");
    }
    // Map subcommands to their respective function
    match matches.subcommand() {
        ("init", Some(m)) => commands::init::run_init(m, &logger),
        _ => Ok(()),
    }
}