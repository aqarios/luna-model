from __future__ import annotations
from typing import Protocol

from luna_model.variable import Variable


class Constant(Protocol): ...


class Linear(Protocol):
    __match_args__ = ("var",)

    @property
    def var(self) -> Variable: ...


class Quadratic(Protocol):
    __match_args__ = ("var_a", "var_b")

    @property
    def var_a(self) -> Variable: ...
    @property
    def var_b(self) -> Variable: ...


class HigherOrder(Protocol):
    __match_args__ = ("vars",)

    @property
    def vars(self) -> list[Variable]: ...


class ExprIter(Protocol):
    def __next__(self) -> tuple[Constant | Linear | Quadratic | HigherOrder, float]: ...
    def __iter__(self) -> ExprIter: ...
