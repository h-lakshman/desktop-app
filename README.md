# Desktop Activity Monitor

A Rust application that monitors desktop activity (mouse movements and keyboard inputs) for AI training data collection.

## Features

- Task-based activity monitoring
- Mouse movement and keyboard input tracking
- Real-time status updates
- CSV data storage

## Data Files

- `monitoring_sessions.csv`: Complete sessions with all actions

  ```csv
  session_id,task_name,start_time,end_time,actions
  20240120_123456,Writing Email,2024-01-20T12:34:56Z,2024-01-20T12:35:56Z,{mouse,2024-01-20T12:34:57Z,(100,200)};{key,2024-01-20T12:34:58Z,"A+B+C"}
  ```

- `latest_session_details.csv`: Detailed events from current session

## Usage

1. Run `cargo run`
2. Enter task name
3. Click "Start Monitoring"
4. Perform your task
5. Click "Stop Monitoring"

## Requirements

- Rust 1.56+
- Windows 10+

## Project Structure

```
src/
├── main.rs     # Application entry point
├── lib.rs      # Library exports
├── types.rs    # Data structures
├── monitor.rs  # Activity monitoring
└── gui.rs      # User interface
```

## Building

```bash
cargo build --release
```

The compiled binary will be in `target/release/desk-monitor.exe`

## Dependencies

- device_query: For mouse and keyboard monitoring
- eframe: For the GUI window
- csv: For data storage
- chrono: For timestamp handling
- serde: For data serialization
- anyhow: For error handling
