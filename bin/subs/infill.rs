use crate::subs::runnable::RunnableSubcommand;

use junocam::{path, rawimage};

use junocam::vprintln;

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Infill Correction", long_about = None)]
pub struct Infill {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}

impl RunnableSubcommand for Infill {
    fn run(&self) {
        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        vprintln!("Loading image file from {}", self.input);
        let mut raw_image = rawimage::RawImage::new_from_image(&self.input).unwrap();

        vprintln!("Running infill process...");
        raw_image
            .apply_infill_correction()
            .expect("Error with infill correction");

        vprintln!("Saving image to {}", self.output);
        let assembled_final = raw_image.assemble();
        assembled_final.save_16bit(&self.output);
    }
}
