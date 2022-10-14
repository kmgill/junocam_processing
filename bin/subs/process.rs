use crate::subs::runnable::RunnableSubcommand;

use junocam::{
    config, path,
    process::{process_image, ProcessOptions, SupportedLens},
    vprintln,
};

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Process RGB JunoCam image", long_about = None)]
pub struct Process {
    #[clap(long, short, help = "Input image")]
    input: String,

    #[clap(long, short, help = "Input metadata json")]
    metadata: String,

    #[clap(long, short, help = "Output image")]
    output: String,

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
}

impl RunnableSubcommand for Process {
    fn run(&self) {
        let juno_config = config::load_configuration().expect("Failed to load config file");

        if !path::file_exists(&self.input) {
            eprintln!("ERROR: Input file not found!");
            process::exit(1);
        }

        let red_weight = match self.red_weight {
            Some(r) => r,
            None => juno_config.defaults.red_weight,
        };

        let green_weight = match self.green_weight {
            Some(g) => g,
            None => juno_config.defaults.green_weight,
        };

        let blue_weight = match self.blue_weight {
            Some(b) => b,
            None => juno_config.defaults.blue_weight,
        };

        let output_width = match self.width {
            Some(w) => w,
            None => 1024,
        };
        vprintln!("Output image width: {}", output_width);

        let output_height = match self.height {
            Some(h) => h,
            None => 1024,
        };
        vprintln!("Output image height: {}", output_height);

        let camera_lens = match &self.lens {
            Some(l) => {
                if let Some(lens) = SupportedLens::from(&l.as_str()) {
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

        match process_image(&ProcessOptions {
            input: self.input.clone(),
            metadata: self.metadata.clone(),
            output: Some(self.output.clone()),
            red_weight: red_weight,
            green_weight: green_weight,
            blue_weight: blue_weight,
            predicted: self.predicted,
            width: output_width,
            height: output_height,
            fov: fov,
            pitch: pitch,
            yaw: yaw,
            roll: roll,
            lens: camera_lens,
        }) {
            Ok(_) => {
                vprintln!("Done")
            }
            Err(why) => {
                eprintln!("Error processing image: {}", why)
            }
        }
    }
}
