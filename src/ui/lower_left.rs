use ratatui::{prelude::*, style::Stylize, widgets::*};

use crate::app::{
    position::PositionArea,
    state::{AppState, ConsoleState, StatusState},
};

use super::{
    util::{scroll_up, str_to_lines},
    UiWidget, NORMAL_PANE_COLOR, SELECTED_PANE_COLOR,
};

impl UiWidget for StatusState {
    fn render(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let border_color = if state.position() == PositionArea::LowerLeft {
            SELECTED_PANE_COLOR
        } else {
            NORMAL_PANE_COLOR
        };
        let title_block = Block::default()
            .fg(border_color)
            .title("Console".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner_area = title_block.inner(area);
        title_block.render(area, buf);
        let messages = state.console_state.messages();
        let para: Vec<&str> = str_to_lines(messages, inner_area.width as usize).collect();

        let (lower_bound, upper_bound) =
            scroll_up(para.len(), inner_area, state.console_state.scroll_up_cell());
        Paragraph::new(para[lower_bound..upper_bound].join("\n"))
            .fg(Color::White)
            .render(inner_area, buf);
    }
}

impl UiWidget for ConsoleState {
    fn render(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let border_color = if state.position() == PositionArea::LowerLeft {
            SELECTED_PANE_COLOR
        } else {
            NORMAL_PANE_COLOR
        };
        let title_block = Block::default()
            .fg(border_color)
            .title("Console".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner_area = title_block.inner(area);
        title_block.render(area, buf);
        let messages = self.messages();
        let para: Vec<&str> = str_to_lines(messages, inner_area.width as usize).collect();

        let (lower_bound, upper_bound) = scroll_up(para.len(), inner_area, self.scroll_up_cell());

        Paragraph::new(para[lower_bound..upper_bound].join("\n"))
            .fg(Color::White)
            .render(inner_area, buf);
    }
}
