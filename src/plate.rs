use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uom::si::{
    length::centimeter,
    mass::kilogram,
    rational64::{Length, Mass},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Plate {
    weight: Mass,
    gauge: Length,
}

impl Plate {
    #[must_use]
    pub fn new(weight: Mass, gauge: Length) -> Self {
        Plate { weight, gauge }
    }

    #[must_use]
    pub fn weight(self) -> Mass {
        self.weight
    }

    #[must_use]
    pub fn gauge(self) -> Length {
        self.gauge
    }
}

impl Display for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}cm) {}kg",
            self.gauge.get::<centimeter>(),
            self.weight.get::<kilogram>(),
        )
    }
}
