from __future__ import annotations

from typing import TYPE_CHECKING, Self

from luna_model._lm import PyConstraintCollection
from luna_model._utils import wrap_c
from luna_model.constraint.cmp import Comparator
from luna_model.constraint.constr import Constraint
from luna_model.constraint.iter import ConstraintCollectionIter

if TYPE_CHECKING:
    from luna_model.environment.env import Environment


class ConstraintCollection:
    """Collection of constraints."""

    _cc: PyConstraintCollection

    def __init__(self) -> None:
        self._cc = PyConstraintCollection()

    @classmethod
    def _from_pycc(cls, py_cc: PyConstraintCollection) -> ConstraintCollection:
        """Construct LunaModel ConstraintCollection from FFI PyConstraintCollection object."""
        cc = cls.__new__(cls)
        cc._cc = py_cc
        return cc

    def add_constraint(self, constraint: Constraint, name: str | None = None) -> None:
        """Add a constraint."""
        self._cc.add_constraint(constraint._c, name)

    def items(self) -> ConstraintCollectionIter:
        """Get the items of the ConstraintCollection as an iterator. Same as __iter__."""
        return ConstraintCollectionIter._from_pycci(self._cc.items())

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Encode the constraint. Same as serialize."""
        return self._cc.encode(compress, level)

    def serialize(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Serialize the constraint."""
        return self.encode(compress, level)

    def get(self, name: str) -> Constraint:
        """Get a constraint for its name."""
        return wrap_c(self._cc.get(name))

    def remove(self, name: str) -> None:
        """Remove a constraint by its name."""
        self._cc.remove(name)

    def equal_contents(self, other: ConstraintCollection) -> bool:
        """Check if two ConstraintCollections have the same contents without checking for equality."""
        return self._cc.equal_contents(other._cc)

    def ctypes(self) -> list[Comparator]:
        """Get the comparator types for each Comparator."""
        return [Comparator._from_pycmp(c) for c in self._cc.ctypes()]

    @classmethod
    def decode(cls, data: bytes, env: Environment) -> ConstraintCollection:
        """Decode into a ConstraintCollection based on the bytes data given an environment. Same as deserialize."""
        return cls._from_pycc(PyConstraintCollection.decode(data, env._env))

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> ConstraintCollection:
        """Deserialize into a ConstraintCollection based on the bytes data given an environment."""
        return cls.decode(data, env)

    def __iadd__(self, other: Constraint | tuple[Constraint, str]) -> Self:
        """Add a new constraint."""
        if isinstance(other, Constraint):
            self._cc.__iadd__(other._c)
        elif isinstance(other, tuple):
            constr, name = other
            self._cc.__iadd__((constr._c, name))
        else:
            msg = f"type of other '{type(other)}' not supported"
            raise TypeError(msg)
        return self

    def __getitem__(self, key: str) -> Constraint:
        """Get a constraint by its name 'key'."""
        return wrap_c(self._cc.__getitem__(key))

    def __setitem__(self, key: str, value: Constraint) -> None:
        """Set a constraint for name 'key'."""
        return self._cc.__setitem__(key, value._c)

    def __len__(self) -> int:
        """Get the length."""
        return self._cc.__len__()

    def __eq__(self, other: ConstraintCollection) -> bool:  # type: ignore[override]
        """Compare for equality."""
        return self._cc.__eq__(other._cc)

    def __iter__(self) -> ConstraintCollectionIter:
        """Iterate the constraints."""
        return ConstraintCollectionIter._from_pycci(self._cc.__iter__())

    def __hash__(self) -> int:
        """Compute hash."""
        return self._cc.__hash__()
