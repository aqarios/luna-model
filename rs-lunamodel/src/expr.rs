//! Owning wrapper over a LunaModel expression handle.
//!
//! Mirrors the Rust `Expression`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Expression` still return `Result` here (the
//! deliberate analog of `cxx-lunamodel` throwing on a non-`Ok` status).

use lunamodel_cext_vtable::{LmBuffer, LmExpr, LmVtype, LmVtypeBuffer};

use crate::api::api;
use crate::env::Env;
use crate::error::{Result, check};

/// Variable type (mirror of the Rust `Vtype`).
pub type Vtype = LmVtype;

/// Owning handle to a LunaModel expression (frees on drop).
pub struct Expression {
    handle: *mut LmExpr,
}

impl Expression {
    /// Wraps a raw handle owned by the caller.
    fn from_raw(handle: *mut LmExpr) -> Self {
        Expression { handle }
    }

    // ---- creation --------------------------------------------------------

    /// Creates an empty expression bound to `env` (Rust `Expression::new(env)`).
    pub fn new(env: &Env) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_new)(env.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// Creates a constant expression bound to `env`
    /// (Rust `Expression::constant(env, value)`).
    pub fn constant(env: &Env, value: f64) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_new_constant)(env.as_ptr(), value, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// Creates a constant expression from a float (Rust `Expression::from(f64)`).
    pub fn from_f64(value: f64) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_from_double)(value, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// Creates a constant expression from an integer
    /// (Rust `Expression::from(usize)`).
    pub fn from_usize(value: usize) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_from_size_t)(value, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    // ---- access ----------------------------------------------------------

    /// Returns a handle to the shared environment (Rust `expr.env`).
    pub fn env(&self) -> Result<Env> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_env)(self.handle, &mut out) })?;
        Ok(unsafe { Env::from_raw(out) })
    }

    /// Returns the constant offset (Rust `expr.offset`).
    pub fn offset(&self) -> Result<f64> {
        let mut out = 0.0;
        check(unsafe { (api().expr_offset)(self.handle, &mut out) })?;
        Ok(out)
    }

    /// Returns the number of distinct variables (Rust `expr.num_vars()`).
    pub fn num_vars(&self) -> Result<usize> {
        let mut out = 0;
        check(unsafe { (api().expr_num_vars)(self.handle, &mut out) })?;
        Ok(out)
    }

    /// Returns the degree of the expression (Rust `expr.degree()`).
    pub fn degree(&self) -> Result<usize> {
        let mut out = 0;
        check(unsafe { (api().expr_degree)(self.handle, &mut out) })?;
        Ok(out)
    }

    /// Returns whether the expression is a plain constant
    /// (Rust `expr.is_constant()`).
    pub fn is_constant(&self) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_is_constant)(self.handle, &mut out) })?;
        Ok(out)
    }

    /// Returns whether any quadratic term is present (Rust `expr.has_quadratic()`).
    pub fn has_quadratic(&self) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_has_quadratic)(self.handle, &mut out) })?;
        Ok(out)
    }

    /// Returns whether any higher-order term is present
    /// (Rust `expr.has_higher_order()`).
    pub fn has_higher_order(&self) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_has_higher_order)(self.handle, &mut out) })?;
        Ok(out)
    }

    /// Returns the distinct variable types used by the expression
    /// (Rust `expr.vtypes()`).
    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        let mut buf = LmVtypeBuffer {
            data: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        };
        check(unsafe { (api().expr_vtypes)(self.handle, &mut buf) })?;
        let out = if buf.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec()
        };
        unsafe { (api().vtype_buffer_free)(buf) };
        Ok(out)
    }

    // ---- arithmetic (returning a new expression) -------------------------

    /// `self + bias` (Rust `&expr + bias`).
    pub fn add_bias(&self, bias: f64) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_bias)(self.handle, bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self + other` (Rust `&expr + &expr`).
    pub fn add(&self, other: &Expression) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_expr)(self.handle, other.handle, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self - bias` (Rust `&expr - bias`).
    pub fn sub_bias(&self, bias: f64) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_bias)(self.handle, bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self - other` (Rust `&expr - &expr`).
    pub fn sub(&self, other: &Expression) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_expr)(self.handle, other.handle, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self * bias` (Rust `&expr * bias`).
    pub fn mul_bias(&self, bias: f64) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_bias)(self.handle, bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self * other` (Rust `&expr * &expr`).
    pub fn mul(&self, other: &Expression) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_expr)(self.handle, other.handle, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `-self` (Rust `-&expr`).
    pub fn neg(&self) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_neg)(self.handle, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self.pow(n)` (Rust `expr.pow(n)`).
    pub fn pow(&self, n: usize) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_pow)(self.handle, n, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    // ---- arithmetic (in place) -------------------------------------------

    /// `self += bias` (Rust `expr += bias`).
    pub fn add_assign_bias(&mut self, bias: f64) -> Result<()> {
        check(unsafe { (api().expr_iadd_bias)(self.handle, bias) })
    }

    /// `self += other` (Rust `expr += &expr`).
    pub fn add_assign(&mut self, other: &Expression) -> Result<()> {
        check(unsafe { (api().expr_iadd_expr)(self.handle, other.handle) })
    }

    /// `self -= bias` (Rust `expr -= bias`).
    pub fn sub_assign_bias(&mut self, bias: f64) -> Result<()> {
        check(unsafe { (api().expr_isub_bias)(self.handle, bias) })
    }

    /// `self -= other` (Rust `expr -= &expr`).
    pub fn sub_assign(&mut self, other: &Expression) -> Result<()> {
        check(unsafe { (api().expr_isub_expr)(self.handle, other.handle) })
    }

    /// `self *= bias` (Rust `expr *= bias`).
    pub fn mul_assign_bias(&mut self, bias: f64) -> Result<()> {
        check(unsafe { (api().expr_imul_bias)(self.handle, bias) })
    }

    /// `self *= other` (Rust `expr *= &expr`).
    pub fn mul_assign(&mut self, other: &Expression) -> Result<()> {
        check(unsafe { (api().expr_imul_expr)(self.handle, other.handle) })
    }

    /// `self = self.pow(n)`, in place (Rust `expr.pow_assign(n)`).
    pub fn pow_assign(&mut self, n: usize) -> Result<()> {
        check(unsafe { (api().expr_ipow)(self.handle, n) })
    }

    // ---- clone / equality / serialization --------------------------------

    /// Clones into a new handle sharing the environment (Rust `Clone`).
    ///
    /// Named `try_clone` because the fallible C call cannot satisfy `Clone`.
    pub fn try_clone(&self) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_clone)(self.handle, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// Deep-clones into an independent expression rebound to `env`
    /// (Rust `expr.deep_clone(env)`).
    pub fn deep_clone(&self, env: &Env) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_deep_clone)(self.handle, env.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// Content equality, ignoring environment identity
    /// (Rust `expr.equal_contents(&other)`).
    pub fn equal_contents(&self, other: &Expression) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_equal_contents)(self.handle, other.handle, &mut out) })?;
        Ok(out)
    }

    /// Serializes the expression to bytes.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buf = LmBuffer {
            data: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        };
        check(unsafe { (api().expr_serialize)(self.handle, &mut buf) })?;
        let bytes = if buf.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec()
        };
        unsafe { (api().buffer_free)(buf) };
        Ok(bytes)
    }

    /// Deserializes an expression from bytes, rebinding it to `env`.
    pub fn deserialize(data: &[u8], env: &Env) -> Result<Expression> {
        let mut out = std::ptr::null_mut();
        check(unsafe {
            (api().expr_deserialize)(data.as_ptr(), data.len(), env.as_ptr(), &mut out)
        })?;
        Ok(Self::from_raw(out))
    }

    /// Borrows the raw handle for C interop.
    pub fn as_ptr(&self) -> *const LmExpr {
        self.handle
    }
}

impl Default for Expression {
    /// Creates an empty expression with a fresh environment
    /// (Rust `Expression::default()`).
    fn default() -> Self {
        Self::from_raw((api().expr_default)())
    }
}

impl PartialEq for Expression {
    /// Structural equality including environment identity (Rust `PartialEq`).
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        let mut out = false;
        unsafe { (api().expr_eq)(self.handle, other.handle, &mut out) };
        out
    }
}

impl Drop for Expression {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // SAFETY: `handle` was produced by this library and is freed once.
            unsafe { (api().expr_free)(self.handle) };
        }
    }
}
