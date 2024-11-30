mod schema;
mod db;
mod utils;

use db::models::NewRun;
use db::connection::establish_connection;
use db::repository::RunRepository;
use utils::system::get_current_username;

fn main() {
    println!("Welcome to SliceR!");
    let mut conn = establish_connection();
    let mut repo = RunRepository::new(&mut conn);
    let username = get_current_username();

    let new_run = NewRun {
        description: Some(format!("Started by user: {}", username)),
        parameters: None,
    };

    let run = repo.create(new_run)
        .expect("Error saving new run");

    println!("\n📊 Run Details");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔑 ID          : {}", run.id);
    println!("⏱️  Start Time  : {}", run.start_time.format("%Y-%m-%d %H:%M:%S"));
    println!("⏲️  End Time    : {}", run.end_time.map_or("Not finished".to_string(), |t| t.format("%Y-%m-%d %H:%M:%S").to_string()));
    println!("📋 Status      : {}", run.status);
    println!("📝 Description : {}", run.description.unwrap_or_else(|| "No description".to_string()));
    println!("⚙️  Parameters  : {}", run.parameters.map_or("None".to_string(), |p| p.to_string()));
    println!("📅 Created     : {}", run.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("🔄 Updated     : {}", run.updated_at.format("%Y-%m-%d %H:%M:%S"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    repo.mark_completed(run.id)
        .expect("Error updating run status");
}
