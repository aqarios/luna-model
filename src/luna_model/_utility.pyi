import sys

if sys.version_info < (3, 13):  # noqa: PYI066
    class DeprecationWarning(Warning): ...  # noqa: A001

    class deprecated:  # noqa: N801
        def __init__(
            self,
            message: str,
            /,
            *,
            category: type[Warning] | None = ...,
            stacklevel: int = 1,
        ) -> None: ...
        def __call__(self, arg, /): ...  # noqa: ANN001,ANN204

else:
    from warnings import deprecated

__all__ = ["deprecated"]
