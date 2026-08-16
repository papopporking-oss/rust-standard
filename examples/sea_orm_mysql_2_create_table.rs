/*
cargo build --release --bin sea_orm_mysql_2_create_table
target\release\sea_orm_mysql_2_create_table.exe
*/

use std::env;
use clap::Parser;
use tracing::{info};
use tracing_subscriber::EnvFilter;
use sea_orm::{ConnectionTrait, Database, Statement};

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

async fn create_table(db: &sea_orm::DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS test_table_message (
            id BIGINT AUTO_INCREMENT NOT NULL,
            create_date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
            update_date DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
            message VARCHAR(255) NOT NULL,
            number BIGINT NOT NULL,
            PRIMARY KEY (id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
    "#;
    db.execute_unprepared(sql).await?;
    info!("Table test_table_message created successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().unwrap();
    dotenvy::from_path(current_dir.join("configs/.env")).ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))).init();
    let app_name = env::var("APP_NAME").unwrap_or_else(|_| "".to_string());
    info!("APP_NAME = {}", app_name);
    info!("START");
    let args = Args::parse();

    // Step 1: Get database connection URI from environment
    let db_url = env::var("DB_MYSQL_URI").expect("DB_MYSQL_URI must be set in configs/.env");

    // Step 2: Connect to MySQL using SeaORM
    let db = Database::connect(&db_url).await?;

    // Step 3: Create table
    create_table(&db).await?;
    
    info!("END");
    Ok(())
}