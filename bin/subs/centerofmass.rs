use crate::subs::runnable::RunnableSubcommand;

use junocam::path;
use sciimg::prelude::*;
use sciimg::util;

use junocam::vprintln;

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Center of mass centering", long_about = None)]
pub struct CenterOfMass {
    #[clap(
        long,
        short,
        help = "Input images",
        required = true,
        multiple_values = true
    )]
    inputs: Vec<String>,

    #[clap(long, short, help = "Object detection threshold")]
    threshold: Option<f32>,
}

impl RunnableSubcommand for CenterOfMass {
    fn run(&self) {
        let threshold = self.threshold.unwrap_or(100.0);

        for file_path in &self.inputs {
            if !path::file_exists(file_path) {
                eprintln!("ERROR: Input file not found: {}", file_path);
                process::exit(1);
            }
            vprintln!("Loading image file from {}", file_path);

            let mut img = RgbImage::open16(file_path).unwrap();

            let offset = img.calc_center_of_mass_offset(threshold, 0);
            img.shift(offset.h, offset.v);

            let output_filename = util::replace_image_extension(file_path, "-com.png");
            img.save(&output_filename);
        }
    }
}
