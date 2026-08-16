/*
cd C:\Users\papop\projects\rust-standard
cargo build --example template_ui_opencv_1_mp4 --release
cargo run --example template_ui_opencv_1_mp4
*/

use std::env;
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;
use opencv::{core, highgui, imgproc, videoio, Result};
use opencv::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    /// Verbose mode
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Dry run mode
    #[arg(short = 'd', long = "dry-run")]
    dry_run: bool,

    /// String value
    #[arg(long)]
    string: Option<String>,
}

fn show_mp4<P: AsRef<Path>>(video_path: P) -> Result<()> {
    let path_ref = video_path.as_ref();
    println!("Loading video from: {}", path_ref.display());
    let video_path_str = path_ref.to_str().expect("Invalid path string");
    let mut cap = videoio::VideoCapture::from_file(video_path_str, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        println!("Error: Could not open video file.");
        return Ok(());
    }
    let orig_width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as f64;
    let orig_height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as f64;
    println!("Original video size: {} x {} pixels", orig_width as i32, orig_height as i32);
    let max_size = 500.0;
    let scale = if orig_width > orig_height {
        max_size / orig_width
    } else {
        max_size / orig_height
    };
    let new_width = (orig_width * scale) as i32;
    let new_height = (orig_height * scale) as i32;
    println!("Playing video in loop... Press 'q' or 'Q' to exit.");
    let mut frame = core::Mat::default();
    let mut resized_frame = core::Mat::default();
    let dsize = core::Size::new(new_width, new_height);
    loop {
        loop {
            let success = cap.read(&mut frame)?;
            if !success || frame.empty() {
                break;
            }
            imgproc::resize(
                &frame,
                &mut resized_frame,
                dsize,
                0.0,
                0.0,
                imgproc::INTER_LINEAR,
            )?;
            highgui::imshow("Test OpenCV Video Window (Balanced)", &resized_frame)?;
            let key = highgui::wait_key(30)?;
            if key == 'q' as i32 || key == 'Q' as i32 {
                println!("User exited video playback.");
                return Ok(());
            }
        }
        cap.set(videoio::CAP_PROP_POS_FRAMES, 0.0)?;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().unwrap();
    dotenvy::from_path(current_dir.join("configs/.env")).ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))).init();
    let app_name = env::var("APP_NAME").unwrap_or_else(|_| "".to_string());
    info!("APP_NAME = {}", app_name);
    info!("START");
    
    let args = Args::parse();
    println!("verbose = {}", args.verbose);
    println!("dry_run = {}", args.dry_run);
    println!("string = {:?}", args.string);

    let video_path = current_dir.join("static/car/cars.mp4");
    show_mp4(&video_path)?;
    
    info!("END");
    Ok(())
}