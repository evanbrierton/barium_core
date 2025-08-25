use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::BarKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[display("{kind} ({gauge})")]
pub struct Bar {
    weight: u32,
    gauge: u32,
    kind: BarKind,
}

impl Bar {
    #[must_use]
    pub fn new(weight: u32, gauge: u32, kind: BarKind) -> Self {
        Bar {
            weight,
            gauge,
            kind,
        }
    }

    #[must_use]
    pub fn weight(&self) -> u32 {
        self.weight
    }

    #[must_use]
    pub fn gauge(&self) -> u32 {
        self.gauge
    }

    #[must_use]
    pub fn kind(&self) -> &BarKind {
        &self.kind
    }
}
