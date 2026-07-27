"""C-API ABI compatibility checks between `luna_model` and an extension."""

from __future__ import annotations

import luna_model


def test_host_exposes_c_api_abi() -> None:
    abi = luna_model._lm._C_API_ABI_VERSION
    assert isinstance(abi, int)
    assert abi >= 1


def test_extension_import_agrees_with_host_c_api_abi(ext) -> None:
    # Initializing rs-lunamodel validates the table ABI before any call.
    from luna_model import Bounds

    value = Bounds(-1, 4)
    assert ext.roundtrip_bounds(value) == value
