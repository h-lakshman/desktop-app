use serde::Serialize;

#[derive(Debug, Clone)]
pub enum Action {
    MouseMove {
        timestamp: String,
        coords: (i32, i32),
    },
    KeyPress {
        timestamp: String,
        keys: Vec<String>,
    },
}

impl Action {
    pub fn to_csv_string(&self) -> String {
        match self {
            Action::MouseMove { timestamp, coords } => {
                format!("{{mouse,{},({},{})}}", timestamp, coords.0, coords.1)
            }
            Action::KeyPress { timestamp, keys } => {
                format!("{{key,{},{:?}}}", timestamp, keys.join("+"))
            }
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub session_id: String,
    pub task_name: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub actions: Vec<Action>,
}

impl Session {
    pub fn to_csv_record(&self) -> Vec<String> {
        let actions_str = self
            .actions
            .iter()
            .map(|action| action.to_csv_string())
            .collect::<Vec<_>>()
            .join(";");

        vec![
            self.session_id.clone(),
            self.task_name.clone(),
            self.start_time.clone(),
            self.end_time.clone().unwrap_or_default(),
            actions_str,
        ]
    }
}

#[derive(Debug, Serialize)]
pub struct DetailedEvent {
    pub timestamp: String,
    pub task_name: String,
    pub event_type: String,
    pub details: String,
    pub mouse_x: i32,
    pub mouse_y: i32,
} 