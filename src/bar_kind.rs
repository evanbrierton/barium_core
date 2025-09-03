use std::str::FromStr;

use derive_more::From;
use serde::{Deserialize, Serialize};
use strum::{Display};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, From, Display, Serialize, Deserialize)]
pub enum BarKind {
    Dumbbell,
    Barbell,
}

impl BarKind {
    #[must_use]
    pub(crate) fn required_similar_plates(self) -> usize {
        match self {
            BarKind::Dumbbell => 4,
            BarKind::Barbell => 2,
        }
    }
}

impl FromStr for BarKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" | "db" | "dumbbell" => Ok(BarKind::Dumbbell),
            "b" | "bb" | "barbell" => Ok(BarKind::Barbell),
            _ => Err("Invalid bar kind".to_string()),
        }
    }
}
