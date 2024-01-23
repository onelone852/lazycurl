mod console_state;
pub mod health;
mod request_area_state;
mod status_state;

use std::error::Error;

use crate::event::client::Client;

use self::health::Health;
pub use self::{console_state::*, request_area_state::*, status_state::*};

use super::{
    handler::{EventHandler, RequestResult},
    position::PositionArea,
};

#[derive(Debug, Default)]
pub struct AppState {
    pub request_area_state: RequestAreaState,
    pub console_state: ConsoleState,
    pub status_state: StatusState,
    position: PositionArea,
    pub health: Health,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            request_area_state: RequestAreaState::default(),
            console_state: ConsoleState::default(),
            status_state: StatusState::default(),
            position: PositionArea::default(),
            health: Health::default(),
        }
    }

    pub fn position(&self) -> PositionArea {
        self.position
    }

    pub fn set_position(&mut self, position: PositionArea) {
        self.position = position;
    }

    pub fn get_handler(&self) -> &'static dyn EventHandler {
        self.into()
    }

    pub fn check_channel(
        &mut self,
        client: &mut Client<RequestResult>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(msg) = client.immediate_recv() {
            match msg.response {
                Ok(_) => {
                    self.status_state.add_response(msg);
                    Ok(())
                }
                Err(err) => Err(err),
            }
        } else {
            Ok(())
        }
    }
}
