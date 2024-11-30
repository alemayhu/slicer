mod schema;
mod db;
mod utils;
mod ui;

use eframe::egui;
use ui::app::SlicerApp;
use utils::single_instance::SingleInstance;

fn ensure_single_instance() -> Result<SingleInstance, String> {
    SingleInstance::new("slicer")
}

fn main() -> eframe::Result<()> {
    let _instance = ensure_single_instance()
        .map_err(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 480.0])
            .with_min_inner_size([320.0, 480.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "SliceR",
        native_options,
        Box::new(|cc| Box::new(SlicerApp::new(cc)))
    )
}
