use ratatui::{prelude::*, style::Stylize, widgets::*};

use crate::app::{position::PositionArea, state::AppState};

use super::{
    util::{self, centered_area, get_inner, scroll_up, str_to_lines},
    ERROR_COLOR, NORMAL_PANE_COLOR, SELECTED_PANE_COLOR, SUCCESS_COLOR,
};

pub fn generate_upper_right_area<'a>(
    frame: &mut Frame,
    state: &'a AppState,
    area: Rect,
) -> Option<Box<dyn FnOnce(&mut Frame) + 'a>> {
    let border_color = if state.position() == PositionArea::UpperRight {
        SELECTED_PANE_COLOR
    } else {
        NORMAL_PANE_COLOR
    };
    let title_block = Block::default()
        .fg(border_color)
        .title("Status".bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let inner_area = title_block.inner(area);
    frame.render_widget(title_block, area);
    let status = &state.status_state;
    let response = status.responses();
    let index = status.get_response_index();
    let height = inner_area.height as usize;
    let is_full = response.len() >= height;
    let (lower_bound, upper_bound) = if is_full {
        let in_last_area = index >= response.len() - height;
        if in_last_area {
            (response.len() - height, response.len())
        } else {
            (index, index + height)
        }
    } else {
        (0, response.len())
    };

    let para: Vec<Line<'static>> = status.responses()[lower_bound..upper_bound]
        .iter()
        .enumerate()
        .map(|(i, res)| {
            let item_color = if res.response.is_ok() {
                SUCCESS_COLOR
            } else {
                ERROR_COLOR
            };
            Line::from(if i + lower_bound == index {
                Span::raw(format!("Response #{}", res.item_num))
                    .fg(item_color)
                    .underlined()
            } else {
                Span::raw(format!("Response #{}", res.item_num)).fg(item_color)
            })
        })
        .collect();

    let paragraph = Text::from(para);
    frame.render_widget(Paragraph::new(paragraph).fg(Color::White), inner_area);

    if let Some(response) = status.get_response().filter(|_| status.is_viewing()) {
        Some(Box::new(|frame| {
            let inner_area = get_inner(frame);
            let popup_area = centered_area(inner_area, 50, 50);
            let res = response.response.as_ref().expect("unimplemented");
            let item_color = if response.response.is_ok() {
                SUCCESS_COLOR
            } else {
                ERROR_COLOR
            };
            let block = Block::default()
                .fg(item_color)
                .title(format!("Response #{}", response.item_num).bold())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let body_space = block.inner(popup_area);
            let body = str_to_lines(&res.body, body_space.width as usize).collect::<Vec<&str>>();
            let scroll_up = state
                .status_state
                .view_state
                .as_ref()
                .expect("Should be viewing");
            let (lower_bound, upper_bound) = util::scroll_up(body.len(), popup_area, scroll_up);

            let widget = Paragraph::new(body[lower_bound..upper_bound].join("\n"))
                .fg(Color::White)
                .block(block);

            util::popup(popup_area, frame, widget);
        }))
    } else {
        None
    }
}
