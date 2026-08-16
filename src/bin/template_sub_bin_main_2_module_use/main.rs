/*
cargo build --release --bin sub_bin_main_2_module_use
target\release\sub_bin_main_2_module_use.exe
*/

use std::env;
use clap::Parser;
use tracing::{info};

mod add_numbers;
mod print_custom_message;

use add_numbers::add_numbers;
use print_custom_message::print_custom_message;

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

    let sum = add_numbers(10, 20);
    print_custom_message(&format!("Sum result = {}", sum));
    
    info!("END");
    Ok(())
}