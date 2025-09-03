use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uom::si::{
    length::centimeter,
    mass::kilogram,
    rational64::{Length, Mass},
};

use crate::BarKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Bar {
    weight: Mass,
    gauge: Length,
    kind: BarKind,
}

impl Bar {
    #[must_use]
    pub fn new(weight: Mass, gauge: Length, kind: BarKind) -> Self {
        Bar {
            weight,
            gauge,
            kind,
        }
    }

    #[must_use]
    pub fn weight(&self) -> Mass {
        self.weight
    }

    #[must_use]
    pub fn gauge(&self) -> Length {
        self.gauge
    }

    #[must_use]
    pub fn kind(&self) -> &BarKind {
        &self.kind
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}cm) {}kg",
            self.kind,
            self.gauge.get::<centimeter>(),
            self.weight.get::<kilogram>(),
        )
    }
}
