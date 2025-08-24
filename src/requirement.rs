use std::fmt::Display;

use crate::{BarKind, Dumbbell};

#[derive(Debug, Clone, Copy)]
pub struct Requirement {
    weight: u32,
    bar_kind: BarKind,
}

impl Requirement {
    #[must_use]
    pub fn new(weight: u32, bar_kind: BarKind) -> Self {
        Requirement { weight, bar_kind }
    }

    #[must_use]
    pub fn matches(self, dumbbell: &Dumbbell) -> bool {
        self.weight == dumbbell.weight() && self.bar_kind == *dumbbell.bar().kind()
    }

    #[must_use]
    pub fn bar_kind(self) -> BarKind {
        self.bar_kind
    }

    #[must_use]
    pub fn weight(self) -> u32 {
        self.weight
    }
}

impl Display for Requirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kg {}", f64::from(self.weight) / 1000.0, self.bar_kind)
    }
}
