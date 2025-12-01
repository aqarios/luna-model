from __future__ import annotations
from typing import Protocol


class Constant: ...


class ExprIter(Protocol):
    def __next__(self) -> tuple[Constant | Linear | Quadratic | HigherOrder, float]: ...
    def __iter__(self) -> ExprIter: ...
