//! The plugin-side mirror of the transpiler's `PassContext`.

use std::ffi::{CString, c_void};

use lunamodel_capi::transpiler::LmPassContext;

use crate::error::{Error, Status};

use super::AnalysisResult;

/// Read-only per-pass execution context: the analyses a pass may read during [`forward`].
///
/// A 1:1 mirror of `lunamodel_transpiler::PassContext` — a read-only view over the run's analysis
/// store. It is populated by the host as analyses run (never by a pass), and lets a pass read
/// results computed earlier in the same run, including ones a *different* plugin's analysis produced
/// (see [`require_analysis`]). It only borrows the host's context for the duration of the call.
///
/// [`forward`]: super::TransformationPass::forward
/// [`require_analysis`]: Self::require_analysis
pub struct PassContext {
    raw: *const LmPassContext,
}

// SAFETY: the borrowed context is used transiently within one `forward` call, and a detached one
// holds only a null pointer; neither is shared across threads while pointing at live host state.
unsafe impl Send for PassContext {}
unsafe impl Sync for PassContext {}

impl PassContext {
    /// Wraps a context pointer lent by the host for the duration of a call (does not own it).
    pub(crate) fn from_raw(raw: *const LmPassContext) -> Self {
        Self { raw }
    }

    /// Creates a detached context with no analyses.
    ///
    /// Every lookup reports the result as absent. Used as the default when no host context is
    /// supplied (e.g. calling `forward` by hand outside a run).
    pub fn detached() -> Self {
        Self {
            raw: std::ptr::null(),
        }
    }

    /// Reads an analysis result deposited under `name` earlier in the same run, decoding it into
    /// `R`, or `None` if no such result was computed.
    ///
    /// The host verifies the stored result's type against `R` before decoding, so a name that
    /// resolves to a different result type is an error rather than a silent mis-decode.
    ///
    /// ```ignore
    /// let widths: Option<CoefficientWidths> = ctx.get_analysis("myplugin::coeff-widths")?;
    /// ```
    pub fn get_analysis<R: AnalysisResult>(&self, name: &str) -> crate::Result<Option<R>> {
        if self.raw.is_null() {
            return Ok(None);
        }

        let name_c = CString::new(name).map_err(|_| interior_nul("name"))?;
        let tag_c = CString::new(R::static_type_tag()).map_err(|_| interior_nul("type tag"))?;

        let mut buf: Vec<u8> = Vec::new();
        let status = unsafe {
            (crate::api::api().pass_context_get_analysis)(
                self.raw,
                name_c.as_ptr(),
                tag_c.as_ptr(),
                collect_bytes,
                (&mut buf as *mut Vec<u8>).cast::<c_void>(),
            )
        };

        if !crate::error::present(status)? {
            return Ok(None);
        }
        R::deserialize(&buf).map(Some)
    }

    /// Reads an analysis result deposited under `name`, decoding it into `R`, erroring if no such
    /// result was computed.
    ///
    /// Use this when the pass listed `name` in [`requires`](super::TransformationPass::requires):
    /// the host guarantees the analysis ran, so its absence is a real failure.
    ///
    /// ```ignore
    /// let widths: CoefficientWidths = ctx.require_analysis("myplugin::coeff-widths")?;
    /// ```
    pub fn require_analysis<R: AnalysisResult>(&self, name: &str) -> crate::Result<R> {
        self.get_analysis::<R>(name)?.ok_or_else(|| {
            Error::new(
                Status::Absent,
                format!("required analysis '{name}' was not computed"),
            )
        })
    }
}

impl Default for PassContext {
    fn default() -> Self {
        Self::detached()
    }
}

fn interior_nul(what: &str) -> Error {
    Error::new(
        Status::InvalidArgument,
        format!("analysis {what} contains an interior NUL byte"),
    )
}

/// `LmByteSink`: appends the host-provided analysis bytes into a `Vec<u8>`.
unsafe extern "C" fn collect_bytes(ctx: *mut c_void, data: *const u8, len: usize) {
    if ctx.is_null() || data.is_null() || len == 0 {
        return;
    }
    let out = unsafe { &mut *(ctx as *mut Vec<u8>) };
    out.extend_from_slice(unsafe { std::slice::from_raw_parts(data, len) });
}
