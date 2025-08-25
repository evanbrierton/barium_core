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
