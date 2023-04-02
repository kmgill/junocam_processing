use crate::subs::runnable::RunnableSubcommand;

use junocam::rawimage;

use junocam::vprintln;
use sciimg::path;
use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Calibration (dark, flat)", long_about = None)]
pub struct Calibrate {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}

impl RunnableSubcommand for Calibrate {
    fn run(&self) {
        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        vprintln!("Loading image file from {}", self.input);
        let mut raw_image = rawimage::RawImage::new_from_image(&self.input).unwrap();

        vprintln!("Running calibration process...");
        raw_image
            .apply_darknoise()
            .expect("Error with dark/flat field correction");

        vprintln!("Saving image to {}", self.output);
        let assembled_final = raw_image.assemble();
        assembled_final.save_16bit(&self.output);
    }
}
