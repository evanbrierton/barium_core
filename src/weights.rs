use std::collections::HashMap;

use derive_more::{From, IntoIterator};
use uom::si::u32::Mass;

use crate::BarKind;

#[derive(Clone, IntoIterator, From)]
#[into_iterator(owned, ref, ref_mut)]
pub struct Weights(pub HashMap<BarKind, Vec<Mass>>);

impl Weights {
    #[must_use]
    pub fn get(&self, kind: BarKind) -> Vec<Mass> {
        self.0.get(&kind).cloned().unwrap_or_default()
    }
}
