use crossterm::event::Event;

use crate::{
    app::{handler::util::change_to_another_pane, state::AppState},
    event::{client::Client, util::is_ctrl_c},
};

use super::{EventHandler, Handle, RequestResult, DONE, EXIT};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConsoleHandler;

impl EventHandler for ConsoleHandler {
    fn handle(&self, event: Event, state: &mut AppState, _: &Client<RequestResult>) -> Handle {
        if let Event::Key(key) = event {
            use crossterm::event::{KeyCode::*, KeyModifiers};
            change_to_another_pane(key, state);
            if is_ctrl_c(key) {
                EXIT
            } else if key.modifiers == KeyModifiers::NONE {
                match key.code {
                    Up => *state.console_state.get_mut_scroll_up() += 1,
                    Down => {
                        *state.console_state.get_mut_scroll_up() =
                            state.console_state.scroll_up().saturating_sub(1);
                    }
                    _ => (),
                }
                DONE
            } else if key.modifiers == KeyModifiers::ALT && key.code == Char('c') {
                state.console_state.clear_messages();
                DONE
            } else {
                DONE
            }
        } else {
            DONE
        }
    }
}
