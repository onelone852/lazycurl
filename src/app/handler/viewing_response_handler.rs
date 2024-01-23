use crossterm::event::Event;

use crate::{
    app::state::AppState,
    event::{client::Client, util::is_ctrl_c},
};

use super::{EventHandler, Handle, RequestResult, DONE, EXIT};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ViewingResponseHandler;

impl EventHandler for ViewingResponseHandler {
    fn handle(&self, event: Event, state: &mut AppState, _: &Client<RequestResult>) -> Handle {
        if let Event::Key(key) = event {
            use crossterm::event::{KeyCode::*, KeyModifiers};
            let viewing_state = state
                .status_state
                .view_state
                .as_mut()
                .expect("User should be viewing response");
            if is_ctrl_c(key) {
                EXIT
            } else if key.modifiers == KeyModifiers::NONE {
                match key.code {
                    Up => {
                        *viewing_state.get_mut() += 1;
                    }
                    Down => {
                        viewing_state.set(viewing_state.get().saturating_sub(1));
                    }
                    Esc => {
                        state.status_state.end_view();
                    }
                    _ => (),
                }
                DONE
            } else {
                DONE
            }
        } else {
            DONE
        }
    }
}
