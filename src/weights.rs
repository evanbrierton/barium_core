use std::collections::HashMap;

use derive_more::{From, IntoIterator};

use crate::BarKind;

#[derive(Clone, IntoIterator, From)]
pub struct Weights(pub HashMap<BarKind, Vec<u32>>);

impl Weights {
    #[must_use]
    pub fn get(&self, kind: BarKind) -> Vec<u32> {
        self.0.get(&kind).cloned().unwrap_or_default()
    }
}
