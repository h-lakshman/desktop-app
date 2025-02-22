# Desktop Activity Monitor

A Rust-based desktop activity monitoring tool that records mouse movements and keyboard actions for AI training purposes.

## Features

- GUI window with simple start/stop controls
- Records mouse movements with coordinates
- Captures keyboard inputs
- Saves sessions to CSV files
- Real-time status updates
- Minimal system resource usage

## Data Collection

The application collects data in two files:

1. `monitoring_sessions.csv`: Contains complete sessions with all actions
2. `latest_session_details.csv`: Contains detailed events from the current/last session

### Session Data Format

The `monitoring_sessions.csv` file contains the following columns:

- session_id: Unique identifier for each monitoring session (timestamp-based)
- start_time: ISO 8601 formatted start time
- end_time: ISO 8601 formatted end time
- actions: Semicolon-separated list of actions in chronological order, where each action is formatted as:
  - Mouse moves: `{mouse,timestamp,(x,y)}`
  - Key presses: `{key,timestamp,keys_pressed}`

Example:

```csv
session_id,start_time,end_time,actions
20240120_123456,2024-01-20T12:34:56Z,2024-01-20T12:35:56Z,{mouse,2024-01-20T12:34:57Z,(100,200)};{key,2024-01-20T12:34:58Z,"A+B+C"}
```

### Detailed Event Data

The `latest_session_details.csv` file contains detailed events from the current/last session with columns:

- timestamp: ISO 8601 formatted timestamp
- event_type: Type of event (keyboard/mouse_move)
- details: Specific details about the event
- mouse_x: Current X coordinate of the mouse
- mouse_y: Current Y coordinate of the mouse

## Usage

1. Run the application using `cargo run`
2. A window titled "Desktop Activity Monitor" will appear
3. Click "Start Monitoring" to begin recording
4. Click "Stop Monitoring" to end the session
5. Close the window to exit the application

The application will show real-time status updates about mouse movements and keyboard actions in the window.

## Requirements

- Rust 1.56 or higher
- Windows 10 or higher

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
