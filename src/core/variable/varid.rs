use crate::core::expression::One;
use crate::errors::ParseFromStringError;
use crate::types::Id;
use std::fmt::Debug;
use std::{ops::AddAssign, str::FromStr};

/// The VarId is a wrapper around some primitive type used as the index in Expressions.
#[derive(Debug, Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct VarId(pub Id);

impl One for VarId {
    fn one() -> Self {
        VarId(1)
    }
}

impl AddAssign<VarId> for VarId {
    fn add_assign(&mut self, rhs: VarId) {
        self.0 += rhs.0
    }
}

impl ToString for VarId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for VarId {
    type Err = ParseFromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VarId(s.parse::<Id>()?))
    }
}

impl Into<usize> for VarId {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for VarId {
    fn from(value: usize) -> Self {
        assert!(
            value <= Id::MAX as usize,
            "value out of range for Index"
        );
        VarId(value as Id)
    }
}

impl Into<u64> for VarId {
    fn into(self) -> u64 {
        self.0 as u64
    }
}
