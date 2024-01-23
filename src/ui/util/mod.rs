mod str_to_lines;

use std::cell::Cell;

pub use str_to_lines::*;

use ratatui::{
    prelude::*,
    widgets::{Clear, Widget},
};

pub struct LCLayout {
    pub upper_left: Rect,
    pub lower_left: Rect,
    pub upper_right: Rect,
    pub lower_right: Rect,
}

pub fn create_layout(area: Rect) -> LCLayout {
    let layout1 = Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);
    let mut layout = layout1.iter().map(|a| {
        Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(*a)
    });
    let upper_part = layout.next().expect("Should success");
    let upper_left = upper_part[0];
    let upper_right = upper_part[1];
    let lower_part = layout.next().expect("Should success");
    let lower_left = lower_part[0];
    let lower_right = lower_part[1];
    LCLayout {
        upper_left,
        lower_left,
        upper_right,
        lower_right,
    }
}

/// Create a centered area.
pub fn centered_area(area: Rect, height_percentage: u16, width_percentage: u16) -> Rect {
    let horizaontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([
            (100 - width_percentage) / 2,
            width_percentage,
            (100 - width_percentage) / 2,
        ]))
        .split(area);
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(Constraint::from_percentages([
            (100 - height_percentage) / 2,
            height_percentage,
            (100 - height_percentage) / 2,
        ]))
        .split(horizaontal_layout[1]);
    vertical_layout[1]
}

pub fn get_inner(frame: &Frame) -> Rect {
    crate::ui::UI::title().inner(frame.size())
}

pub fn popup<W: Widget>(area: Rect, frame: &mut Frame, widget: W) {
    frame.render_widget(Clear, area);
    frame.render_widget(widget, area);
}

pub fn scroll_up(
    need_showing_para_height: usize,
    area: Rect,
    scroll_up: &Cell<u16>,
) -> (usize, usize) {
    let mut lower_bound = if need_showing_para_height > area.height as usize {
        need_showing_para_height - area.height as usize
    } else {
        0
    };
    if lower_bound == 0 {
        scroll_up.set(0);
    } else if scroll_up.get() as usize > lower_bound {
        scroll_up.set(lower_bound as u16);
        lower_bound = 0;
    } else {
        lower_bound -= scroll_up.get() as usize;
    }
    let upper_bound = if lower_bound + area.height as usize >= need_showing_para_height {
        need_showing_para_height
    } else {
        lower_bound + area.height as usize + 1
    };
    (lower_bound, upper_bound)
}
