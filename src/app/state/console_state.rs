use std::cell::Cell;

#[derive(Debug)]
pub struct ConsoleState {
    content: String,
    scroll_up: Cell<u16>,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            content: "LAZYCURL activate!".to_string(),
            scroll_up: Cell::new(0),
        }
    }
}

impl ConsoleState {
    pub fn add_message<T>(&mut self, msg: T)
    where
        T: AsRef<str>,
    {
        let leading = if self.content.is_empty() {
            "> "
        } else {
            "\n> "
        };
        self.content.push_str(leading);
        self.content.push_str(msg.as_ref());
    }

    pub fn clear_messages(&mut self) {
        self.content.clear();
    }

    pub fn messages(&self) -> &str {
        &self.content
    }

    pub fn scroll_up(&self) -> u16 {
        self.scroll_up.get()
    }

    pub fn scroll_up_cell(&self) -> &Cell<u16> {
        &self.scroll_up
    }

    pub fn set_scroll_up(&self, scroll_up: u16) {
        self.scroll_up.set(scroll_up)
    }

    pub fn get_mut_scroll_up(&mut self) -> &mut u16 {
        self.scroll_up.get_mut()
    }
}
