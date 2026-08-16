/*
cargo clean
cargo build --release --bin template_ui_opencv
target\release\template_ui_opencv.exe
*/

use std::env;
use clap::Parser;
use tracing::{info};
use opencv::{highgui, imgcodecs, Result};

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
    tracing_subscriber::fmt::init();
    let current_dir = env::current_dir().unwrap();
    info!("{}", current_dir.display());
    dotenvy::from_path(current_dir.join("configs/.env")).ok();
    let app_name = env::var("APP_NAME").unwrap_or_else(|_| "DefaultApp".to_string());
    info!("APP_NAME = {}", app_name);
    info!("START");
    let args = Args::parse();
    
    println!("verbose = {}", args.verbose);
    println!("dry_run = {}", args.dry_run);
    println!("string = {:?}", args.string);

    let image_path = current_dir.join("static/010-peugeot-308-sw.jpg");
    println!("Loading image from: {}", image_path.display());
    let image_path_str = image_path.to_str().expect("Invalid path string");
    let img = imgcodecs::imread(image_path_str, imgcodecs::IMREAD_COLOR)?;
    highgui::imshow("Test OpenCV Window", &img)?;
    highgui::wait_key(0)?;
    
    info!("END");
    Ok(())
}