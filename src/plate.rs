use derive_more::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
#[display("{weight} ({gauge})")]
pub struct Plate {
    weight: u32,
    gauge: u32,
}

impl Plate {
    #[must_use]
    pub fn new(weight: u32, gauge: u32) -> Self {
        Plate { weight, gauge }
    }

    #[must_use]
    pub fn weight(self) -> u32 {
        self.weight
    }

    #[must_use]
    pub fn gauge(self) -> u32 {
        self.gauge
    }
}
