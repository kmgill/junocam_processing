use junocam::print;
mod subs;
use subs::runnable::RunnableSubcommand;
use subs::*;


extern crate wild;
use clap::{
    Parser, 
    Subcommand
};

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
    Weights(weights::Weights)
}




fn main() {
    let args = Cli::parse_from(wild::args());

    if args.verbose {
        print::set_verbose(true);
    }

    match args.command {
        Juno::TripletCount(args) => {
            args.run();
        },
        Juno::Infill(args) => {
            args.run();
        },
        Juno::Decompand(args) => {
            args.run();
        },
        Juno::Calibrate(args) => {
            args.run();
        },
        Juno::Hpc(args) => {
            args.run();
        },
        Juno::Weights(args) => {
            args.run();
        }

    };
}