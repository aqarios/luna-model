from luna_model._lm import PyConstraintCollection

from luna_model.constraint.constr import Constraint
from luna_model.constraint.iter import ConstraintCollectionIter


class ConstraintCollection:
    """ """

    _cc: PyConstraintCollection

    def __init__(self) -> None:
        self._cc = PyConstraintCollection()

    def add_constraint(self, constraint: Constraint, name: str):
        self._cc.add_constraint(constraint._c, name)  # type: ignore

    def items(self) -> ConstraintCollectionIter:
        return self._cc.items()
