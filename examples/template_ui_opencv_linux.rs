/*
cargo build --example template_ui_opencv_linux --release
cargo run --example template_ui_opencv_linux
*/

use std::env;
use clap::Parser;
use tracing::{info};
use tracing_subscriber::EnvFilter;
use opencv::{core, imgcodecs, imgproc, prelude::MatTraitConst, Result};
use chrono::Local;

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
    println!("Loading image from: {}", image_path.display());
    let image_path_str = image_path.to_str().expect("Invalid path string");
    let img = imgcodecs::imread(image_path_str, imgcodecs::IMREAD_COLOR)?;

    println!("Image loaded successfully");
    println!("Image width = {}", img.cols());
    println!("Image height = {}", img.rows());

    let mut test_img = core::Mat::new_rows_cols_with_default(
        500,
        800,
        core::CV_8UC3,
        core::Scalar::new(255.0, 255.0, 255.0, 0.0),
    )?;

    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    imgproc::put_text(
        &mut test_img,
        &format!("Datetime: {}", datetime_now),
        core::Point::new(100, 250),
        imgproc::FONT_HERSHEY_SIMPLEX,
        0.6,
        core::Scalar::new(0.0, 0.0, 0.0, 0.0),
        1,
        imgproc::LINE_AA,
        false,
    )?;

    let output_path = current_dir.join("static/test-opencv.jpg");
    let output_path_str = output_path.to_str().expect("Invalid path string");
    imgcodecs::imwrite(output_path_str, &test_img, &core::Vector::new())?;

    println!("OpenCV test image saved to: {}", output_path.display());

    info!("END");
    Ok(())
}