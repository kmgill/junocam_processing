use crate::subs::runnable::RunnableSubcommand;
use junocam::{
    config,
    process::{process_image, ProcessOptions, SupportedLens},
    vprintln,
};
use anyhow::Result;
use sciimg::path;
use sciimg::util;
use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Process RGB JunoCam image", long_about = None)]
pub struct Process {
    #[clap(long, short, help = "Input images", multiple_values = true)]
    inputs: Vec<String>,

    #[clap(long, short, help = "Input metadata json", multiple_values = true)]
    metadata: Vec<String>,

    // #[clap(long, short, help = "Output images", multiple_values = true)]
    // outputs: Vec<String>,
    #[clap(long, short = 'R', help = "Red weight")]
    red_weight: Option<f32>,

    #[clap(long, short = 'G', help = "Green weight")]
    green_weight: Option<f32>,

    #[clap(long, short = 'B', help = "Blue weight")]
    blue_weight: Option<f32>,

    #[clap(long, short, help = "Use predicted kernels")]
    predicted: bool,

    #[clap(long, short, help = "Output width")]
    width: Option<usize>,

    #[clap(long, short = 'H', help = "Output height")]
    height: Option<usize>,

    #[clap(long, short, help = "Fisheye camera field of view, in degrees")]
    fov: Option<f64>,

    #[clap(
        long,
        short = 'P',
        help = "Camera pitch, in degrees",
        allow_hyphen_values(true)
    )]
    pitch: Option<f64>,

    #[clap(
        long,
        short,
        help = "Camera yaw, in degrees",
        allow_hyphen_values(true)
    )]
    yaw: Option<f64>,

    #[clap(
        long,
        short = 'r',
        help = "Camera roll, in degrees",
        allow_hyphen_values(true)
    )]
    roll: Option<f64>,

    #[clap(long, short, help = "Camera lens (cylindrical, fisheye)")]
    lens: Option<String>,

    #[clap(long, short = 'F', help = "Fast, skip every other line/sample")]
    fast: bool,

    #[clap(long, short = 'd', help = "Perform decorrelated color stretch")]
    decorrelated_color_stretch: bool,
}

#[async_trait::async_trait]
impl RunnableSubcommand for Process {
    async fn run(&self) -> Result<()> {
        let juno_config = config::load_configuration().expect("Failed to load config file");

        let red_weight = self.red_weight.unwrap_or(juno_config.defaults.red_weight);
        let green_weight = self
            .green_weight
            .unwrap_or(juno_config.defaults.green_weight);
        let blue_weight = self.blue_weight.unwrap_or(juno_config.defaults.blue_weight);

        let output_width = self.width.unwrap_or(1024);
        vprintln!("Output image width: {}", output_width);

        let output_height = self.height.unwrap_or(1024);
        vprintln!("Output image height: {}", output_height);

        let camera_lens = match &self.lens {
            Some(l) => {
                if let Some(lens) = SupportedLens::from(l.as_str()) {
                    lens
                } else {
                    eprintln!("Error: Invalid camera lens requested: {}", l);
                    eprintln!("Use either 'cylidrical' or 'fisheye'");
                    process::exit(1);
                }
            }
            None => SupportedLens::from(&juno_config.defaults.camera_lens_projection)
                .expect("Invalid default camera lens projection"),
        };

        let fov = match self.fov {
            Some(f) => f,
            None => juno_config.defaults.fisheye_field_of_view,
        };
        vprintln!("Fisheye field of view: {}", fov);

        let pitch = match self.pitch {
            Some(p) => p.to_radians() * -1.0, // Make it positive up
            None => 0.0,
        };
        vprintln!("Fisheye camera pitch: {}", pitch.to_degrees());

        let yaw = match self.yaw {
            Some(y) => y.to_radians() * -1.0, // Make it positive right
            None => 0.0,
        };
        vprintln!("Fisheye camera yaw: {}", yaw.to_degrees());

        let roll = match self.roll {
            Some(r) => r.to_radians(),
            None => 0.0,
        };
        vprintln!("Fisheye camera roll: {}", roll.to_degrees());

        if self.inputs.len() != self.metadata.len() {
            eprintln!("Error: Inputs do not match outputs.");
            process::exit(1);
        }

        self.inputs
            .iter()
            .zip(self.metadata.iter())
            .for_each(|(file_path, metadata)| {
                vprintln!("Image: {} -- Metadata: {}", file_path, metadata);
                if !path::file_exists(file_path) {
                    eprintln!("ERROR: Input file not found: {}", file_path);
                    process::exit(1);
                }
                vprintln!("Loading image file from {}", file_path);

                let output_filename = util::replace_image_extension(file_path, "-processed.png");

                match process_image(&ProcessOptions {
                    input: file_path.to_string(),
                    metadata: metadata.to_string(),
                    output: Some(output_filename),
                    red_weight,
                    green_weight,
                    blue_weight,
                    predicted: self.predicted,
                    width: output_width,
                    height: output_height,
                    fov,
                    pitch,
                    yaw,
                    roll,
                    lens: camera_lens,
                    fast: self.fast,
                    decorrelated_color_stretch: self.decorrelated_color_stretch,
                }) {
                    Ok(_) => {
                        vprintln!("Done")
                    }
                    Err(why) => {
                        eprintln!("Error processing image: {}", why)
                    }
                }
            });

        Ok(())
    }
}
