use anyhow::Result;
use chrono::Local;
use csv::Writer;
use device_query::{DeviceQuery, DeviceState, MouseState};
use eframe::egui;
use serde::Serialize;
use std::{
    fs::OpenOptions,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
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
    is_monitoring: Arc<AtomicBool>,
    csv_writer: Arc<Mutex<Writer<std::fs::File>>>,
    events_recorded: Arc<AtomicBool>,
    status_text: Arc<Mutex<String>>,
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
            is_monitoring: Arc::new(AtomicBool::new(false)),
            csv_writer: Arc::new(Mutex::new(csv_writer)),
            events_recorded: Arc::new(AtomicBool::new(false)),
            status_text: Arc::new(Mutex::new(String::from("Ready to start monitoring"))),
        })
    }

    fn start_monitoring(&self) {
        if self.is_monitoring.load(Ordering::SeqCst) {
            if let Ok(mut status) = self.status_text.lock() {
                *status = "Already monitoring!".to_string();
            }
            return;
        }

        if let Ok(mut status) = self.status_text.lock() {
            *status = "Started monitoring...".to_string();
        }
        
        self.is_monitoring.store(true, Ordering::SeqCst);
        let is_monitoring = Arc::clone(&self.is_monitoring);
        let csv_writer = Arc::clone(&self.csv_writer);
        let events_recorded = Arc::clone(&self.events_recorded);
        let status_text = Arc::clone(&self.status_text);
        let device_state = DeviceState::new();

        thread::spawn(move || {
            let mut last_keys = Vec::new();
            let mut last_mouse_pos = (0, 0);

            while is_monitoring.load(Ordering::SeqCst) {
                // Monitor keyboard
                let keys = device_state.get_keys();
                if keys != last_keys {
                    let keys_str = format!("{:?}", keys);
                    if let Ok(mut writer) = csv_writer.lock() {
                        let mouse: MouseState = device_state.get_mouse();
                        let record = ActivityRecord {
                            timestamp: Local::now().to_rfc3339(),
                            event_type: "keyboard".to_string(),
                            details: keys_str.clone(),
                            mouse_x: mouse.coords.0,
                            mouse_y: mouse.coords.1,
                        };

                        if let Err(e) = writer.serialize(&record) {
                            if let Ok(mut status) = status_text.lock() {
                                *status = format!("Error: {}", e);
                            }
                        } else {
                            events_recorded.store(true, Ordering::SeqCst);
                            if let Ok(mut status) = status_text.lock() {
                                *status = format!("Keyboard: {}", keys_str);
                            }
                        }
                        writer.flush().unwrap_or_else(|e| eprintln!("Error flushing: {}", e));
                    }
                    last_keys = keys;
                }

                // Monitor mouse
                let mouse: MouseState = device_state.get_mouse();
                let current_pos = mouse.coords;
                if current_pos != last_mouse_pos {
                    if let Ok(mut writer) = csv_writer.lock() {
                        let record = ActivityRecord {
                            timestamp: Local::now().to_rfc3339(),
                            event_type: "mouse_move".to_string(),
                            details: format!("Moved to {:?}", current_pos),
                            mouse_x: current_pos.0,
                            mouse_y: current_pos.1,
                        };

                        if let Err(e) = writer.serialize(&record) {
                            if let Ok(mut status) = status_text.lock() {
                                *status = format!("Error: {}", e);
                            }
                        } else {
                            events_recorded.store(true, Ordering::SeqCst);
                            if let Ok(mut status) = status_text.lock() {
                                *status = format!("Mouse: ({}, {})", current_pos.0, current_pos.1);
                            }
                        }
                        writer.flush().unwrap_or_else(|e| eprintln!("Error flushing: {}", e));
                    }
                    last_mouse_pos = current_pos;
                }

                thread::sleep(Duration::from_millis(50));
            }
            
            if let Ok(mut status) = status_text.lock() {
                if events_recorded.load(Ordering::SeqCst) {
                    *status = "Monitoring stopped. Activities were recorded.".to_string();
                } else {
                    *status = "Monitoring stopped. No activities were recorded.".to_string();
                }
            }
        });
    }

    fn stop_monitoring(&self) {
        if !self.is_monitoring.load(Ordering::SeqCst) {
            if let Ok(mut status) = self.status_text.lock() {
                *status = "Monitoring is not running".to_string();
            }
            return;
        }
        if let Ok(mut status) = self.status_text.lock() {
            *status = "Stopping monitoring...".to_string();
        }
        self.is_monitoring.store(false, Ordering::SeqCst);
    }
}

struct MonitorApp {
    monitor: Arc<ActivityMonitor>,
}

impl MonitorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            monitor: Arc::new(ActivityMonitor::new().unwrap()),
        }
    }
}

impl eframe::App for MonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            if let Ok(status) = self.monitor.status_text.lock() {
                ui.label(&*status);
            }

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
