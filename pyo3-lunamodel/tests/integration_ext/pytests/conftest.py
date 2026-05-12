import importlib

import pytest


@pytest.fixture(scope="session")
def ext():
    return importlib.import_module("pyo3_lunamodel_integration_ext")
