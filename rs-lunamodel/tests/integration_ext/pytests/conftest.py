import importlib
from importlib import metadata

import luna_model
import pytest
from packaging.version import Version


def _extension():
    return importlib.import_module("rs_lunamodel_integration_ext")


def _published_alpha():
    """The version of the host if it is an alpha installed from an index, else `None`."""
    host = metadata.distribution("luna-model")
    # Only installs from a path or URL have a `direct_url.json` (PEP 610).
    published = host.read_text("direct_url.json") is None
    pre = Version(host.version).pre
    return host.version if published and pre is not None and pre[0] == "a" else None


def pytest_collection_modifyitems(config, items):
    # The extension may only fail against a host of a different C API ABI version (luna-model-ffi
    # rejects such a host) or against a published alpha, which gives no ABI guarantee (see
    # ci/tools/check-abi.py); against any other host every test must pass. Skip, not fail, the
    # whole session in those cases so they stay distinguishable from a real regression.
    host = luna_model._lm._C_API_ABI_VERSION
    extension = _extension().C_API_ABI_VERSION
    alpha = _published_alpha()
    if host != extension:
        reason = f"host C API ABI {host} != extension C API ABI {extension}"
    elif alpha is not None:
        reason = f"host {alpha} is an alpha: no C API ABI guarantee"
    else:
        return
    skip = pytest.mark.skip(reason=reason)
    for item in items:
        item.add_marker(skip)


@pytest.fixture(scope="session")
def ext():
    return _extension()
