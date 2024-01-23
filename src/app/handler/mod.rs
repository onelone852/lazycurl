mod console_handler;
mod health_problem_handler;
mod requesting_handler;
mod status_handler;
mod util;
mod viewing_response_handler;

pub use {
    health_problem_handler::*, requesting_handler::*, status_handler::*, util::RequestResult,
};

use std::{error::Error, ops::ControlFlow};

use crossterm::event::Event;

use crate::{app::position::PositionArea, event::client::Client};

use super::state::AppState;

pub type Handle = Result<ControlFlow<()>, Box<dyn Error>>;

const EXIT: Handle = Ok(ControlFlow::Break(()));
const DONE: Handle = Ok(ControlFlow::Continue(()));

pub trait EventHandler {
    fn handle(&self, event: Event, state: &mut AppState, client: &Client<RequestResult>) -> Handle;
}

impl<'a> From<&AppState> for &'a dyn EventHandler {
    fn from(state: &AppState) -> Self {
        if state.health.have_problem() {
            &health_problem_handler::HealthProblemHandler
        } else {
            match state.position() {
                PositionArea::LowerLeft => &console_handler::ConsoleHandler,
                PositionArea::UpperRight if state.status_state.is_viewing() => {
                    &viewing_response_handler::ViewingResponseHandler
                }
                PositionArea::UpperRight => &status_handler::StatusHandler,
                _ => &requesting_handler::RequestHandler,
            }
        }
    }
}
