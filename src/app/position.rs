#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum PositionArea {
    #[default]
    UpperLeft,
    LowerLeft,
    UpperRight,
    LowerRight,
}

impl PositionArea {
    pub fn left(&self) -> Option<Self> {
        match self {
            Self::UpperRight => Some(Self::UpperLeft),
            Self::LowerRight => Some(Self::LowerLeft),
            _ => None,
        }
    }

    pub fn left_limited(&self) -> Self {
        self.left().unwrap_or(*self)
    }

    pub fn right(&self) -> Option<Self> {
        match self {
            Self::UpperLeft => Some(Self::UpperRight),
            Self::LowerLeft => Some(Self::LowerRight),
            _ => None,
        }
    }

    pub fn right_limited(&self) -> Self {
        self.right().unwrap_or(*self)
    }

    pub fn up(&self) -> Option<Self> {
        match self {
            Self::LowerLeft => Some(Self::UpperLeft),
            Self::LowerRight => Some(Self::UpperRight),
            _ => None,
        }
    }

    pub fn up_limited(&self) -> Self {
        self.up().unwrap_or(*self)
    }

    pub fn down(&self) -> Option<Self> {
        match self {
            Self::UpperLeft => Some(Self::LowerLeft),
            Self::UpperRight => Some(Self::LowerRight),
            _ => None,
        }
    }

    pub fn down_limited(&self) -> Self {
        self.down().unwrap_or(*self)
    }
}
