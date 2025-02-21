# Desktop Activity Monitor

A Rust-based desktop activity monitoring tool that records mouse movements and keyboard actions for AI training purposes.

## Features

- System tray application for easy control
- Records mouse movements with coordinates
- Captures keyboard inputs
- Saves all activity to a CSV file
- Minimal system resource usage
- Easy to start/stop monitoring

## Data Collection

The application collects the following data:

- Timestamp of each event
- Event type (keyboard or mouse)
- Event details (key pressed, mouse coordinates)
- Mouse X and Y coordinates

All data is stored in `activity_log.csv` in the application directory.

## Usage

1. Run the application using `cargo run`
2. A system tray icon will appear
3. Right-click the tray icon to access the menu
4. Select "Start Monitoring" to begin recording
5. Select "Stop Monitoring" to pause recording
6. Select "Quit" to exit the application

## Data Format

The CSV file contains the following columns:

- timestamp: ISO 8601 formatted timestamp
- event_type: Type of event (keyboard/mouse_move)
- details: Specific details about the event
- mouse_x: Current X coordinate of the mouse
- mouse_y: Current Y coordinate of the mouse

## Requirements

- Rust 1.56 or higher
- Windows 10 or higher

## Building

```bash
cargo build --release
```

The compiled binary will be in `target/release/desk-monitor.exe`
