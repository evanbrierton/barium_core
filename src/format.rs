use rational_extensions::to_dec_string;
use uom::si::{
    length::centimeter,
    mass::kilogram,
    rational64::{Length, Mass},
};

pub fn mass_to_dec_string(mass: Mass) -> String {
    to_dec_string(&mass.get::<kilogram>(), 10)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn length_to_dec_string(length: Length) -> String {
    to_dec_string(&length.get::<centimeter>(), 10)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
