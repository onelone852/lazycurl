mod health_check;
mod lower_left;
mod upper_left;
mod upper_right;
pub mod util;

use std::io::Stdout;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::Stylize, widgets::*};

use crate::app::state::AppState;

use self::health_check::show_health_checking;

pub const SELECTED_PANE_COLOR: Color = Color::LightYellow;
pub const NORMAL_PANE_COLOR: Color = Color::White;
pub const HINT_COLOR: Color = Color::Green;
pub const WARNING_COLOR: Color = Color::Yellow;
pub const ERROR_COLOR: Color = Color::LightRed;
pub const SUCCESS_COLOR: Color = Color::LightBlue;

trait UiWidget {
    fn render(&self, area: Rect, buf: &mut Buffer, state: &AppState);
}

impl<'a> StatefulWidget for &'a dyn UiWidget {
    type State = &'a AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut &'a AppState) {
        self.render(area, buf, *state);
    }
}

impl UiWidget for AppState {
    fn render(&self, area: Rect, buf: &mut Buffer, _: &AppState) {
        if area.height < 12 || area.width < 18 {
            Block::default()
                .fg(ERROR_COLOR)
                .title("Not Enough Space for Render Panels".bold())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .render(area, buf);

            return;
        }
        let title_block = UI::title();
        let inner_area = title_block.inner(area);
        title_block.render(area, buf);

        let util::LCLayout {
            upper_left,
            lower_left,
            upper_right,
            lower_right,
        } = util::create_layout(area);
        self.request_area_state.render(upper_left, buf, self);
        self.console_state.render(lower_left, buf, self);
        let popup = upper_right::generate_upper_right_area(frame, state, upper_right);
        frame.render_widget(
            Paragraph::new("I am here!").fg(Color::White).block(
                Block::default()
                    .fg(SELECTED_PANE_COLOR)
                    .title("Hi")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Left),
            ),
            lower_right,
        );
        popup.map(|func| func(frame));
        show_health_checking(frame, state, area);
    }
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Default for UI {
    fn default() -> Self {
        Self::new().expect("Default should succeed")
    }
}

impl UI {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
        })
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.terminal.clear()
    }

    pub fn enter(&mut self) -> Result<(), std::io::Error> {
        std::io::stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), std::io::Error> {
        std::io::stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn render(&mut self, state: &AppState) -> Result<(), std::io::Error> {
        self.terminal.draw(|f| {
            let term_area = Self::title().inner(f.size());
            Self::ui(f, state);
            show_health_checking(f, state, term_area);
        })?;
        Ok(())
    }

    fn title() -> Block<'static> {
        Block::new()
            .fg(Color::LightCyan)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(
                block::Title::default()
                    .content("LAZYCURL".on_cyan().white().bold())
                    .alignment(Alignment::Center),
            )
            .title(
                block::Title::default()
                    .content("Ctrl-C to EXIT")
                    .alignment(Alignment::Center)
                    .position(block::Position::Bottom),
            )
    }

    fn ui(frame: &mut Frame, state: &AppState) {
        let term_area = frame.size();
        if term_area.height < 12 || term_area.width < 18 {
            frame.render_widget(
                Block::default()
                    .fg(ERROR_COLOR)
                    .title("Not Enough Space for Render Panels".bold())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
                term_area,
            );
            return;
        }
        let title_block = Self::title();
        let area = title_block.inner(term_area);
        frame.render_widget(title_block, term_area);

        let util::LCLayout {
            upper_left,
            lower_left,
            upper_right,
            lower_right,
        } = util::create_layout(area);
        upper_left::generate_lower_left_area(frame, state, upper_left);
        lower_left::generate_lower_left_area(frame, state, lower_left);
        let popup = upper_right::generate_upper_right_area(frame, state, upper_right);
        frame.render_widget(
            Paragraph::new("I am here!").fg(Color::White).block(
                Block::default()
                    .fg(SELECTED_PANE_COLOR)
                    .title("Hi")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Left),
            ),
            lower_right,
        );
        popup.map(|func| func(frame));
        show_health_checking(frame, state, area);
    }
}
