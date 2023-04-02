use crate::subs::runnable::RunnableSubcommand;

use junocam::rawimage;

use junocam::vprintln;
use sciimg::path;
use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Infill Correction", long_about = None)]
pub struct Weights {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,

    #[clap(long, short, help = "Red weight")]
    red: Option<f32>,

    #[clap(long, short, help = "Green weight")]
    green: Option<f32>,

    #[clap(long, short, help = "Blue weight")]
    blue: Option<f32>,
}

impl RunnableSubcommand for Weights {
    fn run(&self) {
        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        vprintln!("Loading image file from {}", self.input);
        let mut raw_image = rawimage::RawImage::new_from_image(&self.input).unwrap();

        let red_weight = self.red.unwrap_or(1.0);
        let green_weight = self.green.unwrap_or(1.0);
        let blue_weight = self.blue.unwrap_or(1.0);

        vprintln!("Applying weights...");
        raw_image
            .apply_weights(red_weight, green_weight, blue_weight)
            .expect("Error applying channel weight values");

        vprintln!("Saving image to {}", self.output);
        let assembled_final = raw_image.assemble();
        assembled_final.save_16bit(&self.output);
    }
}
