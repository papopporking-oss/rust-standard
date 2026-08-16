/*
cargo run --example sea_orm_postgres_3_query
cargo build --release --bin sea_orm_postgres_3_query
target\release\sea_orm_postgres_3_query.exe
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

async fn create_table(db: &sea_orm::DatabaseConnection,) -> Result<(), Box<dyn std::error::Error>> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS public.test_table_message (
            id bigserial NOT NULL,
            create_date timestamp DEFAULT now() NOT NULL,
            update_date timestamp DEFAULT now() NOT NULL,
            message varchar NOT NULL,
            "number" int8 NOT NULL,
            CONSTRAINT test_table_message_pk PRIMARY KEY (id)
        );
    "#;
    db.execute_unprepared(sql).await?;
    info!("Table public.test_table_message created successfully");
    Ok(())
}

async fn insert_table(db: &sea_orm::DatabaseConnection, message: &str, number: i64,) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
            INSERT INTO public.test_table_message
                (message, "number")
            VALUES
                ($1, $2);
        "#,
        [message.into(), number.into()],
    );
    db.execute_raw(stmt).await?;
    info!("Insert table successfully");
    Ok(())
}

async fn update_table(db: &sea_orm::DatabaseConnection, id: i64, message: &str, number: i64,) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
            UPDATE public.test_table_message
            SET
                message = $1,
                "number" = $2,
                update_date = now()
            WHERE id = $3;
        "#,
        [message.into(), number.into(), id.into()],
    );
    db.execute_raw(stmt).await?;
    info!("Update table successfully");
    Ok(())
}

async fn select_table(db: &sea_orm::DatabaseConnection,) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = Statement::from_string(
        db.get_database_backend(),
        r#"
            SELECT
                id,
                create_date,
                update_date,
                message,
                "number"
            FROM public.test_table_message
            ORDER BY id;
        "#
        .to_string(),
    );
    let rows = db.query_all_raw(stmt).await?;
    for row in rows {
        let id: i64 = row.try_get("", "id")?;
        let message: String = row.try_get("", "message")?;
        let number: i64 = row.try_get("", "number")?;

        info!(
            "id={}, message={}, number={}",
            id,
            message,
            number
        );
    }
    Ok(())
}

async fn select_table_by_id(db: &sea_orm::DatabaseConnection, id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
            SELECT
                id,
                create_date,
                update_date,
                message,
                "number"
            FROM public.test_table_message
            WHERE id = $1;
        "#,
        [id.into()],
    );
    let row = db.query_one_raw(stmt).await?;
    if let Some(row) = row {
        let id: i64 = row.try_get("", "id")?;
        let message: String = row.try_get("", "message")?;
        let number: i64 = row.try_get("", "number")?;

        info!(
            "id={}, message={}, number={}",
            id,
            message,
            number
        );
    } else {
        info!("Record not found: id={}", id);
    }
    Ok(())
}

async fn delete_table_by_id(db: &sea_orm::DatabaseConnection, id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
            DELETE FROM public.test_table_message
            WHERE id = $1;
        "#,
        [id.into()],
    );
    let result = db.execute_raw(stmt).await?;
    info!(
        "Delete table by id successfully: id={}, rows_affected={}",
        id,
        result.rows_affected()
    );
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
    let db_url = env::var("DB_POSTGRES_URI").expect("DB_POSTGRES_URI must be set in configs/.env");

    // Step 2: Connect to PostgreSQL using SeaORM
    let db = Database::connect(&db_url).await?;

    // Step 3: Create table
    create_table(&db).await?;

    // Step 4: Insert table
    insert_table(&db, "Hello World", 100).await?;

    // Step 5: Update table
    update_table(&db, 1, "Hello Rust", 200).await?;

    // Step 6: Select table
    select_table(&db).await?;

    // Step 7: Select table by id
    select_table_by_id(&db, 1).await?;

    // Step 8: Delete table by id
    delete_table_by_id(&db, 1).await?;

    info!("END");
    Ok(())
}


