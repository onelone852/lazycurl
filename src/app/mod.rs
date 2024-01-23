pub mod handler;
pub mod position;
pub mod state;

use std::{error::Error, time::Duration};

use crossterm::event::{Event, EventStream};
use futures::FutureExt;

use crate::{
    event::{client::Client, listener::Listener},
    ui::UI,
};

use self::{handler::RequestResult, state::AppState};

pub struct App {
    ui: UI,
    event_listener: Listener<Result<Event, std::io::Error>>,
    state: AppState,
    client: Client<RequestResult>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // TODO: Use DI instead
        let ui = UI::new()?;
        let listener = Listener::new(EventStream::new());
        let client = Client::new();
        Ok(Self {
            ui,
            event_listener: listener,
            state: AppState::new(),
            client,
        })
    }

    fn check_health(&mut self) -> Result<(), std::io::Error> {
        use state::health::Problem::*;
        if self.client.is_invalid() {
            self.state.health.add_problem(ClientCannotBeBuilt);
        }
        Ok(())
    }

    async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.state.check_channel(&mut self.client)?;
            self.ui.render(&self.state)?;
            let opevent = match self.event_listener.listen() {
                Some(res) => Some(res?),
                None => None,
            };
            if let Some(event) = opevent {
                if self
                    .state
                    .get_handler()
                    .handle(event, &mut self.state, &self.client)?
                    .is_break()
                {
                    break Ok(());
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.ui.enter()?;
        self.ui.clear()?;
        self.check_health()?;
        let res = std::panic::AssertUnwindSafe(self.main_loop())
            .catch_unwind()
            .await;
        self.ui.clear()?;
        self.ui.exit()?;
        if let Ok(real_res) = res {
            real_res?;
        }
        Ok(())
    }

    pub fn create_and_run() -> Result<(), Box<dyn Error>> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Tokio Runtime should can be built")
            .block_on(async {
                let app = App::new()?;
                app.run().await
            })
    }
}
