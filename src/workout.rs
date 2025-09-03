use std::collections::HashMap;

use derive_more::{From, IntoIterator};

use crate::{bar::Bar, dumbbell::Dumbbell};

#[derive(Default, IntoIterator, From)]
#[into_iterator(owned, ref, ref_mut)]
pub struct Workout(pub HashMap<Bar, Vec<Dumbbell>>);

impl Workout {
    #[must_use]
    pub fn bars(&self) -> Vec<Bar> {
        self.0.keys().copied().collect()
    }

    #[must_use]
    pub fn get(&self, bar: Bar) -> Vec<Dumbbell> {
        self.0.get(&bar).cloned().unwrap_or_default()
    }
}
