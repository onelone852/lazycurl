use std::borrow::Cow;

use ratatui::{prelude::*, style::Stylize, widgets::*};

use crate::app::{
    position::PositionArea,
    state::{AppState, RequestAreaEditing, RequestAreaState},
};

use super::{UiWidget, NORMAL_PANE_COLOR, SELECTED_PANE_COLOR};

/// Create method options with a specfic iterator.
fn create_method_option<'a, I>(method_index: usize, iter: I) -> Paragraph<'a>
where
    I: IntoIterator,
    I::Item: Into<Cow<'a, str>>,
{
    let mut line = Line::raw("").alignment(Alignment::Center);
    for (index, raw_method) in iter.into_iter().enumerate() {
        let method = raw_method.into();
        let mut span = match method.as_ref() {
            "GET" => Span::raw(method).bg(Color::Yellow),
            "POST" => Span::raw(method).bg(Color::Magenta),
            "PUT" => Span::raw(method).bg(Color::LightBlue),
            "DELETE" => Span::raw(method).bg(Color::LightRed),
            _ => Span::raw(method).bg(Color::Gray),
        }
        .bold()
        .fg(Color::White);
        if index == method_index {
            span = span.underlined();
        }
        line.spans.push(span);
        line.spans.push(Span::raw(" "));
    }
    line.spans.pop();
    Paragraph::new(line)
}

impl UiWidget for RequestAreaState {
    fn render(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let border_color = if state.position() == PositionArea::UpperLeft {
            SELECTED_PANE_COLOR
        } else {
            NORMAL_PANE_COLOR
        };

        let title_block = Block::default()
            .fg(border_color)
            .title("Request".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner_area = title_block.inner(area);
        title_block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(2),
            ])
            .split(inner_area);
        create_method_option(self.method_index(), self.methods().iter().map(Box::as_ref))
            .render(layout[0], buf);

        let (link_border_color, hint_text) = if self.is_editing == RequestAreaEditing::Link {
            (SELECTED_PANE_COLOR, "Esc or Enter to exit")
        } else {
            (NORMAL_PANE_COLOR, "R to edit")
        };

        Paragraph::new(state.request_area_state.link.as_str())
            .fg(Color::White)
            .block(
                Block::new()
                    .fg(link_border_color)
                    .title(block::Title::default().content("Link".bold()))
                    .title(
                        block::Title::default()
                            .content(hint_text.italic())
                            .position(block::Position::Bottom)
                            .alignment(Alignment::Right),
                    )
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(layout[1], buf);
    }
}
