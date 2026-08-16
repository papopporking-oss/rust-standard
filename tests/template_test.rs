/*
cargo test
cargo test --test template_test
cargo test --test template_test -- --nocapture
*/

use std::env;
// use clap::Parser;
use tracing::{info};

use rust_standard::add_numbers;

#[test]
fn test_integration_flow() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let current_dir = env::current_dir().unwrap();
    info!("{}", current_dir.display());
    dotenvy::from_path(current_dir.join("configs/.env")).ok();
    let app_name = env::var("APP_NAME").unwrap_or_else(|_| "DefaultApp".to_string());
    info!("APP_NAME = {}", app_name);
    info!("START");

    let sum = add_numbers(100, 200);
    assert_eq!(sum, 300);

    info!("END");
    Ok(())
}