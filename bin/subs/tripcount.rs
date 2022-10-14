use crate::subs::runnable::RunnableSubcommand;

use junocam::{path, rawimage};

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Triplet Count", long_about = None)]
pub struct TripletCount {
    #[clap(long, short, help = "Input image", multiple_values(true))]
    input: String,
}

impl RunnableSubcommand for TripletCount {
    fn run(&self) {
        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        let raw_image = rawimage::RawImage::new_from_image(&self.input).unwrap();
        println!("Image File: {}", self.input);
        println!("Triplet Count: {}", raw_image.get_triplet_count());
    }
}
