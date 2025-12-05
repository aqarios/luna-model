from __future__ import annotations
from typing import TYPE_CHECKING, Protocol

if TYPE_CHECKING:
    from luna_model.constraint.constr import Constraint


class ConstraintCollectionIter(Protocol):
    def __next__(self) -> tuple[str, Constraint]: ...
    def __iter__(self) -> ConstraintCollectionIter: ...
