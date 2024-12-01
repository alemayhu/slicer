use eframe::egui;
use crate::db::{models::{NewRun, Run}, connection::establish_connection, repository::RunRepository};
use crate::utils::system::get_current_username;

#[derive(Clone)]
pub struct SlicerApp {
    run: crate::db::models::Run,
    has_error: bool,
}

impl SlicerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::connect_to_db()
    }

    fn connect_to_db() -> Self {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost:5432/slicer_db".to_string());

        let conn_result = establish_connection(&db_url);
        match conn_result {
            Ok(mut conn) => {
                let mut repo = RunRepository::new(&mut conn);
                let username = get_current_username();
                let new_run = NewRun {
                    description: Some(format!("Started by user: {}", username)),
                    parameters: None,
                };
                match repo.create(new_run) {
                    Ok(run) => Self { run, has_error: false },
                    Err(e) => Self::default_run(&format!("Failed to create run: {}", e))
                }
            },
            Err(e) => Self::default_run(&format!("Database connection failed: {}", e))
        }
    }

    fn default_run(error_msg: &str) -> Self {
        Self {
            run: Run::default_with_error(error_msg),
            has_error: true
        }
    }
}

impl eframe::App for SlicerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_run_details(ctx);
    }
}

impl SlicerApp {
    fn render_run_details(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Run Details");
            ui.separator();
            
            self.render_run_info(ui);
        });
    }

    fn render_run_info(&mut self, ui: &mut egui::Ui) {
        if self.has_error {
            if ui.button("🔄 Reconnect to Database").clicked() {
                let new_app = Self::connect_to_db();
                ui.ctx().data_mut(|data| {
                    data.insert_temp(egui::Id::new(1), new_app.clone())
                });
                *self = new_app;
            }
            ui.separator();
        }

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

impl Run {
    pub fn default_with_error(error_msg: &str) -> Self {
        Run {
            id: 0,
            start_time: chrono::Utc::now().naive_utc(),
            end_time: None,
            status: "ERROR".to_string(),
            description: Some(error_msg.to_string()),
            parameters: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}