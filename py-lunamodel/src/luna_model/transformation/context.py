from __future__ import annotations

from luna_model._lm import PyPassContext


class PassContext:
    """The pass context provides access to the analysis results in Transformation and analysis passes."""

    _c: PyPassContext

    @classmethod
    def _from_pyctx(cls, py_ctx: PyPassContext) -> PassContext:
        ctx = cls.__new__(cls)
        ctx._c = py_ctx
        return ctx
