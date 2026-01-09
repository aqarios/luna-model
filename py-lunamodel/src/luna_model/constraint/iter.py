from __future__ import annotations
from typing import TYPE_CHECKING
from luna_model._lm import PyConstraintCollectionIterator
from luna_model._utils import wrap_c

if TYPE_CHECKING:
    from luna_model.constraint.constr import Constraint


class ConstraintCollectionIter:
    _i: PyConstraintCollectionIterator

    def __next__(self) -> tuple[str, Constraint]:
        name, c = self._i.__next__()
        return name, wrap_c(c)

    def __iter__(self) -> ConstraintCollectionIter:
        return self

    @classmethod
    def _from_pycci(
        cls, py_cci: PyConstraintCollectionIterator
    ) -> ConstraintCollectionIter:
        """Construct LunaModel ConstraintCollectionIter from FFI PyConstraintCollectionIterator object."""
        i = cls.__new__(cls)
        i._i = py_cci
        return i
