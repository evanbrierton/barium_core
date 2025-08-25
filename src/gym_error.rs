use derive_more::From;
use thiserror::Error;

use crate::Requirement;

#[derive(Error, Debug, From)]
pub enum GymError {
    #[error("Cannot construct {0} with available plates and bars.")]
    InvalidRequirement(Requirement),
}
