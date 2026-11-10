use std::fmt::{Display, Formatter};

use crate::{BinaryAssignment, IntegerAssignment, RealAssignment, SpinAssignment, Vtype};

/// Type-erased assignment value shared by all LunaModel frontends.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Assignment {
    Binary(BinaryAssignment),
    Spin(SpinAssignment),
    Integer(IntegerAssignment),
    Real(RealAssignment),
}

impl Assignment {
    pub fn value(self) -> f64 {
        match self {
            Self::Binary(value) => value as f64,
            Self::Spin(value) => value as f64,
            Self::Integer(value) => value as f64,
            Self::Real(value) => value,
        }
    }

    /// Converts a numeric ABI value without truncation or saturation.
    pub fn try_from_value(vtype: Vtype, value: f64) -> Option<Self> {
        match vtype {
            Vtype::Binary if value == 0.0 || value == 1.0 => Some(Self::Binary(value as u8)),
            Vtype::Spin if value == -1.0 || value == 1.0 => Some(Self::Spin(value as i8)),
            Vtype::Integer
                if value.fract() == 0.0
                    && value >= i64::MIN as f64
                    && value < -(i64::MIN as f64) =>
            {
                Some(Self::Integer(value as i64))
            }
            Vtype::Real => Some(Self::Real(value)),
            _ => None,
        }
    }
}

impl From<u8> for Assignment {
    fn from(value: u8) -> Self {
        Self::Binary(value)
    }
}

impl From<i8> for Assignment {
    fn from(value: i8) -> Self {
        Self::Spin(value)
    }
}

impl From<i64> for Assignment {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Assignment {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(value) => write!(f, "{value}"),
            Self::Spin(value) => write!(f, "{value}"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Real(value) => {
                let value = format!("{value:.}");
                if value.contains('.') {
                    write!(f, "{value}")
                } else {
                    write!(f, "{value}.0")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_conversion_is_exact() {
        assert_eq!(
            Assignment::try_from_value(Vtype::Binary, 1.0),
            Some(Assignment::Binary(1))
        );
        assert_eq!(Assignment::try_from_value(Vtype::Binary, 0.5), None);
        assert_eq!(Assignment::try_from_value(Vtype::Spin, 0.0), None);
        assert_eq!(
            Assignment::try_from_value(Vtype::Integer, -2.0),
            Some(Assignment::Integer(-2))
        );
        assert_eq!(Assignment::try_from_value(Vtype::Integer, 1.5), None);
        assert_eq!(
            Assignment::try_from_value(Vtype::Integer, -(2_f64.powi(63))),
            Some(Assignment::Integer(i64::MIN))
        );
        assert_eq!(
            Assignment::try_from_value(Vtype::Integer, 2_f64.powi(63)),
            None
        );
        assert_eq!(
            Assignment::try_from_value(Vtype::Real, 1.5),
            Some(Assignment::Real(1.5))
        );
    }
}
