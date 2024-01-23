use std::error::Error;

use crossterm::event::KeyEvent;
use reqwest::StatusCode;

use crate::app::state::AppState;

#[derive(Debug, Clone)]
pub struct Response {
    pub body: String,
    pub status: StatusCode,
}

#[derive(Debug)]
pub struct RequestResult {
    pub response: Result<Response, Box<dyn Error + Send>>,
    pub item_num: usize,
}

pub fn change_to_another_pane(key: KeyEvent, state: &mut AppState) {
    use crossterm::event::KeyModifiers;
    if key.modifiers == KeyModifiers::SHIFT {
        use crossterm::event::KeyCode::*;
        match key.code {
            Up => state.set_position(state.position().up_limited()),
            Down => state.set_position(state.position().down_limited()),
            Left => state.set_position(state.position().left_limited()),
            Right => state.set_position(state.position().right_limited()),
            _ => (),
        }
    }
}
