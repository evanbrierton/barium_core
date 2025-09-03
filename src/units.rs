#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn kgs_to_grams(kgs: f64) -> u32 {
    (kgs * 1000.0) as u32
}
