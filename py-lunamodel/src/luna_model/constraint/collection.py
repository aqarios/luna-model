"""Constraint collections for managing multiple constraints.

This module provides the ConstraintCollection class for storing and managing
multiple constraints in an optimization model.
"""

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
    """Collection for managing multiple constraints.

    A ConstraintCollection stores named constraints and provides methods for
    adding, retrieving, and iterating over constraints in an optimization model.

    Attributes
    ----------
    None directly exposed. Access constraints via indexing or iteration.

    Examples
    --------
    Create and manage constraints:

    >>> from luna_model import Variable, Environment
    >>> from luna_model.constraint import ConstraintCollection
    >>> with Environment():
    ...     x, y = Variable("x"), Variable("y")
    >>> cc = ConstraintCollection()
    >>> cc += x + y <= 10, "capacity"
    >>> cc += x >= 0, "x_lower"
    >>> print(len(cc))  # Number of constraints
    >>> constraint = cc["capacity"]  # Access by name

    Iterate over constraints:

    >>> for name, constr in cc:
    ...     print(f"{name}: {constr}")

    Notes
    -----
    Constraints are stored with unique string names. Adding a constraint with
    an existing name will replace the previous constraint.

    See Also
    --------
    Constraint : Individual constraint class.
    Model : Model class that uses constraint collections.
    """

    _cc: PyConstraintCollection

    def __init__(self) -> None:
        """Initialize an empty constraint collection."""
        self._cc = PyConstraintCollection()

    @classmethod
    def _from_pycc(cls, py_cc: PyConstraintCollection) -> ConstraintCollection:
        """Construct LunaModel ConstraintCollection from FFI PyConstraintCollection object."""
        cc = cls.__new__(cls)
        cc._cc = py_cc
        return cc

    def add_constraint(self, constraint: Constraint, name: str | None = None) -> None:
        """Add a constraint to the collection.

        Parameters
        ----------
        constraint : Constraint
            The constraint to add.
        name : str | None, optional
            Name for the constraint. If None, uses constraint's own name if it has one,
            otherwise generates a name following the pattern ``c{i}`` where i is the
            constraint's index (starting from 0).
        """
        self._cc.add_constraint(constraint._c, name)

    def items(self) -> ConstraintCollectionIter:
        """Get an iterator over (name, constraint) pairs.

        Returns
        -------
        ConstraintCollectionIter
            Iterator yielding (name, constraint) tuples.
        """
        return ConstraintCollectionIter._from_pycci(self._cc.items())

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Encode the constraint collection to bytes.

        Parameters
        ----------
        compress : bool | None, default=True
            Whether to compress the output.
        level : int | None, default=3
            Compression level (0-9).

        Returns
        -------
        bytes
            Encoded constraint collection.
        """
        return self._cc.encode(compress, level)

    def serialize(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Serialize the constraint collection to bytes.

        Parameters
        ----------
        compress : bool | None, default=True
            Whether to compress the output.
        level : int | None, default=3
            Compression level (0-9).

        Returns
        -------
        bytes
            Serialized constraint collection.
        """
        return self.encode(compress, level)

    def get(self, name: str) -> Constraint:
        """Get a constraint by name.

        Parameters
        ----------
        name : str
            The name of the constraint.

        Returns
        -------
        Constraint
            The constraint with the given name.

        Raises
        ------
        KeyError
            If no constraint with the given name exists.
        """
        return wrap_c(self._cc.get(name))

    def remove(self, name: str) -> None:
        """Remove a constraint by name.

        Parameters
        ----------
        name : str
            The name of the constraint to remove.

        Raises
        ------
        KeyError
            If no constraint with the given name exists.
        """
        self._cc.remove(name)

    def equal_contents(self, other: ConstraintCollection) -> bool:
        """Check if two constraint collections have the same contents.

        Parameters
        ----------
        other : ConstraintCollection
            The collection to compare with.

        Returns
        -------
        bool
            True if both collections contain the same constraints.
        """
        return self._cc.equal_contents(other._cc)

    def ctypes(self) -> list[Comparator]:
        """Get the comparator types of all constraints.

        Returns
        -------
        list[Comparator]
            List of comparators for each constraint.
        """
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
        """Add a constraint using += operator.

        Parameters
        ----------
        other : Constraint | tuple[Constraint, str]
            Either a Constraint or a (Constraint, name) tuple.

        Returns
        -------
        Self
            The collection itself for chaining.
        """
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
        """Get a constraint by name using indexing.

        Parameters
        ----------
        key : str
            The constraint name.

        Returns
        -------
        Constraint
            The constraint with the given name.
        """
        return wrap_c(self._cc.__getitem__(key))

    def __setitem__(self, key: str, value: Constraint) -> None:
        """Set a constraint by name using indexing.

        Parameters
        ----------
        key : str
            The constraint name.
        value : Constraint
            The constraint to store.
        """
        return self._cc.__setitem__(key, value._c)

    def __len__(self) -> int:
        """Get the number of constraints in the collection.

        Returns
        -------
        int
            The number of constraints.
        """
        return self._cc.__len__()

    def __eq__(self, other: ConstraintCollection) -> bool:  # type: ignore[override]
        """Compare for equality."""
        return self._cc.__eq__(other._cc)

    def __iter__(self) -> ConstraintCollectionIter:
        """Iterate over (name, constraint) pairs.

        Returns
        -------
        ConstraintCollectionIter
            Iterator over the constraints.
        """
        return ConstraintCollectionIter._from_pycci(self._cc.__iter__())

    def __hash__(self) -> int:
        """Compute hash."""
        return self._cc.__hash__()

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the constraint collection.
        """
        return self._cc.__str__()
