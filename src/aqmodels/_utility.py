"""Utility module for internals."""

import sys

if sys.version_info < (3, 13):
    from warnings import warn

    class DeprecationWarning(Warning): ...  # noqa: A001

    class deprecated:  # noqa: N801
        """Indicate that a class, function or overload is deprecated.

        When this decorator is applied to an object, the type checker
        will generate a diagnostic on usage of the deprecated object.

        Usage:

            @deprecated("Use B instead")
            class A:
                pass

            @deprecated("Use g instead")
            def f():
                pass

            @overload
            @deprecated("int support is deprecated")
            def g(x: int) -> int: ...
            @overload
            def g(x: str) -> int: ...

        The warning specified by *category* will be emitted at runtime
        on use of deprecated objects. For functions, that happens on calls;
        for classes, on instantiation and on creation of subclasses.
        If the *category* is ``None``, no warning is emitted at runtime.
        The *stacklevel* determines where the
        warning is emitted. If it is ``1`` (the default), the warning
        is emitted at the direct caller of the deprecated object; if it
        is higher, it is emitted further up the stack.
        Static type checker behavior is not affected by the *category*
        and *stacklevel* arguments.

        The deprecation message passed to the decorator is saved in the
        ``__deprecated__`` attribute on the decorated object.
        If applied to an overload, the decorator
        must be after the ``@overload`` decorator for the attribute to
        exist on the overload as returned by ``get_overloads()``.

        See PEP 702 for details.

        """

        def __init__(
            self,
            message: str,
            /,
            *,
            category: type[Warning] | None = DeprecationWarning,  # noqa: PYI011,RUF100
            stacklevel: int = 1,
        ) -> None:
            if not isinstance(message, str):
                raise TypeError(  # noqa: TRY003
                    f"Expected an object of type str for 'message', not {type(message).__name__!r}"  # noqa: E501
                )
            self.message = message
            self.category = category
            self.stacklevel = stacklevel

        def __call__(self, arg, /):  # noqa: C901,ANN001,ANN204
            # Make sure the inner functions created below don't
            # retain a reference to self.
            msg = self.message
            category = self.category
            stacklevel = self.stacklevel
            if category is None:
                arg.__deprecated__ = msg
                return arg
            elif isinstance(arg, type):  # noqa: RET505
                import functools  # noqa: PLC0415
                from types import MethodType  # noqa: PLC0415

                original_new = arg.__new__

                @functools.wraps(original_new)
                def __new__(cls, /, *args, **kwargs):  # noqa: N807,ANN001,ANN202,ANN003,ANN002
                    if cls is arg:
                        warn(msg, category=category, stacklevel=stacklevel + 1)
                    if original_new is not object.__new__:
                        return original_new(cls, *args, **kwargs)
                    # Mirrors a similar check in object.__new__.
                    elif cls.__init__ is object.__init__ and (args or kwargs):  # noqa: RET505
                        raise TypeError(f"{cls.__name__}() takes no arguments")  # noqa: TRY003
                    else:
                        return original_new(cls)

                arg.__new__ = staticmethod(__new__)

                original_init_subclass = arg.__init_subclass__
                # We need slightly different behavior if __init_subclass__
                # is a bound method (likely if it was implemented in Python)
                if isinstance(original_init_subclass, MethodType):
                    original_init_subclass = original_init_subclass.__func__

                    @functools.wraps(original_init_subclass)
                    def __init_subclass__(*args, **kwargs):  # noqa: ANN002,ANN202,ANN003,N807
                        warn(msg, category=category, stacklevel=stacklevel + 1)
                        return original_init_subclass(*args, **kwargs)

                    arg.__init_subclass__ = classmethod(__init_subclass__)
                # Or otherwise, which likely means it's a builtin such as
                # object's implementation of __init_subclass__.
                else:

                    @functools.wraps(original_init_subclass)
                    def __init_subclass__(*args, **kwargs):  # noqa: ANN202,ANN002,ANN003,N807
                        warn(msg, category=category, stacklevel=stacklevel + 1)
                        return original_init_subclass(*args, **kwargs)

                    arg.__init_subclass__ = __init_subclass__

                arg.__deprecated__ = __new__.__deprecated__ = msg
                __init_subclass__.__deprecated__ = msg
                return arg
            elif callable(arg):
                import functools  # noqa: PLC0415
                import inspect  # noqa: PLC0415

                @functools.wraps(arg)
                def wrapper(*args, **kwargs):  # noqa: ANN002,ANN003,ANN202
                    warn(msg, category=category, stacklevel=stacklevel + 1)
                    return arg(*args, **kwargs)

                if inspect.iscoroutinefunction(arg):
                    wrapper = inspect.markcoroutinefunction(wrapper)

                arg.__deprecated__ = wrapper.__deprecated__ = msg
                return wrapper
            else:
                raise TypeError(  # noqa: TRY003
                    "@deprecated decorator with non-None category must be applied to "
                    f"a class or callable, not {arg!r}"
                )

else:
    from warnings import deprecated


__all__ = ["deprecated"]
