use rand::Rng;

#[derive(Debug)]
pub enum CallEndReason {
    UserHungUp,
    PeerHungUp,
    ConnectionLost,
    CallRejected,
    PeerUnavailable,
    Timeout,
    Unknown,
}

#[derive(Debug)]
pub enum UiState {
    Login,
    Dashboard,
    OutgoingCall { callee_id: String },
    IncomingCall { caller_id: String },
    InCall { peer_id: String, is_video: bool },
    CallEnded { reason: CallEndReason },
    Settings,
    Error { message: String },
}

pub struct AppState {
    pub user_id: u32,
    pub user_name: String,
    pub ui_state: UiState,
    pub temp_callee: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user_id: rand::rng().random_range(100_000..=1_000_000),
            ui_state: UiState::Login,
            temp_callee: String::new(),
            user_name: String::new()
        }
    }
    pub fn toggle_video(&mut self) {
        if let UiState::InCall { is_video, .. } = &mut self.ui_state {
            *is_video = !*is_video;
        }
    }
}