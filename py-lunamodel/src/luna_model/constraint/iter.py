"""Iterator for constraint collections.

This module provides an iterator class for traversing constraint collections.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._utils import wrap_c

if TYPE_CHECKING:
    from luna_model._lm import PyConstraintCollectionIterator
    from luna_model.constraint.constr import Constraint


class ConstraintCollectionIter:
    """Iterator for traversing constraint collections.

    Yields (name, constraint) tuples when iterating over a ConstraintCollection.

    Examples
    --------
    >>> cc = ConstraintCollection()
    >>> # Add constraints...
    >>> for name, constraint in cc:
    ...     print(f"{name}: {constraint}")

    See Also
    --------
    ConstraintCollection : The collection class that uses this iterator.
    """

    _i: PyConstraintCollectionIterator

    def __next__(self) -> tuple[str, Constraint]:
        """Get the next (name, constraint) pair.

        Returns
        -------
        tuple[str, Constraint]
            The constraint name and constraint object.

        Raises
        ------
        StopIteration
            When there are no more constraints.
        """
        name, c = self._i.__next__()
        return name, wrap_c(c)

    def __iter__(self) -> ConstraintCollectionIter:
        """Return the iterator object itself."""
        return self

    @classmethod
    def _from_pycci(cls, py_cci: PyConstraintCollectionIterator) -> ConstraintCollectionIter:
        """Construct LunaModel ConstraintCollectionIter from FFI PyConstraintCollectionIterator object."""
        i = cls.__new__(cls)
        i._i = py_cci
        return i
