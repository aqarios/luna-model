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

from typing import TYPE_CHECKING, Any, Literal, Protocol, overload

from luna_model._lm import PyAnalysisCache

if TYPE_CHECKING:
    from luna_model.variable.vtype import Vtype


class MaxBias(Protocol):
    """Protocol for maximum bias information stored in the analysis cache.

    This protocol defines the interface for accessing maximum bias values
    computed during model analysis.
    """

    @property
    def val(self) -> float:
        """Get the maximum bias value.

        Returns
        -------
        float
            The maximum bias value.
        """
        ...


class BinarySpinInfo(Protocol):
    """Protocol for binary spin transformation information.

    This protocol defines the interface for accessing information about
    binary-to-spin/spin-to-binary variable transformations, including the
    source and target variable types and the mapping between variable names.
    """

    @property
    def old_vtype(self) -> Vtype:
        """Get the source variable type before transformation.

        Returns
        -------
        Vtype
            The original variable type.
        """
        ...

    @property
    def new_vtype(self) -> Vtype:
        """Get the target variable type after transformation.

        Returns
        -------
        Vtype
            The transformed variable type.
        """
        ...

    @property
    def map(self) -> dict[str, str]:
        """Get the variable name mapping from old to new names.

        Returns
        -------
        dict[str, str]
            Dictionary mapping old variable names to new variable names.
        """
        ...


class IfElseInfo(Protocol):
    """Protocol for if-else pass information.

    This protocol defines the interface for accessing information about
    whether an if-else condition was fulfilled during model transformation.
    """

    @property
    def fulfilled_condition(self) -> bool:
        """Check if the if-else condition was fulfilled.

        Returns
        -------
        bool
            True if the condition was fulfilled, False otherwise.
        """
        ...


class MinConstraintValues(Protocol):
    """Protocol for MinValueForConstraints information stored in the analysis cache.

    This protocol defines the interface for accessing min value for constraints values
    computed during model analysis.
    """

    @property
    def vals(self) -> dict[str, float]:
        """Get the minimum values possible for the constraints.

        Returns
        -------
        dict[str, float]
            The minimum possible value for all constraints.
        """
        ...


class AnalysisCache:
    """Cache for storing analysis metadata during model transformations.

    The AnalysisCache provides a dictionary-like interface for storing and
    retrieving metadata generated during model analysis and transformation
    operations. It supports type-safe access to known cache keys through
    overloaded `__getitem__` methods.
    """

    _ac: PyAnalysisCache

    def __init__(self) -> None:
        """Initialize a new empty analysis cache."""
        self._ac = PyAnalysisCache()

    @classmethod
    def _from_pyac(cls, py_ac: PyAnalysisCache) -> AnalysisCache:
        ac = cls.__new__(cls)
        ac._ac = py_ac
        return ac

    @overload
    def __getitem__(self, key: Literal["max-bias"]) -> MaxBias: ...
    @overload
    def __getitem__(self, key: Literal["binary-spin"]) -> BinarySpinInfo: ...
    @overload
    def __getitem__(self, key: Literal["min-value-for-constraint"]) -> MinConstraintValues: ...
    @overload
    def __getitem__(self, key: str) -> Any: ...  # noqa: ANN401
    def __getitem__(self, key: str) -> Any:
        """Get the cache item for the specified key.

        Parameters
        ----------
        key : str
            The cache key to retrieve.

        Returns
        -------
        Any
            The cached value associated with the key.

        Raises
        ------
        KeyError
            If the key is not found in the cache.
        """
        return self._ac[key]
