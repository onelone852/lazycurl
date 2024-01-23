use crossterm::event::Event;

use crate::{
    app::{handler::util::change_to_another_pane, state::AppState},
    event::{client::Client, util::is_ctrl_c},
};

use super::{EventHandler, Handle, RequestResult, DONE, EXIT};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StatusHandler;

impl EventHandler for StatusHandler {
    fn handle(&self, event: Event, state: &mut AppState, _: &Client<RequestResult>) -> Handle {
        if let Event::Key(key) = event {
            use crossterm::event::{KeyCode::*, KeyModifiers};

            change_to_another_pane(key, state);
            if is_ctrl_c(key) {
                EXIT
            } else if key.modifiers == KeyModifiers::NONE {
                match key.code {
                    Up => state.status_state.prev_response(),
                    Down => state.status_state.next_response(),
                    Enter => {
                        state.status_state.try_start_view();
                    }
                    _ => (),
                }
                DONE
            } else if key.modifiers == KeyModifiers::ALT && key.code == Char('c') {
                state.status_state.clear_responses();
                DONE
            } else {
                DONE
            }
        } else {
            DONE
        }
    }
}
