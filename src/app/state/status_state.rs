use std::cell::Cell;

use crate::app::handler::RequestResult;

#[derive(Debug)]
pub struct StatusState {
    responses: Vec<RequestResult>,
    response_index: usize,
    pub view_state: Option<Cell<u16>>,
}

impl Default for StatusState {
    fn default() -> Self {
        Self {
            responses: Vec::new(),
            response_index: 0,
            view_state: None,
        }
    }
}

impl StatusState {
    pub fn add_response<T>(&mut self, response: T)
    where
        T: Into<RequestResult>,
    {
        if self.responses.len() == self.response_index + 1 {
            self.response_index += 1;
        }
        self.responses.push(response.into());
    }

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    pub fn responses(&self) -> &[RequestResult] {
        &self.responses
    }

    pub fn get_response(&self) -> Option<&RequestResult> {
        self.responses.get(self.response_index)
    }

    pub fn get_response_index(&self) -> usize {
        self.response_index
    }

    pub fn next_response(&mut self) {
        if self.response_index < self.responses.len() - 1 {
            self.response_index += 1;
        }
    }

    pub fn prev_response(&mut self) {
        if self.response_index != 0 {
            self.response_index -= 1;
        }
    }

    pub fn is_viewing(&self) -> bool {
        self.view_state.is_some()
    }

    pub fn try_start_view(&mut self) -> bool {
        if self.responses.len() != 0 && !self.is_viewing() {
            self.view_state = Some(Cell::new(0));
            true
        } else {
            false
        }
    }

    pub fn end_view(&mut self) -> bool {
        if self.is_viewing() {
            self.view_state = None;
            true
        } else {
            false
        }
    }
}
