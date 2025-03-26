from typing import TypeVar
from typing import overload, Callable, TypeVar

T = TypeVar("T", bound=type)


@overload
def export(cls: T) -> T: ...
@overload
def export(*targets: str) -> Callable[[T], T]: ...


def export(*args):  # type: ignore[inconsistentOverload]
    """Decorator used to mark symbols for export by the generator."""
    if len(args) == 1 and isinstance(args[0], type):
        # Used as @export
        return args[0]

    def wrapper(cls):
        return cls

    return wrapper
