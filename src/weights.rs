use std::collections::HashMap;

use crate::BarKind;

#[derive(Clone)]
pub struct Weights {
    weights: HashMap<BarKind, Vec<u32>>,
}

impl Weights {
    #[must_use]
    pub fn new(weights: HashMap<BarKind, Vec<u32>>) -> Self {
        Weights { weights }
    }

    #[must_use]
    pub fn get(&self, kind: BarKind) -> Vec<u32> {
        self.weights.get(&kind).cloned().unwrap_or_default()
    }
}

impl IntoIterator for Weights {
    type Item = (BarKind, Vec<u32>);
    type IntoIter = std::collections::hash_map::IntoIter<BarKind, Vec<u32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.weights.into_iter()
    }
}
