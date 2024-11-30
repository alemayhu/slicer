use eframe::egui;
use crate::db::{models::NewRun, connection::establish_connection, repository::RunRepository};
use crate::utils::system::get_current_username;

pub struct SlicerApp {
    run: crate::db::models::Run,
}

impl SlicerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut conn = establish_connection();
        let mut repo = RunRepository::new(&mut conn);
        let username = get_current_username();

        let new_run = NewRun {
            description: Some(format!("Started by user: {}", username)),
            parameters: None,
        };

        let run = repo.create(new_run)
            .expect("Error saving new run");

        Self { run }
    }
}

impl eframe::App for SlicerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_run_details(ctx);
    }
}

impl SlicerApp {
    fn render_run_details(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Run Details");
            ui.separator();
            
            self.render_run_info(ui);
        });
    }

    fn render_run_info(&self, ui: &mut egui::Ui) {
        ui.label(format!("🔑 ID: {}", self.run.id));
        ui.label(format!("⏱️ Start Time: {}", self.run.start_time.format("%Y-%m-%d %H:%M:%S")));
        ui.label(format!("⏲️ End Time: {}", self.format_end_time()));
        ui.label(format!("📋 Status: {}", self.run.status));
        ui.label(format!("📝 Description: {}", self.format_description()));
        ui.label(format!("⚙️ Parameters: {}", self.format_parameters()));
        ui.label(format!("📅 Created: {}", self.run.created_at.format("%Y-%m-%d %H:%M:%S")));
        ui.label(format!("🔄 Updated: {}", self.run.updated_at.format("%Y-%m-%d %H:%M:%S")));
    }

    fn format_end_time(&self) -> String {
        self.run.end_time.map_or(
            "Not finished".to_string(),
            |t| t.format("%Y-%m-%d %H:%M:%S").to_string()
        )
    }

    fn format_description(&self) -> String {
        self.run.description.clone()
            .unwrap_or_else(|| String::from("No description"))
    }

    fn format_parameters(&self) -> String {
        self.run.parameters.clone()
            .map_or("None".to_string(), |p| p.to_string())
    }
} 