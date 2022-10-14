use crate::subs::runnable::RunnableSubcommand;

use junocam::{enums, path, rawimage};

use junocam::vprintln;

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Decompand raw image", long_about = None)]
pub struct Decompand {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}

impl RunnableSubcommand for Decompand {
    fn run(&self) {
        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        vprintln!("Loading image file from {}", self.input);
        let mut raw_image = rawimage::RawImage::new_from_image(&self.input).unwrap();

        vprintln!("Running decomanding process...");
        raw_image
            .appy_decomanding(enums::SampleBitMode::SQROOT)
            .expect("Error with decompanding");

        vprintln!("Saving image to {}", self.output);
        let assembled_final = raw_image.assemble();
        assembled_final.save_16bit(&self.output);
    }
}
