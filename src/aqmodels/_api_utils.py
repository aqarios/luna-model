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


def export_override(*args):  # type: ignore[inconsistentOverload]
    """Decorator used to mark symbols for export by the generator that replace/extend the actual class."""
    if len(args) == 1 and isinstance(args[0], type):
        # Used as @export
        return args[0]

    def wrapper(cls):
        return cls

    return wrapper


def dispatched(func):
    """Marks a function as dispatched to external (Rust) implementation."""
    return func


def extended_dispatch(base):
    """
    Marks a function that is dispatched to external (Rust) implementation after extending
    it with additional (preperation) functionality.
    """

    print("EXTENDED DISPATCH")

    def decorator(extender):
        print("EXTENDED DISPATCH decorator")

        def inner(*args, **kwargs):
            print("EXTENDED DISPATCH inner")
            return base(extender(*args, **kwargs))

        return inner

    return decorator
