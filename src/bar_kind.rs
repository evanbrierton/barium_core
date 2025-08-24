use core::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum BarKind {
    Dumbbell,
    Barbell,
}

impl BarKind {
    #[must_use]
    pub(crate) fn required_similar_plates(self) -> usize {
        match self {
            BarKind::Dumbbell => 4,
            BarKind::Barbell => 2,
        }
    }
}

impl Display for BarKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarKind::Dumbbell => write!(f, "Dumbbell"),
            BarKind::Barbell => write!(f, "Barbell"),
        }
    }
}
