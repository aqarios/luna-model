"""LunaModel C API helpers for building native extensions against the wheel."""

import os


def get_include() -> str:
    """Path to the bundled C API headers (`luna_model.h` + `luna_model/`).

    Pass to a C extension build, e.g.::

        import luna_model.capi

        Extension(..., include_dirs=[luna_model.capi.get_include()])
    """
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), "include")
