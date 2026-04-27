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
    >>> from luna_model import Variable, Environment
    >>> from luna_model import ConstraintCollection
    >>> with Environment():
    ...     x, y = Variable("x"), Variable("y")
    >>> cc = ConstraintCollection()
    >>> cc += x + y <= 10, "capacity"
    >>> cc += x >= 0, "x_lower"
    >>> for name, constraint in cc:
    ...     print(f"{name}: {constraint}")
    capacity: x + y <= 10
    x_lower: x >= 0
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
