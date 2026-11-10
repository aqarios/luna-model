import importlib

import luna_model
import pytest


def _extension():
    return importlib.import_module("rs_lunamodel_integration_ext")


def pytest_collection_modifyitems(config, items):
    # The extension may only fail against a host of a different C API ABI version (luna-model-ffi
    # rejects such a host); against the same ABI every test must pass. Skip, not fail, the
    # whole session on a mismatch so that case stays distinguishable from a real regression.
    host = luna_model._lm._C_API_ABI_VERSION
    extension = _extension().C_API_ABI_VERSION
    if host != extension:
        skip = pytest.mark.skip(
            reason=f"host C API ABI {host} != extension C API ABI {extension}"
        )
        for item in items:
            item.add_marker(skip)


@pytest.fixture(scope="session")
def ext():
    return _extension()
