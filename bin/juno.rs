use junocam::print;
mod subs;
use subs::runnable::RunnableSubcommand;
use subs::*;
use anyhow::Result;
extern crate wild;
use colored::Colorize;
use clap::{Parser, Subcommand};

#[macro_use]
extern crate stump;


#[derive(Parser)]
#[clap(name = "juno")]
#[clap(about = "JunoCam raw processing", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Juno,

    #[clap(long, short, help = "Verbose output")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Juno {
    TripletCount(tripcount::TripletCount),
    Infill(infill::Infill),
    Decompand(decompand::Decompand),
    Calibrate(calibrate::Calibrate),
    Hpc(hpc::Hpc),
    Weights(weights::Weights),
    Process(process::Process),
    CenterOfMass(centerofmass::CenterOfMass),
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>  {
    let t1 = std::time::Instant::now();

    let args = Cli::parse_from(wild::args());

    stump::set_min_log_level(stump::LogEntryLevel::WARN);
    info!("Initialized logging"); // INFO, which means that this won't be seen
                                  // unless the user overrides via environment
                                  // variable.

    if args.verbose {
        print::set_verbose(true);
    }

    if let Err(why) = match args.command {
        Juno::TripletCount(args) => {
            args.run().await
        }
        Juno::Infill(args) => {
            args.run().await
        }
        Juno::Decompand(args) => {
            args.run().await
        }
        Juno::Calibrate(args) => {
            args.run().await
        }
        Juno::Hpc(args) => {
            args.run().await
        }
        Juno::Weights(args) => {
            args.run().await
        }
        Juno::Process(args) => {
            args.run().await
        }
        Juno::CenterOfMass(args) => {
            args.run().await
        }
    } {
        error!("{}", "Unhandled program error:".red());
        error!("{}", why);
    };

    info!("Runtime: {}s", t1.elapsed().as_secs_f64());
    Ok(())
}
