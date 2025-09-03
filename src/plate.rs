
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Plate {
    weight: u32,
    gauge: u32,
}

impl Plate {
    #[must_use]
    pub fn new(weight: u32, gauge: u32) -> Self {
        Plate { weight, gauge }
    }

    #[must_use]
    pub fn weight(self) -> u32 {
        self.weight
    }

    #[must_use]
    pub fn gauge(self) -> u32 {
        self.gauge
    }
}

impl Display for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) {}kg",
            self.gauge,
            f64::from(self.weight) / 1000.0,
        )
    }
}
