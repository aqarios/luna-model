use crate::{Expression, Result};

impl Expression {
    /// `self.pow(n)` (Rust core expression exponentiation).
    pub fn pow(&self, n: usize) -> Result<Self> {
        out!(expr_pow(@self, n), |o| Ok(Self::from_raw(o)))
    }

    /// `self = self.pow(n)`, in place.
    pub fn pow_assign(&mut self, n: usize) -> Result<()> {
        check!(expr_ipow(@mut self, n))
    }
}
