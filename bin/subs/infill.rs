use crate::subs::runnable::RunnableSubcommand;
use anyhow::Result;
use junocam::rawimage;
use junocam::vprintln;
use sciimg::path;
use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Infill Correction", long_about = None)]
pub struct Infill {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}

#[async_trait::async_trait]
impl RunnableSubcommand for Infill {
    async fn run(&self) -> Result<()>{
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
        let mut assembled_final = raw_image.assemble();
        assembled_final.normalize_mut(0.0, 65535.0);
        assembled_final.save(&self.output)?;

        Ok(())
    }
}
