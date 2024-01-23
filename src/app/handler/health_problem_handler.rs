use crossterm::event::Event;

use crate::{
    app::state::AppState,
    event::{client::Client, util::is_ctrl_c},
};

use super::{EventHandler, Handle, RequestResult, DONE, EXIT};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HealthProblemHandler;

impl EventHandler for HealthProblemHandler {
    fn handle(&self, event: Event, state: &mut AppState, _: &Client<RequestResult>) -> Handle {
        if let Event::Key(key) = event {
            if is_ctrl_c(key) {
                EXIT
            } else {
                use crossterm::event::{KeyCode::*, KeyModifiers};
                match key.code {
                    Enter if key.modifiers == KeyModifiers::NONE => {
                        if state.health.have_errors() {
                            EXIT
                        } else {
                            state.health.ignore_problems();
                            DONE
                        }
                    }
                    _ => DONE,
                }
            }
        } else {
            DONE
        }
    }
}
