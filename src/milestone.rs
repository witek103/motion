#[cfg(feature = "use_std")]
use std::fmt::{Display, Formatter};

use crate::Position;

#[derive(Copy, Clone)]
pub struct Milestone<const X0: usize, const Y0: usize, const THETA0: usize> {
    pub target_position: Position<X0, Y0, THETA0>,
    pub starting_velocity_translational: f64,
    pub target_velocity_translational: f64,
    pub max_velocity_translational: f64,
}

impl<const X0: usize, const Y0: usize, const THETA0: usize> Milestone<X0, Y0, THETA0> {
    pub fn new(
        target_position: Position<X0, Y0, THETA0>,
        starting_velocity_translational: f64,
        target_velocity_translational: f64,
        max_velocity_translational: f64,
    ) -> Self {
        Self {
            target_position,
            starting_velocity_translational,
            target_velocity_translational,
            max_velocity_translational,
        }
    }
}

#[cfg(feature = "use_std")]
impl<const X0: usize, const Y0: usize, const THETA0: usize> Display for Milestone<X0, Y0, THETA0> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "M(t{}, Vs({}), Vt({}), Vm({}))",
            self.target_position,
            self.starting_velocity_translational,
            self.target_velocity_translational,
            self.max_velocity_translational
        )
    }
}

#[cfg(feature = "use_defmt")]
impl<const X0: usize, const Y0: usize, const THETA0: usize> defmt::Format
    for Milestone<X0, Y0, THETA0>
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "M(t{}, Vs({}), Vt({}), Vm({}))",
            self.target_position,
            self.starting_velocity_translational,
            self.target_velocity_translational,
            self.max_velocity_translational
        )
    }
}
