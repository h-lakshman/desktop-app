use crate::monitor::ActivityMonitor;
use eframe::egui;

pub struct MonitorApp {
    monitor: ActivityMonitor,
}

impl MonitorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            monitor: ActivityMonitor::new().unwrap(),
        }
    }
}

impl eframe::App for MonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update monitor state
        self.monitor.update();

        if self
            .monitor
            .is_monitoring
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Desktop Activity Monitor");
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Task Name: ");
                if !self
                    .monitor
                    .is_monitoring
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    ui.text_edit_singleline(&mut self.monitor.task_name);
                } else {
                    ui.label(&self.monitor.task_name);
                }
            });

            ui.add_space(10.0);

            if !self.monitor.task_name.trim().is_empty() {
                if ui.button("Start Monitoring").clicked() {
                    self.monitor.start_monitoring();
                }
            } else {
                ui.add_enabled(false, egui::Button::new("Start Monitoring"));
            }

            if ui.button("Stop Monitoring").clicked() {
                self.monitor.stop_monitoring();
            }

            ui.add_space(20.0);
            ui.label(&self.monitor.status_text);

            ui.add_space(20.0);
            ui.label("Sessions are saved in: monitoring_sessions.csv");
            ui.label("Latest detailed events are in: latest_session_details.csv");
        });
    }
}
