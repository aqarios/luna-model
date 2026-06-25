"""Capsule ABI compatibility checks between `luna_model` and an extension.

The extension and host exchange Rust objects through raw-pointer capsules whose
layout is only sound when both were built against a compatible `luna-model` and
`pyo3`. The host advertises its capsule ABI as `luna_model._lm.__capsule_abi__`,
and extensions verify it at import; see `crates/python/src/ffi/abi.rs`.
"""

from __future__ import annotations

import luna_model


def test_host_exposes_capsule_abi() -> None:
    abi = luna_model._lm.__capsule_abi__
    assert isinstance(abi, int)
    assert abi >= 1


def test_extension_import_agrees_with_host_abi(ext) -> None:
    # Importing the extension and crossing a capsule only succeeds when the
    # extension's compiled-in ABI matches the host's. A mismatch raises
    # ImportError at the first crossing instead of segfaulting.
    from luna_model import Bounds

    value = Bounds(-1, 4)
    assert ext.roundtrip_bounds(value) == value
