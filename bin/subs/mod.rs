pub mod runnable;

/// Creates the default progress bar as a static ref in global space. Uses lazy_static
#[macro_export]
macro_rules! pb_create {
    () => {
        use indicatif::{ProgressBar, ProgressStyle};
        use lazy_static::lazy_static;

        lazy_static! {
            static ref PB: ProgressBar = {
                let pb = ProgressBar::new(1);
                pb.set_style(
                    ProgressStyle::with_template(
                        "{prefix:.bold.dim} [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}",
                    )
                    .unwrap()
                    .progress_chars("#>-"),
                );
                pb
            };
        }
    };
}

/// Creates the default spinner as a static ref in global space. Uses lazy_static
#[macro_export]
macro_rules! pb_create_spinner {
    () => {
        use indicatif::{ProgressBar, ProgressStyle};
        use lazy_static::lazy_static;
        use std::time::Duration;

        lazy_static! {
            static ref PB: ProgressBar = {
                // This is directly ripped from the indicatif examples.
                let pb = ProgressBar::new_spinner();
                pb.enable_steady_tick(Duration::from_millis(80));
                pb.set_style(
                    ProgressStyle::with_template("{spinner:.blue} {msg}")
                        .unwrap()
                        .tick_strings(&[
                            "▹▹▹▹▹▹▹▹▹▹",
                            "▸▹▹▹▹▹▹▹▹▹",
                            "▹▸▹▹▹▹▹▹▹▹",
                            "▹▹▸▹▹▹▹▹▹▹",
                            "▹▹▹▸▹▹▹▹▹▹",
                            "▹▹▹▹▸▹▹▹▹▹",
                            "▹▹▹▹▹▸▹▹▹▹",
                            "▹▹▹▹▹▹▸▹▹▹",
                            "▹▹▹▹▹▹▹▸▹▹",
                            "▹▹▹▹▹▹▹▹▸▹",
                            "▹▹▹▹▹▹▹▹▹▸",
                            "▪▪▪▪▪▪▪▪▪▪",
                        ]),
                );
                pb.set_message("Processing...");
                pb
            };
        }
    };
}

/// Injects the progress bar into the mru::print component to route verbose printing
#[macro_export]
macro_rules! pb_set_print {
    () => {
        use stump;
        stump::set_print(|s| {
            PB.println(s);
        });
    };
}

/// Sets the item length of the progress bar
#[macro_export]
macro_rules! pb_set_length {
    ($x: expr) => {
        PB.set_length($x as u64);
    };
}

/// Sets the item length of the progress bar
#[macro_export]
macro_rules! pb_zero {
    () => {
        PB.set_position(0);
    };
}

/// Combined method as proxy to both pb_set_print! and pb_set_length
#[macro_export]
macro_rules! pb_set_print_and_length {
    ($x: expr) => {
        pb_set_print!();
        pb_set_length!($x);
    };
}

/// Increment the progress bar by a specified amount
#[macro_export]
macro_rules! pb_inc_by {
    ($x: expr) => {
        PB.inc($x);
    };
}

/// Increment the progress bar by one
#[macro_export]
macro_rules! pb_inc {
    () => {
        PB.inc(1)
    };
}

/// Print to the console via the progress bar's println method.
#[macro_export]
macro_rules! pb_println {
    ($x: expr) => {
        PB.println($x);
    };
}

#[macro_export]
macro_rules! pb_set_prefix {
    ($prefix: expr) => {
        PB.set_prefix($prefix);
    };
}

/// Finishes the spinner with a 'Done' message
#[macro_export]
macro_rules! pb_done {
    () => {
        PB.finish_with_message("Done")
    };
}

/// Finishes the spinner with a 'Done with error' message
#[macro_export]
macro_rules! pb_done_with_error {
    () => {
        PB.finish_with_message("Done with error");
    };
}


pub mod calibrate;
pub mod centerofmass;
pub mod decompand;
pub mod hpc;
pub mod infill;
pub mod process;
pub mod tripcount;
pub mod weights;
