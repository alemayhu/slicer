mod schema;
mod db;
mod utils;

use diesel::prelude::*;
use db::models::{Run, NewRun};
use db::connection::establish_connection;
use utils::system::get_current_username;

fn main() {
    println!("Welcome to SliceR!");
    let mut conn = establish_connection();
    let username = get_current_username();

    let new_run = NewRun {
        description: Some(format!("Started by user: {}", username)),
        parameters: None,
    };

    let run: Run = diesel::insert_into(schema::runs::table)
        .values(&new_run)
        .get_result(&mut conn)
        .expect("Error saving new run");

    println!("\n📊 Run Details");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔑 ID          : {}", run.id);
    println!("⏱️  Start Time  : {}", run.start_time.format("%Y-%m-%d %H:%M:%S"));
    println!("⏲️  End Time    : {}", run.end_time.map_or("Not finished".to_string(), |t| t.format("%Y-%m-%d %H:%M:%S").to_string()));
    println!("📋 Status      : {}", run.status);
    println!("🔤 Description : {:?}", run.description);
    println!("🔧 Parameters  : {:?}", run.parameters);
    println!("🗓️ Created At : {}", run.created_at);
    println!("🔄 Updated At : {}", run.updated_at);

    diesel::update(schema::runs::table.find(run.id))
        .set((
            schema::runs::status.eq("completed"),
            schema::runs::end_time.eq(diesel::dsl::now),
        ))
        .execute(&mut conn)
        .expect("Error updating run status");
}
