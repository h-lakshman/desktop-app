use anyhow::Result;
use chrono::Local;
use csv::Writer;
use device_query::{DeviceQuery, DeviceState, MouseState};
use eframe::egui;
use serde::Serialize;
use std::{
    fs::OpenOptions,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Serialize)]
struct ActivityRecord {
    timestamp: String,
    event_type: String,
    details: String,
    mouse_x: i32,
    mouse_y: i32,
}

struct ActivityMonitor {
    is_monitoring: AtomicBool,
    csv_writer: Writer<std::fs::File>,
    events_recorded: AtomicBool,
    status_text: String,
    device_state: DeviceState,
    last_keys: Vec<device_query::Keycode>,
    last_mouse_pos: (i32, i32),
}

impl ActivityMonitor {
    fn new() -> Result<Self> {
        println!("=== Desktop Activity Monitor ===");
        println!("Initializing...");

        // Test if we can get device state
        let test_device = DeviceState::new();
        let test_mouse = test_device.get_mouse();
        println!(
            "✓ Mouse detection working (current position: {:?})",
            test_mouse.coords
        );

        let test_keys = test_device.get_keys();
        println!(
            "✓ Keyboard detection working (current keys: {:?})",
            test_keys
        );

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("activity_log.csv")?;

        let csv_writer = Writer::from_writer(file);
        println!("✓ Created activity_log.csv file for storing activities");

        Ok(Self {
            is_monitoring: AtomicBool::new(false),
            csv_writer,
            events_recorded: AtomicBool::new(false),
            status_text: String::from("Ready to start monitoring"),
            device_state: DeviceState::new(),
            last_keys: Vec::new(),
            last_mouse_pos: (0, 0),
        })
    }

    fn start_monitoring(&mut self) {
        if self.is_monitoring.load(Ordering::SeqCst) {
            self.status_text = "Already monitoring!".to_string();
            return;
        }

        self.status_text = "Started monitoring...".to_string();
        self.is_monitoring.store(true, Ordering::SeqCst);
    }

    fn stop_monitoring(&mut self) {
        if !self.is_monitoring.load(Ordering::SeqCst) {
            self.status_text = "Monitoring is not running".to_string();
            return;
        }
        self.status_text = "Stopping monitoring...".to_string();
        self.is_monitoring.store(false, Ordering::SeqCst);

        if self.events_recorded.load(Ordering::SeqCst) {
            self.status_text = "Monitoring stopped. Activities were recorded.".to_string();
        } else {
            self.status_text = "Monitoring stopped. No activities were recorded.".to_string();
        }
    }

    fn update(&mut self) {
        if !self.is_monitoring.load(Ordering::SeqCst) {
            return;
        }

        // Monitor keyboard
        let keys = self.device_state.get_keys();
        if keys != self.last_keys {
            let keys_str = format!("{:?}", keys);
            let mouse: MouseState = self.device_state.get_mouse();
            let record = ActivityRecord {
                timestamp: Local::now().to_rfc3339(),
                event_type: "keyboard".to_string(),
                details: keys_str.clone(),
                mouse_x: mouse.coords.0,
                mouse_y: mouse.coords.1,
            };

            if let Err(e) = self.csv_writer.serialize(&record) {
                self.status_text = format!("Error: {}", e);
            } else {
                self.events_recorded.store(true, Ordering::SeqCst);
                self.status_text = format!("Keyboard: {}", keys_str);
            }
            self.csv_writer
                .flush()
                .unwrap_or_else(|e| eprintln!("Error flushing: {}", e));
            self.last_keys = keys;
        }

        // Monitor mouse
        let mouse: MouseState = self.device_state.get_mouse();
        let current_pos = mouse.coords;
        if current_pos != self.last_mouse_pos {
            let record = ActivityRecord {
                timestamp: Local::now().to_rfc3339(),
                event_type: "mouse_move".to_string(),
                details: format!("Moved to {:?}", current_pos),
                mouse_x: current_pos.0,
                mouse_y: current_pos.1,
            };

            if let Err(e) = self.csv_writer.serialize(&record) {
                self.status_text = format!("Error: {}", e);
            } else {
                self.events_recorded.store(true, Ordering::SeqCst);
                self.status_text = format!("Mouse: ({}, {})", current_pos.0, current_pos.1);
            }
            self.csv_writer
                .flush()
                .unwrap_or_else(|e| eprintln!("Error flushing: {}", e));
            self.last_mouse_pos = current_pos;
        }
    }
}

struct MonitorApp {
    monitor: ActivityMonitor,
}

impl MonitorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            monitor: ActivityMonitor::new().unwrap(),
        }
    }
}

impl eframe::App for MonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update monitor state
        self.monitor.update();

        // Request continuous updates when monitoring
        if self.monitor.is_monitoring.load(Ordering::SeqCst) {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Desktop Activity Monitor");
            ui.add_space(20.0);

            if ui.button("Start Monitoring").clicked() {
                self.monitor.start_monitoring();
            }

            if ui.button("Stop Monitoring").clicked() {
                self.monitor.stop_monitoring();
            }

            ui.add_space(20.0);
            ui.label(&self.monitor.status_text);

            ui.add_space(20.0);
            ui.label("All activities are being saved to 'activity_log.csv'");
        });
    }
}

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
