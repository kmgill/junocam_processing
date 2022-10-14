use crate::subs::runnable::RunnableSubcommand;

use junocam::{path, rawimage};

use junocam::vprintln;

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Hot Pixel Correction", long_about = None)]
pub struct Hpc {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,

    #[clap(long, short = 't', help = "HPC threshold")]
    threshold: Option<f32>,

    #[clap(long, short = 'w', help = "HPC window size")]
    window: Option<i32>,
}

impl RunnableSubcommand for Hpc {
    fn run(&self) {
        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        let window = match self.window {
            Some(w) => w,
            None => 5,
        };

        let threshold = match self.threshold {
            Some(t) => t,
            None => 2.0,
        };

        vprintln!("Loading image file from {}", self.input);
        let mut raw_image = rawimage::RawImage::new_from_image(&self.input).unwrap();

        vprintln!("Running Hot Pixel Correction process...");
        raw_image
            .apply_hot_pixel_correction(window, threshold)
            .expect("Error wih hot pixel correction");

        vprintln!("Saving image to {}", self.output);
        let assembled_final = raw_image.assemble();
        assembled_final.save_16bit(&self.output);
    }
}
