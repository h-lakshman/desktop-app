pub mod gui;
pub mod monitor;
pub mod types;

// Re-export commonly used items
pub use gui::MonitorApp;
pub use monitor::ActivityMonitor;
pub use types::{Action, DetailedEvent, Session};
