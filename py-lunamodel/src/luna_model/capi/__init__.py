"""LunaModel C API helpers for building native extensions against the wheel."""

import os


def get_include() -> str:
    """Path to the bundled C API headers (`luna_model.h` + `luna_model/`).

    Pass to a C extension build, e.g.::

        import luna_model.capi

        Extension(..., include_dirs=[luna_model.capi.get_include()])
    """
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), "include")


def get_proto() -> str:
    """Path to the bundled protobuf schemas (`luna_model/*.proto`).

    These define the wire format of built-in analysis results, so a plugin in any language can
    generate a binding and consume them across the C-ABI. Use as a ``protoc`` include root, e.g.::

        import luna_model.capi

        # protoc -I <get_proto()> --python_out=. --pyi_out=. luna_model/plugin.proto
        subprocess.run(
            [
                "protoc",
                f"-I{luna_model.capi.get_proto()}",
                "--python_out=.",
                "--pyi_out=.",
                "luna_model/plugin.proto",
            ]
        )
    """
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), "proto")
