from __future__ import annotations
from typing import TYPE_CHECKING, Protocol, TypeAlias, runtime_checkable

from luna_model._lm import PyConstant

if TYPE_CHECKING:
    from luna_model.variable.var import Variable


Constant: TypeAlias = PyConstant


@runtime_checkable
class Linear(Protocol):
    __match_args__ = ("var",)

    @property
    def var(self) -> Variable: ...


@runtime_checkable
class Quadratic(Protocol):
    __match_args__ = ("var_a", "var_b")

    @property
    def var_a(self) -> Variable: ...
    @property
    def var_b(self) -> Variable: ...


@runtime_checkable
class HigherOrder(Protocol):
    __match_args__ = ("vars",)

    @property
    def vars(self) -> list[Variable]: ...


class ExprIter(Protocol):
    def __next__(self) -> tuple[Constant | Linear | Quadratic | HigherOrder, float]: ...
    def __iter__(self) -> ExprIter: ...
