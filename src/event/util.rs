use crossterm::event::{Event, KeyEvent};

pub fn is_keyevent_pressed(event: &Event) -> bool {
    if let &Event::Key(key) = event {
        key.kind == crossterm::event::KeyEventKind::Press
    } else {
        true
    }
}

pub fn is_ctrl_c(key: KeyEvent) -> bool {
    use crossterm::event::KeyModifiers;
    key.code == crossterm::event::KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL
}
