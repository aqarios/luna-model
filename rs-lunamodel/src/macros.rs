#![allow(unused_macros)]

//! ABI-call macros ([`check!`], [`out!`], [`out_or!`]) and the argument sigils they share.
//!
//! TL;DR
//!  ┌─────────┬───────────┬─────────┬────────┬──────────┬─────────┬──────────┬──────────┬────────┐
//!  │    @    │   @mut    │   ?@    │   !    │    ?!    │    ~    │    ?~    │    ?     │ plain  │
//!  ├─────────┼───────────┼─────────┼────────┼──────────┼─────────┼──────────┼──────────┼────────┤
//!  │ handle  │ mut       │ opt     │ C      │ opt C    │ value   │ opt      │ opt      │ by     │
//!  │ ptr     │ receiver  │ handle  │ string │ string   │ conv    │ value    │ value    │ value  │
//!  │         │           │         │        │          │         │ conv     │ ptr      │        │
//!  └─────────┴───────────┴─────────┴────────┴──────────┴─────────┴──────────┴──────────┴────────┘
//!
//! Each argument to these macros may be prefixed with a sigil that says how to turn the Rust value
//! into the C-ABI argument the function expects. The sigil encodes an *operation* (take a pointer,
//! allocate a C string, convert a value, ...), not just a type. That is why inputs are marked
//! per-argument while the single out-value is converted automatically.
//!
//! # Argument sigils
//!
//! | Sigil     | Expands to                                             | Use for |
//! |-----------|--------------------------------------------------------|---------|
//! | `@x`      | `x.as_ptr()`                                           | a handle wrapper → `*const LmX` |
//! | `@mut x`  | `x.as_mut_ptr()`                                       | the in-place receiver of a mutating call → `*mut LmX` |
//! | `?@x`     | `match &x { Some(v) => v.as_ptr(), None => null() }`   | `Option<handle>` → nullable `*const LmX` |
//! | `!x`      | `(x.try_to_cstring()?).as_ptr()`                       | `&str` / `String` → `*const c_char` (allocates a NUL-terminated `CString`; fails on an interior NUL) |
//! | `?!x`     | `opt_cstring_ptr(&(x.try_to_opt_cstring()?))`          | `Option<&str/String>` → nullable `*const c_char` |
//! | `~x`      | `x.lm_into()`                                          | an infallible value conversion to the C-ABI type (e.g. `Vtype` → `LmVtype`, `Sense` → `LmSense`) |
//! | `?~x`     | `(x.map(|v| v.lm_into())).as_opt_ptr()`                | `Option<value>` needing conversion → nullable `*const LmX` (e.g. `Option<Sense>` → `const LmSense*`) |
//! | `?x`      | `x.as_opt_ptr()`                                       | `Option<value>` → nullable `*const T` (e.g. `Option<f64>` → `const double*`) |
//! | `x`       | `x` (by value)                                         | scalars, already-correct raw pointers, `&mut out` args, etc. |
//!
//! The `?`-family splits on the second character — `?` = pointer to a value, `?@` = handle,
//! `?!` = C string, `?~` = converted value — mirroring the non-optional `@` / `!` / `~`.
//!
//! ## Why `!`/`~` are separate from `@`
//!
//! `@`/`?@` borrow a pointer the wrapper already owns: infallible, nothing to keep alive. They stay
//! usable in infallible contexts such as [`out_or!`] (used in `PartialEq::eq`).
//! `!`/`?!` instead *allocate* a `CString` that must outlive the call and can fail on an interior NUL,
//! so they carry a `?` and produce an owner temporary. `~` is a plain infallible value conversion.
//! Keeping them distinct is what lets `@` remain infallible.
//!
//! ## Why the out-value needs no sigil
//!
//! There is exactly one out-value and its target is the return type, so [`out!`] converts it automatically
//! via `lm_try_into()` (identity for scalars). Inputs are many arguments needing *different* operations,
//! so each is marked.

/// Calls a LunaModel C ABI function from the installed API table and converts
/// its returned `LmStatus` into `Result<()>`.
///
/// Use this for ABI functions that return only `LmStatus` and do not write a
/// value through an out pointer. It removes the repeated `api()` lookup,
/// function-pointer call syntax, `unsafe` block, and explicit call to
/// [`crate::error::check`].
///
/// This accepts the same argument syntax as [`out!`], the only difference being
/// that there is no trailing out pointer and the result is `Result<()>`:
/// - `@arg` passes `arg.as_ptr()` (a `*const` handle pointer), identical to
///   [`out!`]. Use this for read-only wrapper handles such as `Env`,
///   `Expression`, or `VarRef`.
/// - `@mut arg` passes `arg.as_mut_ptr()` (a `*mut` handle pointer). Use this
///   for the in-place receiver of mutating ABI functions like `expr_iadd_expr`.
/// - `?arg` passes `arg.as_opt_ptr()` (a nullable `*const`): `Some(v)` becomes a
///   pointer to `v`, `None` becomes null. Use this for optional ABI arguments
///   such as `tol: Option<f64>`.
/// - plain `arg` is passed by value.
///
/// # Safety
/// The caller must ensure all pointers and buffers passed to the ABI call are
/// valid for that function. This macro centralizes the `unsafe` syntax; it does
/// not make invalid ABI calls safe.
///
/// # Example
/// ```ignore
/// // Instead of:
/// crate::error::check(unsafe {
///     (crate::api::api().expr_iadd_expr)(self.handle, rhs.as_ptr())
/// })?;
///
/// // Write:
/// check!(expr_iadd_expr(@mut self, @rhs))
/// ```
///
/// Automatically passing wrapper handles as pointers. `@mut` marks the mutable
/// in-place receiver (`.as_mut_ptr()`); `@` marks read-only handles
/// (`.as_ptr()`):
///
/// ```ignore
/// // Instead of:
/// crate::error::check(unsafe {
///     (crate::api::api().expr_iadd_expr)(self.as_mut_ptr(), rhs.as_ptr())
/// })
///
/// // Write:
/// check!(expr_iadd_expr(@mut self, @rhs))
/// ```
///
/// Mixing pointer-converted handles with by-value arguments:
///
/// ```ignore
/// check!(expr_set_offset(@mut self, value))
/// ```
macro_rules! check {
    ($func:ident ( $($arg:tt)* )) => {
        crate::error::check(unsafe { check!(@call $func, [] $($arg)*) })
    };

    (@call $func:ident, [$($built:expr,)*]) => {
        (crate::api::api().$func)($($built,)*)
    };

    (@call $func:ident, [$($built:expr,)*] @ mut $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* ($arg).as_mut_ptr(),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] @ mut $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* ($arg).as_mut_ptr(),])
    };

    (@call $func:ident, [$($built:expr,)*] @ $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* ($arg).as_ptr(),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] @ $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* ($arg).as_ptr(),])
    };

    (@call $func:ident, [$($built:expr,)*] ? @ $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* match &($arg) { Some(v) => v.as_ptr(), None => ::std::ptr::null() },] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] ? @ $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* match &($arg) { Some(v) => v.as_ptr(), None => ::std::ptr::null() },])
    };

    (@call $func:ident, [$($built:expr,)*] ? ! $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* crate::utils::opt_cstring_ptr(&(crate::traits::TryToOptCString::try_to_opt_cstring($arg)?)),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] ? ! $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* crate::utils::opt_cstring_ptr(&(crate::traits::TryToOptCString::try_to_opt_cstring($arg)?)),])
    };

    (@call $func:ident, [$($built:expr,)*] ? ~ $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&(($arg).map(|v| crate::traits::LmInto::lm_into(v)))),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] ? ~ $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&(($arg).map(|v| crate::traits::LmInto::lm_into(v)))),])
    };

    (@call $func:ident, [$($built:expr,)*] ? $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&($arg)),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] ? $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&($arg)),])
    };

    (@call $func:ident, [$($built:expr,)*] ! $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* (crate::traits::TryToCString::try_to_cstring($arg)?).as_ptr(),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] ! $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* (crate::traits::TryToCString::try_to_cstring($arg)?).as_ptr(),])
    };

    (@call $func:ident, [$($built:expr,)*] ~ $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* crate::traits::LmInto::lm_into($arg),] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] ~ $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* crate::traits::LmInto::lm_into($arg),])
    };

    (@call $func:ident, [$($built:expr,)*] $arg:expr, $($rest:tt)*) => {
        check!(@call $func, [$($built,)* $arg,] $($rest)*)
    };

    (@call $func:ident, [$($built:expr,)*] $arg:expr $(,)?) => {
        check!(@call $func, [$($built,)* $arg,])
    };
}

/// Calls a LunaModel C ABI function from the installed API table and returns its
/// direct output.
///
/// Use this for infallible ABI functions like `expr_default()` or `env_default()`
/// that return a value directly instead of `LmStatus`. It removes the repeated
/// `api()` lookup and function-pointer call syntax.
///
/// # Safety
/// The caller must ensure all arguments passed to the ABI call are valid for
/// that function.
///
/// # Example
/// ```ignore
/// // Instead of:
/// let handle = (crate::api::api().expr_default)();
///
/// // Write:
/// let handle = abi!(expr_default());
/// ```
macro_rules! abi {
    ($func:ident ( $($arg:expr),* $(,)? )) => {
        (crate::api::api().$func)($($arg),*)
    };
}

/// Calls a status-returning ABI function whose final argument is an out pointer,
/// checks the returned status, and returns the initialized out value.
///
/// Use this for ABI functions that follow the common `(..., *mut T) -> LmStatus`
/// shape. It removes the explicit `MaybeUninit`, out-pointer append, `api()`
/// lookup, `unsafe` block, and status check.
///
/// The first form returns the initialized out value as `Result<T>`. The second
/// form passes the initialized out value into a mapper closure, which should
/// itself return a `Result`.
///
/// Prefix an argument with `@` to pass `arg.as_ptr()` to the ABI function. Use
/// this for wrapper handles such as `Env`, `Expression`, or `VarRef`. Prefix an
/// argument with `?` to pass `arg.as_opt_ptr()`, mapping `Option<T>` to a
/// nullable `*const T` (`None` -> null). Unprefixed arguments are passed by
/// value.
///
/// # Safety
/// The ABI function must initialize the out pointer on `LmStatus::Ok`. The
/// caller must still ensure all non-out arguments are valid for that function.
///
/// # Example
/// ```ignore
/// // Instead of:
/// let mut out = std::mem::MaybeUninit::uninit();
/// crate::error::check(unsafe {
///     (crate::api::api().model_env)(model.as_ptr(), out.as_mut_ptr())
/// })?;
/// let env = unsafe { out.assume_init() };
///
/// // Write:
/// let env = out!(model_env(@model))?;
/// ```
///
/// Automatically passing leading wrapper handles as `.as_ptr()`:
///
/// ```ignore
/// // Instead of:
/// let mut out = std::ptr::null_mut();
/// check!(expr_new(env.as_ptr(), &mut out))?;
/// Ok(Self::from_raw(out))
///
/// // Write:
/// out!(expr_new(@env), |out| Ok(Self::from_raw(out)))
/// ```
///
/// Multiple wrapper handles:
///
/// ```ignore
/// // Instead of:
/// let same = out!(expr_eq(self.as_ptr(), other.as_ptr()))?;
///
/// // Write:
/// let same = out!(expr_eq(@self, @other))?;
/// ```
///
/// Mapping the out value before returning:
///
/// ```ignore
/// // Instead of:
/// let mut out = std::ptr::null_mut();
/// check!(expr_new(env.as_ptr(), &mut out))?;
/// Ok(Self::from_raw(out))
///
/// // Write:
/// out!(expr_new(env.as_ptr()), |out| Ok(Self::from_raw(out)))
/// ```
///
/// Mixing pointer-converted handles with by-value arguments:
///
/// ```ignore
/// let expr = out!(expr_new_constant(@env, value), |out| Ok(Self::from_raw(out)))?;
/// ```
macro_rules! out {
    ($func:ident ( $($arg:tt)* ), $map:expr $(,)?) => {{
        let mut out = std::mem::MaybeUninit::uninit();

        crate::error::check(unsafe { out!(@call $func out, [] $($arg)*) })?;

        let out = unsafe { out.assume_init() };
        ($map)(out)
    }};

    ($func:ident ( $($arg:tt)* )) => {{
        let mut out = std::mem::MaybeUninit::uninit();

        crate::error::check(unsafe { out!(@call $func out, [] $($arg)*) })?;

        crate::traits::LmTryInto::lm_try_into(unsafe { out.assume_init() })
    }};

    (@call $func:ident $out:ident, [$($built:expr,)*]) => {
        (crate::api::api().$func)($($built,)* $out.as_mut_ptr())
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ mut $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* ($arg).as_mut_ptr(),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ mut $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* ($arg).as_mut_ptr(),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* ($arg).as_ptr(),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* ($arg).as_ptr(),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? @ $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* match &($arg) { Some(v) => v.as_ptr(), None => ::std::ptr::null() },] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? @ $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* match &($arg) { Some(v) => v.as_ptr(), None => ::std::ptr::null() },])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ! $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* crate::utils::opt_cstring_ptr(&(crate::traits::TryToOptCString::try_to_opt_cstring($arg)?)),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ! $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* crate::utils::opt_cstring_ptr(&(crate::traits::TryToOptCString::try_to_opt_cstring($arg)?)),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ~ $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&(($arg).map(|v| crate::traits::LmInto::lm_into(v)))),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ~ $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&(($arg).map(|v| crate::traits::LmInto::lm_into(v)))),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&($arg)),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&($arg)),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ! $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* (crate::traits::TryToCString::try_to_cstring($arg)?).as_ptr(),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ! $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* (crate::traits::TryToCString::try_to_cstring($arg)?).as_ptr(),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ~ $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* crate::traits::LmInto::lm_into($arg),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ~ $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* crate::traits::LmInto::lm_into($arg),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] $arg:expr, $($rest:tt)*) => {
        out!(@call $func $out, [$($built,)* $arg,] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] $arg:expr $(,)?) => {
        out!(@call $func $out, [$($built,)* $arg,])
    };
}

/// Frees `$self.handle` with the given LunaModel ABI free function if it is non-null.
///
/// Use this in `Drop` impls for wrapper types that own a `handle` field.
///
/// # Safety
/// `$self.handle` must either be null or uniquely owned by the wrapper. The given
/// ABI function must be the correct deallocator for the handle type.
macro_rules! free {
    ($self:expr, $func:ident) => {{
        if !$self.handle.is_null() {
            unsafe { (crate::api::api().$func)($self.handle) };
        }
    }};
}

// Calls a status-returning ABI function with a final out pointer and returns
/// the initialized out value directly, or returns the provided fallback if the
/// ABI call fails.
///
/// Use this only where the surrounding Rust API cannot return `Result`, such as
/// trait methods like `PartialEq::eq`. Prefer `out!` everywhere a `Result` can
/// be returned.
///
/// Supports the same argument syntax as `out!`:
/// - `@arg` passes `arg.as_ptr()`
/// - plain `arg` is passed by value.
///
/// # Safety
/// The ABI function must initialize the out pointer on `LmStatus::Ok`. All
/// arguments passed to the ABI call must be valid for that function. ABI errors
/// are intentionally swallowed and replaced with the fallback.
///
/// # Example
/// ```ignore
/// impl PartialEq for Expression {
///     fn eq(&self, other: &Self) -> bool {
///         out_or!(expr_eq(@self, @other), false)
///     }
/// }
/// ```
macro_rules! out_or {
    ($func:ident ( $($arg:tt)* ), $fallback:expr $(,)?) => {{
        let mut out = std::mem::MaybeUninit::uninit();

        let status = unsafe { out_or!(@call $func out, [] $($arg)*) };

        if crate::error::check(status).is_ok() {
            unsafe { out.assume_init() }
        } else {
            $fallback
        }
    }};

    (@call $func:ident $out:ident, [$($built:expr,)*]) => {
        (crate::api::api().$func)($($built,)* $out.as_mut_ptr())
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ mut $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* ($arg).as_mut_ptr(),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ mut $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* ($arg).as_mut_ptr(),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* ($arg).as_ptr(),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] @ $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* ($arg).as_ptr(),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? @ $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* match &($arg) { Some(v) => v.as_ptr(), None => ::std::ptr::null() },] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? @ $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* match &($arg) { Some(v) => v.as_ptr(), None => ::std::ptr::null() },])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ! $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* crate::utils::opt_cstring_ptr(&(crate::traits::TryToOptCString::try_to_opt_cstring($arg)?)),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ! $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* crate::utils::opt_cstring_ptr(&(crate::traits::TryToOptCString::try_to_opt_cstring($arg)?)),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ~ $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&(($arg).map(|v| crate::traits::LmInto::lm_into(v)))),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? ~ $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&(($arg).map(|v| crate::traits::LmInto::lm_into(v)))),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&($arg)),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ? $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* crate::traits::AsOptPtr::as_opt_ptr(&($arg)),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ! $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* (crate::traits::TryToCString::try_to_cstring($arg)?).as_ptr(),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ! $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* (crate::traits::TryToCString::try_to_cstring($arg)?).as_ptr(),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ~ $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* crate::traits::LmInto::lm_into($arg),] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] ~ $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* crate::traits::LmInto::lm_into($arg),])
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] $arg:expr, $($rest:tt)*) => {
        out_or!(@call $func $out, [$($built,)* $arg,] $($rest)*)
    };

    (@call $func:ident $out:ident, [$($built:expr,)*] $arg:expr $(,)?) => {
        out_or!(@call $func $out, [$($built,)* $arg,])
    };
}
