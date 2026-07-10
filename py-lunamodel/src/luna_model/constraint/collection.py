# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from __future__ import annotations

from collections.abc import Sequence
from typing import TYPE_CHECKING, Self, TypeAlias, overload

from numpy.typing import NDArray

from luna_model._lm import PyConstraintCollection
from luna_model._utils import wrap_c
from luna_model.constraint.cmp import Comparator
from luna_model.constraint.constr import Constraint
from luna_model.constraint.iter import ConstraintCollectionIter
from luna_model.matrix import NDLmArray

if TYPE_CHECKING:
    from luna_model.environment.env import Environment


SingleConstraint: TypeAlias = Constraint | tuple[Constraint, str]
ManyConstraint: TypeAlias = (
    Sequence[Constraint | tuple[Constraint, str] | tuple[str, Constraint]]
    | NDLmArray
    | NDArray
    | tuple[Sequence[Constraint], str]
    | tuple[NDLmArray, str]
    | tuple[NDArray, str]
)


class ConstraintCollection:
    """Collection for managing multiple constraints.

    A ConstraintCollection stores named constraints and provides methods
    for adding, retrieving, and iterating over constraints.

    Attributes
    ----------
    None directly exposed. Access constraints via indexing or iteration.

    Examples
    --------
    Create and manage constraints:

    >>> from luna_model import Variable, Environment
    >>> from luna_model import ConstraintCollection
    >>> with Environment():
    ...     x, y = Variable("x"), Variable("y")
    >>> cc = ConstraintCollection()
    >>> cc += x + y <= 10, "capacity"
    >>> cc += x >= 0, "x_lower"
    >>> print(len(cc))  # Number of constraints
    2
    >>> constraint = cc["capacity"]  # Access by name

    Iterate over constraints:

    >>> for name, constr in cc:
    ...     print(f"{name}: {constr}")
    capacity: x + y <= 10
    x_lower: x >= 0

    Notes
    -----
    Constraints are stored with unique string names. Adding a constraint with
    an existing name will raise a ``DuplicateConstraintNameError``.
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
        name : str, optional
            Name for the constraint. If None, uses constraint's own name if it has one
            that is not auto-generated, otherwise generates a name following the pattern
            ``c{i}`` where i is the constraint's index in the collection (starting from 0).
        """
        self._cc.add_constraint(constraint, name)

    @overload
    def add_constraints(
        self,
        constraints: ConstraintCollection | Sequence[Constraint | tuple[str, Constraint] | tuple[Constraint, str]],
        name: str | None = None,
    ) -> list[str]: ...
    @overload
    def add_constraints(
        self,
        constraints: NDArray | NDLmArray | Sequence[Constraint],
        name: str | Sequence[str] | None = None,
    ) -> list[str]: ...
    def add_constraints(
        self,
        constraints: ConstraintCollection
        | NDArray
        | NDLmArray
        | Sequence[Constraint]
        | Sequence[Constraint | tuple[str, Constraint] | tuple[Constraint, str]],
        name: str | Sequence[str] | None = None,
    ) -> list[str]:
        """Add multiple constraints to the collection.

        Parameters
        ----------
        constraints : ConstraintCollection | NDArray | NDLmArray | Sequence[Constraint]
                        | Sequence[Constraint | tuple[str, Constraint] | tuple[Constraint, str]]
            Constraints to add.

            Behavior depends on the input kind:

            - ``ConstraintCollection``: imports the full collection.
            - ``NDArray`` / ``NDLmArray`` / ``Sequence[Constraint]``: each element is added as one constraint.
            - ``Sequence[Constraint | tuple[str, Constraint] | tuple[Constraint, str]]``:
              each tuple provides an explicit per-constraint name.
        name : str | Sequence[str] | None, optional
            Naming mode for added constraints:

            - ``None``: use explicit tuple names when present; otherwise use each
              constraint's own non-auto-generated name, falling back to generated names.
            - ``str``: use as a base prefix.
              For plain sequences/arrays names are ``{name}_{i}``.
              For tuple inputs names are ``{name}_{tuple_name}`` when a tuple name is
              present, otherwise ``{name}_{i}``.
            - ``Sequence[str]``: allowed only with ``NDArray``, ``NDLmArray``, or
              ``Sequence[Constraint]`` and must have the same length as ``constraints``.

        Returns
        -------
        list[str]
            The names of the added constraints.

        Raises
        ------
        DuplicateConstraintNameError
            If a constraint with the same name already exists.
        IllegalConstraintNameError
            If the constraint name is invalid. Constraint names cannot be empty
            strings and must start with an alphabetical character. Additionally,
            constraint names cannot start with ``inf`` or ``nan`` due to
            limitations of other modeling software.
        LunaModelError
            If ``name`` is a sequence and its length does not match the number of
            constraints. Also raised for unsupported combinations when runtime type
            checks are bypassed (for example, ``ConstraintCollection`` with
            ``Sequence[str]`` names).
        """
        return self._cc.add_constraints(constraints, name)

    def items(self) -> ConstraintCollectionIter:
        """Get an iterator over (name, constraint) pairs.

        Returns
        -------
        ConstraintCollectionIter
            Iterator yielding (name, constraint) tuples.
        """
        return ConstraintCollectionIter._from_pycci(self._cc.items())

    def encode(self) -> bytes:
        """Encode the constraint collection to bytes.

        Returns
        -------
        bytes
            Encoded constraint collection.
        """
        return self._cc.encode()

    def serialize(self) -> bytes:
        """Serialize the constraint collection to bytes.

        Returns
        -------
        bytes
            Serialized constraint collection.
        """
        return self.encode()

    def get(self, name: str) -> Constraint:
        """Get a constraint by its name.

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
        NoConstraintForKeyError
            If no constraint with the given name exists.
        """
        return wrap_c(self._cc.get(name))

    def remove(self, name: str) -> None:
        """Remove a constraint by its name.

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
        return self._cc.equal_contents(other)

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
        return cls._from_pycc(PyConstraintCollection.decode(data, env))

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> ConstraintCollection:
        """Deserialize into a ConstraintCollection based on the bytes data given an environment."""
        return cls.decode(data, env)

    def __iadd__(
        self, other: SingleConstraint | ManyConstraint | ConstraintCollection | tuple[ConstraintCollection, str]
    ) -> Self:
        """Add a constraint using += operator.

        Parameters
        ----------
        other : SingleConstraint | ManyConstraint | ConstraintCollection | tuple[ConstraintCollection, str]
            Either a Constraint, a (Constraint, name) tuple, a ConstraintCollection,
            a (ConstraintCollection, prefix) tuple, a sequence of either Constraint or
            (Constraint, str) or a (Sequence[Constraint], base_name) tuple.

            The constraint names of the added Constraint, ConstraintCollection or Sequence have to be different from
            this collection's constraint names. Otherwise the DuplicateConstraintNameError is
            raised.


        Returns
        -------
        Self
            The collection itself for chaining.

        Raises
        ------
        DuplicateConstraintNameError
            If a constraint is added for a name that is already contained in this collection.
            Or if a ConstraintCollection is added containing a constraint with a name that is
            already in this collection.
            Or if a Sequence is added containing a constraint with a name that is
            already in this collection.
        """
        self._cc.__iadd__(other)
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
        return self._cc.__setitem__(key, value)

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
        return self._cc.__eq__(other)

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

    def __contains__(self, constr: str) -> bool:
        """Check if a constraint name exists in this constraint collection.

        Parameters
        ----------
        constr : str
            The constraint name to check.

        Returns
        -------
        bool
            True if a constraint with the given name exists.
        """
        return self._cc.__contains__(constr)
