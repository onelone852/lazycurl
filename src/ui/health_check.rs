use ratatui::{prelude::*, style::Stylize, widgets::*};

use crate::app::state::AppState;

use super::{util, ERROR_COLOR, WARNING_COLOR};

pub fn show_health_checking(frame: &mut Frame, state: &AppState, area: Rect) {
    let health = &state.health;
    if health.have_problem() {
        let have_errors = health.have_errors();
        let (accent_color, subtitle) = if have_errors {
            (ERROR_COLOR, "Enter To Exit")
        } else {
            (WARNING_COLOR, "Enter to ignore")
        };
        let problems = health.problems();
        let mut problem_text = Text::raw("");
        for problem in problems {
            let level = problem.level();
            let mut line = Line::raw("");

            line.spans.push(level.text().bold());
            line.spans.push(": ".bold());
            line.spans.push(Span::raw(problem.to_string()));
            line.patch_style(Style::default().fg(level.color()));
            problem_text.lines.push(line);
        }
        let para = Paragraph::new(problem_text).reset().block(
            Block::new()
                .title(
                    block::Title::default()
                        .content("HEALTH CHECK PROBLEMS".bold().fg(accent_color))
                        .alignment(Alignment::Center),
                )
                .title(
                    block::Title::default()
                        .content(subtitle.fg(accent_color))
                        .alignment(Alignment::Center)
                        .position(block::Position::Bottom),
                )
                .fg(accent_color)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        let centered_area = util::centered_area(area, 50, 50);
        frame.render_widget(Clear, centered_area); // NOTE: Clear for not overlapping
        frame.render_widget(para, centered_area);
    }
}
