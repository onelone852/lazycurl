use std::error::Error;

use crossterm::event::{Event, KeyModifiers};

use crate::{
    app::state::{AppState, RequestAreaEditing},
    event::{client::Client, util::is_ctrl_c},
};

use super::{
    util::{change_to_another_pane, Response},
    EventHandler, Handle, RequestResult, DONE, EXIT,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RequestHandler;

fn send_request(
    state: &mut AppState,
    client: &Client<RequestResult>,
) -> Result<(), Box<dyn Error>> {
    let item_num = state.request_area_state.get_request_item_num();
    let link = &state.request_area_state.link;
    match link.parse::<url::Url>() {
        Ok(url) => {
            let sender = client.get_sender();
            let request = client
                .get_inner()
                .request(state.request_area_state.get_method(), url);
            tokio::spawn(async move {
                let ori_response = request.send().await;
                let response = match ori_response {
                    Ok(res) => {
                        let status = res.status();
                        let body = res.text().await.expect("Non-utf8");
                        Ok(Response { body, status })
                    }
                    Err(err) => Err(Box::new(err) as Box<dyn Error + Send>),
                };
                let res = RequestResult { response, item_num };
                sender
                    .send(res)
                    .await
                    .expect("The recver should not have dropped");
            });
            Ok(())
        }
        Err(err) => {
            let msg = format!("Invalid URL: {}", err);
            state.console_state.add_message(msg);
            Ok(())
        }
    }
}

fn none_editing_handler(
    event: Event,
    state: &mut AppState,
    client: &Client<RequestResult>,
) -> Handle {
    if let Event::Key(key) = event {
        if is_ctrl_c(key) {
            EXIT
        } else {
            change_to_another_pane(key, state);
            use crossterm::event::KeyCode::*;

            if key.modifiers == KeyModifiers::NONE {
                match key.code {
                    Left => state.request_area_state.prev_method(),
                    Right => state.request_area_state.next_method(),
                    Char('r') => state.request_area_state.is_editing = RequestAreaEditing::Link,
                    _ => (),
                }
            } else if key.modifiers == KeyModifiers::CONTROL && key.code == Char('r') {
                send_request(state, client)?;
            }
            DONE
        }
    } else {
        DONE
    }
}

fn link_editing_handler(event: Event, state: &mut AppState) -> Handle {
    if let Event::Key(key) = event {
        if is_ctrl_c(key) {
            EXIT
        } else {
            if key.modifiers == KeyModifiers::NONE {
                use crossterm::event::KeyCode::*;
                match key.code {
                    Char(ch) => state.request_area_state.link.push(ch),
                    Backspace | Delete => {
                        state.request_area_state.link.pop();
                    }
                    Esc | Enter => state.request_area_state.is_editing = RequestAreaEditing::None,
                    _ => (),
                }
            }
            DONE
        }
    } else if let Event::Paste(content) = event {
        state.request_area_state.link.push_str(&content);
        DONE
    } else {
        DONE
    }
}

impl EventHandler for RequestHandler {
    fn handle(&self, event: Event, state: &mut AppState, client: &Client<RequestResult>) -> Handle {
        use crate::app::state::RequestAreaEditing::*;
        match state.request_area_state.is_editing {
            None => none_editing_handler(event, state, client),
            Link => link_editing_handler(event, state),
        }
    }
}
