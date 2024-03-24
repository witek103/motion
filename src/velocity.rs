#[cfg(feature = "use_std")]
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Velocity {
    pub translational: f64,
    pub rotational: f64,
}

impl Velocity {
    pub fn zero() -> Self {
        Self {
            translational: 0.0,
            rotational: 0.0,
        }
    }

    pub fn new(translational: f64, rotational: f64) -> Self {
        Self {
            translational,
            rotational,
        }
    }
}

#[cfg(feature = "use_std")]
impl Display for Velocity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "V({}, {})", self.translational, self.rotational,)
    }
}

#[cfg(feature = "use_defmt")]
impl defmt::Format for Velocity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "V({}, {})", self.translational, self.rotational,);
    }
}
