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


# def wrap_from_dimod_sample_set(f):
#     @functools.wraps(SampleSetTranslator.from_dimod_sample_set)
#     def inner(sample_set: SampleSet, timing: Timing | None = None) -> Solution:
#         sample_set = sample_set.aggregate()
#         record = sample_set.record
#         sample = record.sample.astype(np.int64, order="C")
#         num_occurrences = record.num_occurrences.astype(np.int64, order="C")
#
#         return f(sample, num_occurrences, timing)
#
#     return inner
#
#
# SampleSetTranslator.from_dimod_sample_set = wrap_from_dimod_sample_set(
#     SampleSetTranslator.from_dimod_sample_set
# )
