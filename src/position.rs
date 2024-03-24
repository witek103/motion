use angle::Angle;
use libm::sqrt;
#[cfg(feature = "use_std")]
use std::fmt::{Display, Formatter};

use crate::Velocity;

pub type Millimeters = f64;
pub type Seconds = f64;

#[derive(Copy, Clone)]
pub struct Position<const X0: usize, const Y0: usize, const THETA0: usize> {
    pub x: Millimeters,
    pub y: Millimeters,
    pub theta: Angle,
}

impl<const X0: usize, const Y0: usize, const THETA0: usize> Position<X0, Y0, THETA0> {
    pub fn new(x: Millimeters, y: Millimeters, theta: Angle) -> Self {
        Self { x, y, theta }
    }

    pub fn distance(&self, other: &Position<X0, Y0, THETA0>) -> Millimeters {
        sqrt((other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y))
    }

    pub fn start_position() -> Self {
        let mut position = Self::new(0.0, 0.0, Angle::radians(0.0));

        position.reset();

        position
    }

    pub fn reset(&mut self) {
        self.x = X0 as f64;
        self.y = Y0 as f64;
        self.theta = Angle::degrees(THETA0 as f64);
    }

    pub fn velocity_update(&mut self, velocity: Velocity, period: Seconds) {
        self.x += velocity.translational * self.theta.cos() * period;
        self.y += velocity.translational * self.theta.sin() * period;
        self.theta = Angle::radians(self.theta.as_radians() + velocity.rotational * period);
    }
}

#[cfg(feature = "use_std")]
impl<const X0: usize, const Y0: usize, const THETA0: usize> Display for Position<X0, Y0, THETA0> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "P({}, {}, {})", self.x, self.y, self.theta)
    }
}

#[cfg(feature = "use_defmt")]
impl<const X0: usize, const Y0: usize, const THETA0: usize> defmt::Format
    for Position<X0, Y0, THETA0>
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P({}, {}, {})", self.x, self.y, self.theta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libm::fabs;

    #[test]
    fn start_position() {
        let p1 = Position::<70, 80, 90>::start_position();

        assert!(fabs(p1.x - 70.0) < 0.001);
        assert!(fabs(p1.y - 80.0) < 0.001);
        assert!(fabs(p1.theta.as_degrees() - 90.0) < 0.001);
    }

    #[test]
    fn distance() {
        type P = Position<90, 0, 90>;

        let p1 = P::start_position();
        let p2 = P::new(90.0, 0.0, Angle::degrees(90.0));
        let p3 = P::new(0.0, 0.0, Angle::degrees(90.0));
        let p4 = P::new(90.0 + 30.0, 40.0, Angle::degrees(90.0));

        assert!(fabs(p1.distance(&p2)) < 0.001);
        assert!((fabs(p1.distance(&p3)) - 90.0) < 0.001);
        assert!((fabs(p1.distance(&p4)) - 50.0) < 0.001);
    }

    #[test]
    fn velocity_update() {
        type P = Position<0, 0, 90>;

        let mut p1 = P::start_position();
        let p2 = P::new(0.0, 50.0, Angle::degrees(45.0));
        let p3 = P::new(35.355, 50.0 + 35.355, Angle::degrees(0.0));
        let p4 = P::new(35.355 + 50.0, 50.0 + 35.355, Angle::degrees(-45.0));
        let velocity = Velocity::new(50.0, Angle::degrees(-45.0).as_radians());

        p1.velocity_update(velocity, 1.0);
        assert!(fabs(p1.distance(&p2)) < 0.001);
        assert!(p1.theta.is_within(&p2.theta, Angle::degrees(0.001)));

        p1.velocity_update(velocity, 1.0);
        assert!(fabs(p1.distance(&p3)) < 0.001);
        assert!(p1.theta.is_within(&p3.theta, Angle::degrees(0.001)));

        p1.velocity_update(velocity, 1.0);
        assert!(fabs(p1.distance(&p4)) < 0.001);
        assert!(p1.theta.is_within(&p4.theta, Angle::degrees(0.001)));
    }
}
