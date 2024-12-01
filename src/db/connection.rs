use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection(database_url: &str) -> Result<PgConnection, diesel::ConnectionError> {
    // Try connecting to postgres directly first
    let postgres_url = database_url.replace("/slicer_db", "/postgres");
    if let Ok(mut conn) = PgConnection::establish(&postgres_url) {
        // Create database if it doesn't exist
        diesel::sql_query("CREATE DATABASE slicer_db")
            .execute(&mut conn)
            .unwrap_or(0); // Ignore error if DB exists
    }

    // Now connect to slicer_db
    let mut conn = PgConnection::establish(database_url)?;
    
    // Run migrations
    conn.run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|e| panic!("Failed to run migrations: {}", e));

    Ok(conn)
}