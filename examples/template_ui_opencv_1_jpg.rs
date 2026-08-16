/*
cargo build --example template_ui_opencv_1_jpg --release
cargo run --example template_ui_opencv_1_jpg
*/

use std::env;
use clap::Parser;
use tracing::{info};
use tracing_subscriber::EnvFilter;
use opencv::{core, highgui, imgcodecs, imgproc, Result};
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

    //// Required string
    // #[arg(long = "string-require")]
    // string_require: String,
}

fn show_jpg<P: AsRef<Path>>(image_path: P) -> Result<()> {
    let path_ref = image_path.as_ref();
    println!("Loading image from: {}", path_ref.display());
    let image_path_str = path_ref.to_str().expect("Invalid path string");
    let img = imgcodecs::imread(image_path_str, imgcodecs::IMREAD_COLOR)?;
    let width = img.cols() as f64;
    let height = img.rows() as f64;
    println!("Current image size: {} x {} pixels", width as i32, height as i32);
    let max_size = 500.0;
    let scale = if width > height {
        max_size / width
    } else {
        max_size / height
    };
    let new_width = (width * scale) as i32;
    let new_height = (height * scale) as i32;
    let mut resized_img = core::Mat::default();
    let dsize = core::Size::new(new_width, new_height);
    imgproc::resize(
        &img,
        &mut resized_img,
        dsize,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;
    highgui::imshow("Test OpenCV Window (Balanced)", &resized_img)?;
    highgui::wait_key(0)?;
    Ok(())
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

    let image_path = current_dir.join("static/car/car-001.jpg");
    show_jpg(&image_path)?;

    let image_path = current_dir.join("static/car/car-002.jpg");
    show_jpg(&image_path)?;
    
    info!("END");
    Ok(())
}