use crate::subs::runnable::RunnableSubcommand;
use junocam::{enums, rawimage};
use anyhow::Result;
use junocam::vprintln;
use sciimg::path;
use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Decompand raw image", long_about = None)]
pub struct Decompand {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}

#[async_trait::async_trait]
impl RunnableSubcommand for Decompand {
    async fn run(&self) -> Result<()>{
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
        let mut assembled_final = raw_image.assemble();
        assembled_final.normalize_mut(0.0, 65535.0);
        assembled_final.save(&self.output)?;

        Ok(())
    }
}
