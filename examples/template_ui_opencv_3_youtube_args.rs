/*
cd C:\Users\papop\projects\rust-standard
cargo build --example template_ui_opencv_1_youtube_args --release
cargo run --example template_ui_opencv_1_youtube_args -- --url="https://www.youtube.com/watch?v=xnaOzlC2RXg"
cargo run --example template_ui_opencv_1_youtube_args -- --url="https://www.youtube.com/watch?v=d4JnshyKOOQ"

Live
cargo run --example template_ui_opencv_1_youtube_args -- --url="https://www.youtube.com/watch?v=sTF-6_xinUU"
cargo run --example template_ui_opencv_1_youtube_args -- --url="https://www.youtube.com/watch?v=EO_1LWqsCNE"
cargo run --example template_ui_opencv_1_youtube_args -- --url="https://www.youtube.com/watch?v=3nyPER2kzqk"
cargo run --example template_ui_opencv_1_youtube_args -- --url="https://www.youtube.com/watch?v=OElvQUElCmU"

Release
target\release\examples\template_ui_opencv_1_youtube_args.exe --url="https://www.youtube.com/watch?v=xnaOzlC2RXg"
target\release\examples\template_ui_opencv_1_youtube_args.exe --url="https://www.youtube.com/watch?v=sTF-6_xinUU"
target\release\examples\template_ui_opencv_1_youtube_args.exe --url="https://www.youtube.com/watch?v=EO_1LWqsCNE"
target\release\examples\template_ui_opencv_1_youtube_args.exe --url="https://www.youtube.com/watch?v=3nyPER2kzqk"
target\release\examples\template_ui_opencv_1_youtube_args.exe --url="https://www.youtube.com/watch?v=OElvQUElCmU"
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

    /// YouTube URL to play
    #[arg(short, long)]
    url: String,
}

fn show_youtube<P: AsRef<Path>>(video_path: P) -> Result<()> {
    let path_ref = video_path.as_ref();
    let youtube_url = path_ref.to_str().expect("Invalid path string");
    println!("Resolving stream URL via yt-dlp for: {}", youtube_url);

    // เรียก yt-dlp เพื่อดึง direct stream URL (ไม่ดาวน์โหลดไฟล์ลงเครื่อง)
    let output: std::process::Output = std::process::Command::new("yt-dlp")
        .args([
            "-f", "best[ext=mp4]/best",
            "-g", // get URL only
            youtube_url,
        ])
        .output()
        .expect("Failed to execute yt-dlp. Is it installed and in PATH?");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: yt-dlp failed: {}", stderr);
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stream_url = stdout.lines().next().unwrap_or("").trim().to_string();

    if stream_url.is_empty() {
        println!("Error: yt-dlp returned empty stream URL.");
        return Ok(());
    }

    println!("Loading video from resolved stream URL...");
    let mut cap = videoio::VideoCapture::from_file(&stream_url, videoio::CAP_FFMPEG)?;
    if !cap.is_opened()? {
        println!("Error: Could not open video stream.");
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

    println!("Playing video... Press 'q' or 'Q' to exit.");
    let mut frame = core::Mat::default();
    let mut resized_frame = core::Mat::default();
    let dsize = core::Size::new(new_width, new_height);

    loop {
        let success = cap.read(&mut frame)?;
        if !success || frame.empty() {
            println!("Stream ended or read failed.");
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
    println!("url = {}", args.url);

    show_youtube(&args.url)?;
    
    info!("END");
    Ok(())
}