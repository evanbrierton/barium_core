use std::{fmt::Display, hash::Hash};

use itertools::Itertools;
use uom::si::rational64::Mass;

use crate::{Bar, Plate, format};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]

pub struct Dumbbell {
    plates: Vec<Plate>,
    bar: Bar,
    weight: Mass,
}

impl Dumbbell {
    #[must_use]
    pub fn new(plates: Vec<Plate>, bar: Bar) -> Self {
        let plates_weight = plates.iter().map(|plate| plate.weight()).sum();

        Dumbbell {
            plates,
            bar,
            weight: bar.weight() + plates_weight + plates_weight,
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
    pub fn weight(&self) -> &Mass {
        &self.weight
    }

    #[must_use]
    pub(crate) fn adjacent(&self, other: &Self) -> bool {
        if self.bar != other.bar {
            return false;
        }

        if self.plates.len().abs_diff(other.plates.len()) != 1 {
            return false;
        }

        let longer = if self.plates.len() > other.plates.len() {
            self
        } else {
            other
        };

        let last_plate = longer.plates.last().unwrap();

        (*self.weight() - *other.weight()).abs() == last_plate.weight() + last_plate.weight()
    }
}

impl PartialOrd for Dumbbell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dumbbell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().cmp(other.weight())
    }
}

impl Display for Dumbbell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plates = self
            .plates
            .iter()
            .map(|p| p.weight())
            .map(format::mass_to_dec_string)
            .join(", ");

        write!(
            f,
            "[{}] ({}kg)",
            plates,
            format::mass_to_dec_string(*self.weight())
        )
    }
}
