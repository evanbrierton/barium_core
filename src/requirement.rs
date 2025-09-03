use std::{fmt::Display, str::FromStr};

use rational_extensions::{MinMax, try_from_dec_str};
use uom::si::{mass::kilogram, rational64::Mass};

use uom::num_rational::Rational64;

use crate::{BarKind, Dumbbell, GymError};

#[derive(Debug, Clone, Copy)]
pub struct Requirement {
    weight: Mass,
    bar_kind: BarKind,
}

impl Requirement {
    #[must_use]
    pub fn new(weight: Mass, bar_kind: BarKind) -> Self {
        Requirement { weight, bar_kind }
    }

    #[must_use]
    pub fn matches(self, dumbbell: &Dumbbell) -> bool {
        self.weight == dumbbell.weight() && self.bar_kind == *dumbbell.bar().kind()
    }

    #[must_use]
    pub fn bar_kind(self) -> BarKind {
        self.bar_kind
    }

    #[must_use]
    pub fn weight(self) -> Mass {
        self.weight
    }

    fn from_str_with_unit(weight: &str, unit: &str, bar_kind: &str) -> Result<Self, GymError> {
        let min_max = MinMax::new(0, 10).unwrap();
        let weight: Rational64 = try_from_dec_str(weight, &min_max)
            .map_err(|_| GymError::InvalidWeight(weight.to_string()))?;
        let weight = Mass::from_str(format!("{weight} {unit}").as_str())
            .map_err(|_| GymError::InvalidWeight(weight.to_string()))?;
        let bar_kind = BarKind::from_str(bar_kind)?;

        Ok(Requirement::new(weight, bar_kind))
    }

    fn from_str_without_unit(s: &str) -> Result<Self, GymError> {
        let (weight, bar_kind) = s.split_at(s.len() - 1);
        Requirement::from_str_with_unit(weight.trim(), "kg", bar_kind.trim())
    }
}

impl Display for Requirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kg {}", self.weight.get::<kilogram>(), self.bar_kind)
    }
}

impl FromStr for Requirement {
    type Err = GymError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_whitespace()
            .map(str::to_lowercase)
            .collect::<Vec<_>>();

        match (parts.len(), parts.as_slice()) {
            (1, _) => Requirement::from_str_without_unit(s),
            (3, [weight, unit, kind]) => Requirement::from_str_with_unit(weight, unit, kind),
            _ => Err(GymError::InvalidRequirement(s.to_string())),
        }
    }
}
