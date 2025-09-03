use std::{fmt::Display, hash::Hash};

use itertools::Itertools;
use uom::si::{mass::kilogram, u32::Mass};

use crate::{Bar, Plate};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]

pub struct Dumbbell {
    plates: Vec<Plate>,
    bar: Bar,
}

impl Dumbbell {
    #[must_use]
    pub fn new(plates: Vec<Plate>, bar: Bar) -> Self {
        Dumbbell {
            plates: plates
                .into_iter()
                .sorted()
                .rev()
                .filter(|p| p.gauge() == bar.gauge())
                .collect(),
            bar,
        }
    }

    #[must_use]
    pub fn plates(&self) -> &[Plate] {
        &self.plates
    }

    #[must_use]
    pub fn bar(&self) -> &Bar {
        &self.bar
    }

    #[must_use]
    pub fn weight(&self) -> Mass {
        self.bar.weight() + self.plates.iter().map(|plate| plate.weight()).sum::<Mass>() * 2
    }

    #[must_use]
    pub(crate) fn adjacent(&self, other: &Self) -> bool {
        if self.bar != other.bar {
            return false;
        }

        if self.plates.len().abs_diff(other.plates.len()) != 1 {
            return false;
        }

        self.plates
            .iter()
            .zip(&other.plates)
            .all(|(p1, p2)| p1.weight() == p2.weight())
    }
}

impl PartialOrd for Dumbbell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dumbbell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl Display for Dumbbell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plates = self
            .plates
            .iter()
            .map(|p| p.weight())
            .map(|w| w.get::<kilogram>())
            .map(|w| format!("{w}"))
            .join(", ");

        write!(f, "[{}] ({}kg)", plates, self.weight().get::<kilogram>(),)
    }
}
