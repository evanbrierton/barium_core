#![warn(clippy::pedantic)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::ptr_as_ptr)]

mod gym_state;
mod bar;
mod bar_kind;
mod dumbbell;
mod gym;
mod gym_error;
mod plate;
mod requirement;
mod weights;
mod workout;

pub(crate) use gym_state::GymState;
pub(crate) use gym_state::GymStateId;

pub use bar::Bar;
pub use bar_kind::BarKind;
pub use dumbbell::Dumbbell;
pub use gym::Gym;
pub use gym_error::GymError;
pub use plate::Plate;
pub use requirement::Requirement;
pub use weights::Weights;
pub use workout::Workout;
