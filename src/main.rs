use anyhow::Result;
use desk_monitor::MonitorApp;
use eframe::egui;

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 200.0])
            .with_title("Desktop Activity Monitor"),
        ..Default::default()
    };

    eframe::run_native(
        "Desktop Activity Monitor",
        options,
        Box::new(|cc| Box::new(MonitorApp::new(cc))),
    )
    .unwrap();

    Ok(())
}
