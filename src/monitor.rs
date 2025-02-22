use crate::types::{Action, DetailedEvent, Session};
use anyhow::Result;
use chrono::Local;
use csv::Writer;
use device_query::{DeviceQuery, DeviceState, MouseState};
use std::{
    fs::{File, OpenOptions},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct ActivityMonitor {
    pub is_monitoring: AtomicBool,
    session_writer: Writer<File>,
    pub detailed_writer: Writer<File>,
    pub events_recorded: AtomicBool,
    pub status_text: String,
    device_state: DeviceState,
    last_keys: Vec<device_query::Keycode>,
    last_mouse_pos: (i32, i32),
    pub current_session: Session,
    pub task_name: String,
}

impl ActivityMonitor {
    pub fn new() -> Result<Self> {
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

        let session_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("monitoring_sessions.csv")?;

        let detailed_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("latest_session_details.csv")?;

        let mut session_writer = Writer::from_writer(session_file);
        let detailed_writer = Writer::from_writer(detailed_file);

        // Write headers for both files
        session_writer.write_record(&[
            "session_id",
            "task_name",
            "start_time",
            "end_time",
            "actions",
        ])?;
        session_writer.flush()?;

        println!("✓ Created monitoring_sessions.csv for storing sessions");
        println!("✓ Created latest_session_details.csv for detailed events");

        // Create a new session file for appending after headers are written
        let session_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("monitoring_sessions.csv")?;

        let session_writer = Writer::from_writer(session_file);

        Ok(Self {
            is_monitoring: AtomicBool::new(false),
            session_writer,
            detailed_writer,
            events_recorded: AtomicBool::new(false),
            status_text: String::from("Enter task name to start monitoring"),
            device_state: DeviceState::new(),
            last_keys: Vec::new(),
            last_mouse_pos: (0, 0),
            current_session: Session {
                session_id: Local::now().format("%Y%m%d_%H%M%S").to_string(),
                task_name: String::new(),
                start_time: Local::now().to_rfc3339(),
                end_time: None,
                actions: Vec::new(),
            },
            task_name: String::new(),
        })
    }

    pub fn start_monitoring(&mut self) {
        if self.is_monitoring.load(Ordering::SeqCst) {
            self.status_text = "Already monitoring!".to_string();
            return;
        }

        if self.task_name.trim().is_empty() {
            self.status_text = "Please enter a task name first".to_string();
            return;
        }

        // Start new session
        self.current_session = Session {
            session_id: Local::now().format("%Y%m%d_%H%M%S").to_string(),
            task_name: self.task_name.clone(),
            start_time: Local::now().to_rfc3339(),
            end_time: None,
            actions: Vec::new(),
        };

        // Clear the detailed log file by creating a new writer
        let detailed_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("latest_session_details.csv")
            .unwrap();
        self.detailed_writer = Writer::from_writer(detailed_file);

        self.status_text = format!("Started monitoring task: {}", self.task_name);
        self.is_monitoring.store(true, Ordering::SeqCst);
    }

    pub fn stop_monitoring(&mut self) {
        if !self.is_monitoring.load(Ordering::SeqCst) {
            self.status_text = "Monitoring is not running".to_string();
            return;
        }

        self.status_text = "Stopping monitoring...".to_string();
        self.is_monitoring.store(false, Ordering::SeqCst);

        // Save the current session
        self.current_session.end_time = Some(Local::now().to_rfc3339());

        // Write session to CSV with explicit fields to ensure order
        let record = vec![
            self.current_session.session_id.clone(),
            self.current_session.task_name.clone(),
            self.current_session.start_time.clone(),
            self.current_session.end_time.clone().unwrap_or_default(),
            self.current_session
                .actions
                .iter()
                .map(|action| action.to_csv_string())
                .collect::<Vec<_>>()
                .join(";"),
        ];

        if let Err(e) = self.session_writer.write_record(&record) {
            self.status_text = format!("Error saving session: {}", e);
        }
        if let Err(e) = self.session_writer.flush() {
            self.status_text = format!("Error flushing session data: {}", e);
        }

        if self.events_recorded.load(Ordering::SeqCst) {
            self.status_text = format!(
                "Monitoring stopped for task: {}. Activities were recorded.",
                self.task_name
            );
        } else {
            self.status_text = format!(
                "Monitoring stopped for task: {}. No activities were recorded.",
                self.task_name
            );
        }
    }

    pub fn update(&mut self) {
        if !self.is_monitoring.load(Ordering::SeqCst) {
            return;
        }

        // Monitor keyboard
        let keys = self.device_state.get_keys();
        if keys != self.last_keys {
            let timestamp = Local::now().to_rfc3339();
            let keys_str: Vec<String> = keys.iter().map(|k| format!("{:?}", k)).collect();
            let mouse: MouseState = self.device_state.get_mouse();

            // Add to current session
            let action = Action::KeyPress {
                timestamp: timestamp.clone(),
                keys: keys_str.clone(),
            };
            self.current_session.actions.push(action);

            // Add to detailed log
            let detailed_event = DetailedEvent {
                timestamp,
                task_name: self.task_name.clone(),
                event_type: "keyboard".to_string(),
                details: format!("{:?}", keys_str),
                mouse_x: mouse.coords.0,
                mouse_y: mouse.coords.1,
            };

            if let Err(e) = self.detailed_writer.serialize(&detailed_event) {
                self.status_text = format!("Error: {}", e);
            } else {
                self.events_recorded.store(true, Ordering::SeqCst);
                self.status_text = format!("Task: {} - Keyboard: {:?}", self.task_name, keys_str);
            }
            self.detailed_writer
                .flush()
                .unwrap_or_else(|e| eprintln!("Error flushing: {}", e));
            self.last_keys = keys;
        }

        // Monitor mouse
        let mouse: MouseState = self.device_state.get_mouse();
        let current_pos = mouse.coords;
        if current_pos != self.last_mouse_pos {
            let timestamp = Local::now().to_rfc3339();

            // Add to current session
            let action = Action::MouseMove {
                timestamp: timestamp.clone(),
                coords: current_pos,
            };
            self.current_session.actions.push(action);

            // Add to detailed log
            let detailed_event = DetailedEvent {
                timestamp,
                task_name: self.task_name.clone(),
                event_type: "mouse_move".to_string(),
                details: format!("Moved to {:?}", current_pos),
                mouse_x: current_pos.0,
                mouse_y: current_pos.1,
            };

            if let Err(e) = self.detailed_writer.serialize(&detailed_event) {
                self.status_text = format!("Error: {}", e);
            } else {
                self.events_recorded.store(true, Ordering::SeqCst);
                self.status_text = format!(
                    "Task: {} - Mouse: ({}, {})",
                    self.task_name, current_pos.0, current_pos.1
                );
            }
            self.detailed_writer
                .flush()
                .unwrap_or_else(|e| eprintln!("Error flushing: {}", e));
            self.last_mouse_pos = current_pos;
        }
    }
}
