/// Angle units
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AngleUnits {
    /// Degrees
    Deg,
    /// Radians
    Rad,
}

/// Angle value with units
#[derive(Clone, Copy, Debug)]
pub struct Angle {
    value: f64,
    units: AngleUnits,
}

const RAD_TO_DEG: f64 = 180.0 / core::f64::consts::PI;
const DEG_TO_RAD: f64 = core::f64::consts::PI / 180.0;

impl Angle {
    pub fn new(value: f64, units: AngleUnits) -> Self {
        Self { value, units }
    }

    pub fn to(self, units: AngleUnits) -> Self {
        if self.units == units {
            self
        } else {
            match units {
                AngleUnits::Deg => Self::new(self.value * RAD_TO_DEG, units),
                AngleUnits::Rad => Self::new(self.value * DEG_TO_RAD, units),
            }
        }
    }

    pub fn raw(&self) -> &f64 {
        &self.value
    }
}

impl AsRef<Angle> for Angle {
    fn as_ref(&self) -> &Angle {
        self
    }
}
