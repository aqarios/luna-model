"""Checks that the extension can *interpret* host objects, not just pass them on.

The `roundtrip_*` functions only clone the `Arc` out of the capsule and hand it
back, so they never read the host's memory and cannot detect a layout mismatch
between the host `luna_model` and the `luna-model` the extension was built with.
These probes read and compute on the data in Rust.
"""

from __future__ import annotations

import pytest

from luna_model import Model, Vtype


@pytest.fixture
def hubo():
    model = Model("hubo")
    x, y, z, w = (model.add_variable(n, vtype=Vtype.BINARY) for n in "xyzw")
    return x * y * z + 2 * x * y * z * w + x


def test_read_higher_order_items(ext, hubo):
    assert ext.expression_higher_order_items(hubo) == [([0, 1, 2], 1.0), ([0, 1, 2, 3], 2.0)]


def test_lookup_higher_order_bias(ext, hubo):
    assert ext.expression_higher_order_bias(hubo, [0, 1, 2]) == 1.0
    assert ext.expression_higher_order_bias(hubo, [0, 1, 2, 3]) == 2.0


def test_compute_on_higher_order(ext, hubo):
    assert ext.expression_scaled_higher_order_items(hubo, 3.0) == [
        ([0, 1, 2], 3.0),
        ([0, 1, 2, 3], 6.0),
    ]
