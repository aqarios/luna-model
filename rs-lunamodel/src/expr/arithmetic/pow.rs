use crate::{Expression, Result, api::api, error::check};

impl Expression {
    /// `self.pow(n)` (Rust core expression exponentiation).
    pub fn pow(&self, n: usize) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_pow)(self.as_ptr(), n, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self = self.pow(n)`, in place.
    pub fn pow_assign(&mut self, n: usize) -> Result<()> {
        check(unsafe { (api().expr_ipow)(self.handle, n) })
    }
}
