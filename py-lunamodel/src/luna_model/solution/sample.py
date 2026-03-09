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

from typing import TYPE_CHECKING, Protocol, overload, runtime_checkable

if TYPE_CHECKING:
    from collections.abc import Sequence

    from luna_model.variable import Variable

@runtime_checkable
class Sample(Protocol):
    """Protocol for a single solution sample.

    Represents variable assignments for one solution. Can be accessed
    by variable ID, name, or Variable object.

    Examples
    --------
    >>> from luna_model import Environment, Solution, Vtype
    >>> solution = Solution(
    ...     [{"x": 1, "y": 0, "z": 1}], counts=[1], raw_energies=[3], vtypes=[Vtype.BINARY, Vtype.BINARY, Vtype.BINARY]
    ... )
    >>> result = solution[0]
    >>> sample = result.sample
    >>> value = sample["x"]  # Access by name
    >>> print(value)
    1
    >>> print(sample.to_dict())
    {'x': 1, 'y': 0, 'z': 1}
    """

    def to_dict(self) -> dict[str, int | float]:
        """Convert sample to dictionary mapping variable names to values.

        Returns
        -------
        dict[str, int or float]
            Dictionary of variable assignments.
        """
        ...

    def __getitem__(self, item: int | Variable | str) -> int | float:
        """Get a variable value by ID, Variable, or name."""
        ...

    def __len__(self) -> int:
        """Get the number of variables in the sample."""
        ...

    def __iter__(self) -> SampleIter:
        """Iterate over variable values in the sample."""
        ...

    def __str__(self, /) -> str:
        """Return string representation of the sample."""
        ...

@runtime_checkable
class Samples(Protocol):
    """Protocol for a collection of samples.

    Represents multiple solution samples. Can be indexed to access
    individual samples or specific variable values.

    Examples
    --------
    >>> from luna_model import Environment, Solution, Vtype
    >>> solution = Solution(
    ...     [
    ...         {"x": 1, "y": 0, "z": 1},
    ...         {"x": 0, "y": 0, "z": 1},
    ...         {"x": 1, "y": 1, "z": 1},
    ...     ],
    ...     counts=[1, 1, 2],
    ...     raw_energies=[2, 1, 3],
    ...     vtypes=[Vtype.BINARY, Vtype.BINARY, Vtype.BINARY],
    ... )
    >>> samples = solution.samples
    >>> first_sample = samples[0]
    >>> print(first_sample)
    [1, 0, 1]
    >>> specific_value = samples[0, 1]  # Row 0, column (var) 1
    >>> print(specific_value)
    0.0
    >>> all_samples = samples.tolist()
    >>> print(all_samples)
    [[1, 0, 1], [0, 0, 1], [1, 1, 1]]
    """

    def tolist(self) -> Sequence[Sequence[int | float]]:
        """Convert all samples to a list of lists.

        Returns
        -------
        Sequence[Sequence[int or float]]
            List where each inner list is a sample.
        """
        ...

    @overload
    def __getitem__(self, item: int) -> Sample: ...
    @overload
    def __getitem__(self, item: tuple[int, int]) -> float: ...
    def __getitem__(self, item: int | tuple[int, int]) -> Sample | float:
        """Get a sample by index or a specific value by (sample_idx, var_idx)."""
        ...

    def __len__(self) -> int:
        """Get the number of samples."""
        ...

    def __iter__(self) -> SamplesIter:
        """Iterate over all samples."""
        ...


@runtime_checkable
class SamplesIter(Protocol):
    """Iterator over multiple samples."""

    def __iter__(self) -> SamplesIter:
        """Return the iterator object itself."""
        ...

    def __next__(self) -> Sample:
        """Get the next sample."""
        ...

@runtime_checkable
class SampleIter(Protocol):
    """Iterator over values in a single sample."""

    def __iter__(self) -> SampleIter:
        """Return the iterator object itself."""
        ...

    def __next__(self) -> int | float:
        """Get the next variable value."""
        ...
