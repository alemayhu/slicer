mod schema;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::process::Command;

#[allow(dead_code)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::runs)]
struct Run {
    id: i32,
    start_time: chrono::NaiveDateTime,
    end_time: Option<chrono::NaiveDateTime>,
    status: String,
    description: Option<String>,
    parameters: Option<serde_json::Value>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::runs)]
struct NewRun {
    description: Option<String>,
    parameters: Option<serde_json::Value>,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

fn main() {
    println!("Welcome to SliceR!");
    let mut conn = establish_connection();
    let username = Command::new("whoami")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
        .trim()
        .to_string();

    let new_run = NewRun {
        description: Some(format!("Started by user: {}", username)),
        parameters: None,
    };
    let run: Run = diesel::insert_into(schema::runs::table)
        .values(&new_run)
        .get_result(&mut conn)
        .expect("Error saving new run");
    println!("Created new run with ID: {}", run.id);
    diesel::update(schema::runs::table.find(run.id))
        .set((
            schema::runs::status.eq("completed"),
            schema::runs::end_time.eq(diesel::dsl::now),
        ))
        .execute(&mut conn)
        .expect("Error updating run status");
}
