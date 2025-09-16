use std::collections::HashMap;

use crate::{Bar, BarKind, Dumbbell, Gym, GymError, Requirement, Workout};

impl Gym {
    /// Order-preserving shortest path workout: respects the input requirement order and
    /// minimizes total transitions between states for each `bar_kind`.
    ///
    /// # Errors
    /// Returns an error if any requirement cannot be satisfied.
    pub fn workout_ordered(&self, requirements: &[Requirement]) -> Result<Workout, GymError> {
        let mut result = HashMap::<Bar, Vec<Dumbbell>>::new();

        // Group by bar kind and solve per kind while preserving order
        let requirements_by_kind: HashMap<BarKind, Vec<Requirement>> = requirements
            .iter()
            .copied()
            .fold(HashMap::new(), |mut acc, req| {
                acc.entry(req.bar_kind()).or_default().push(req);
                acc
            });

        for (bar_kind, reqs) in requirements_by_kind {
            if reqs.is_empty() {
                continue;
            }

            let ordered = self.order_by_kind_ordered(bar_kind, &reqs)?;
            for (bar, dumbbells) in ordered {
                result.entry(bar).or_default().extend(dumbbells);
            }
        }

        Ok(result.into())
    }

    fn order_by_kind_ordered(
        &self,
        bar_kind: BarKind,
        requirements: &[Requirement],
    ) -> Result<HashMap<Bar, Vec<Dumbbell>>, GymError> {
        if requirements.is_empty() {
            return Ok(HashMap::new());
        }

        let optimal_sequence = self.find_optimal_sequence_public(bar_kind, requirements)?;

        let mut result = HashMap::<Bar, Vec<Dumbbell>>::new();
        let mut requirement_index = 0;

        for state_id in optimal_sequence {
            let state = &self.states_of(bar_kind)[&state_id];
            let req = requirements[requirement_index];
            let bars = self.bars_of(req.bar_kind());

            let mut matched = false;
            for bar in bars {
                if let Some(dumbbell) = state.get(bar)
                    && req.matches(dumbbell)
                {
                    result.entry(*bar).or_default().push(dumbbell.clone());
                    matched = true;
                }
            }

            if !matched {
                return Err(GymError::ImpossibleRequirement(req));
            }

            if requirement_index < requirements.len() - 1 {
                requirement_index += 1;
            }
        }

        Ok(result)
    }
}


