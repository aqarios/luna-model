from __future__ import annotations
from typing import TYPE_CHECKING, Self

from luna_model.constraint.constr import Constraint
from luna_model._utils import wrap_c
from luna_model._lm import PyConstraintCollection

if TYPE_CHECKING:
    from luna_model.environment.environment import Environment
    from luna_model.constraint.cmp import Comparator
    from luna_model.constraint.iter import ConstraintCollectionIter


class ConstraintCollection:
    _cc: PyConstraintCollection

    def __init__(self) -> None:
        self._cc = PyConstraintCollection()

    @classmethod
    def _from_pycc(cls, py_cc: PyConstraintCollection) -> ConstraintCollection:
        """Construct LunaModel ConstraintCollection from FFI PyConstraintCollection object."""
        cc = cls.__new__(cls)
        cc._cc = py_cc
        return cc

    def add_constraint(self, constraint: Constraint, name: str | None = None):
        self._cc.add_constraint(constraint._c, name)  # type: ignore

    def items(self) -> ConstraintCollectionIter:
        return self._cc.items()

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        return self._cc.encode(compress, level)

    def serialize(
        self, /, compress: bool | None = True, level: int | None = 3
    ) -> bytes:
        return self.encode(compress, level)

    def get(self, item: str) -> Constraint:
        return wrap_c(self._cc.get(item))

    def remove(self, item: str) -> None:
        self._cc.remove(item)

    def equal_contents(self, other: ConstraintCollection) -> bool:
        return self._cc.equal_contents(other._cc)

    def ctypes(self) -> list[Comparator]:
        return [Comparator(c) for c in self._cc.ctypes()]

    @classmethod
    def decode(cls, data: bytes, env: Environment) -> ConstraintCollection:
        return cls._from_pycc(PyConstraintCollection.decode(data, env._env))

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> ConstraintCollection:
        return cls.decode(data, env)

    def __iadd__(self, other: Constraint | tuple[Constraint, str]) -> Self:
        if isinstance(other, Constraint):
            self._cc.__iadd__(other._c)
        elif isinstance(other, tuple):
            constr, name = other
            self._cc.__iadd__((constr._c, name))
        else:
            raise TypeError(f"type of other '{type(other)}' not supported")
        return self

    def __getitem__(self, key: str) -> Constraint:
        return self._cc.__getitem__(key)

    def __setitem__(self, key: str, value: Constraint) -> None:
        return self._cc.__setitem__(key, value._c)

    def __len__(self) -> int:
        return self._cc.__len__()

    def __eq__(self, other: ConstraintCollection) -> bool:  # type: ignore[override]
        return self._cc.__eq__(other._cc)
