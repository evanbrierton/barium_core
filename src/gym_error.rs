use thiserror::Error;

use crate::Requirement;

#[derive(Error, Debug)]
pub enum GymError {
    #[error("Cannot construct {0} with available plates and bars.")]
    ImpossibleRequirement(Requirement),

    #[error("Invalid requirement: {0}")]
    InvalidRequirement(String),

    #[error("Invalid weight: {0} - must be of the format <number>(d | b).")]
    InvalidWeight(String),

    #[error("Unknown bar kind: {0} - must be one of: d, b.")]
    InvalidBarKind(String),
}
