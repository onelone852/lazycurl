use std::fmt::Display;

use ratatui::style::Color;

#[derive(Debug, Default)]
pub struct Health {
    problems: Vec<Problem>,
}

impl Health {
    pub fn problems(&self) -> &[Problem] {
        &self.problems
    }

    pub fn have_problem(&self) -> bool {
        !self.problems.is_empty()
    }

    pub fn ignore_problems(&mut self) {
        self.problems = Vec::new();
    }

    pub fn add_problem<P>(&mut self, problem: P)
    where
        P: Into<Problem>,
    {
        self.problems.push(problem.into());
    }

    pub fn have_errors(&self) -> bool {
        self.problems
            .iter()
            .any(|p| p.level() == WarningLevel::Error)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WarningLevel {
    Hint,
    Warning,
    Error,
}

impl WarningLevel {
    pub fn color(&self) -> Color {
        use crate::ui::*;
        use WarningLevel::*;
        match self {
            Hint => HINT_COLOR,
            Warning => WARNING_COLOR,
            Error => ERROR_COLOR,
        }
    }

    pub fn text(&self) -> &'static str {
        use WarningLevel::*;
        match self {
            Hint => "HINT",
            Warning => "WARNING",
            Error => "ERROR",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Problem {
    ClientCannotBeBuilt,
}

impl Problem {
    pub fn level(&self) -> WarningLevel {
        use Problem::*;
        use WarningLevel::*;
        match self {
            ClientCannotBeBuilt => Error,
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Problem::*;
        match self {
            ClientCannotBeBuilt => write!(f, "Request client cannot be built"),
        }
    }
}
