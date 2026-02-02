"""Sample protocols for solution variable assignments.

This module defines protocols for sample objects that provide access to
variable assignments in a solution. Samples can be indexed by variable
ID, name, or Variable object.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Protocol, overload

if TYPE_CHECKING:
    from luna_model.variable import Variable


class Sample(Protocol):
    """Protocol for a single solution sample.

    Represents variable assignments for one solution. Can be accessed
    by variable ID, name, or Variable object.

    Examples
    --------
    >>> result = solution.get_result(0)
    >>> sample = result.sample
    >>> value = sample["x"]  # Access by name
    >>> print(sample.to_dict())

    See Also
    --------
    Samples : Collection of multiple samples.
    Result : Result object that contains a sample.
    """

    def to_dict(self) -> dict[str, int | float]:
        """Convert sample to dictionary mapping variable names to values.
        
        Returns
        -------
        dict[str, int | float]
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


class Samples(Protocol):
    """Protocol for a collection of samples.

    Represents multiple solution samples. Can be indexed to access
    individual samples or specific variable values.

    Examples
    --------
    >>> samples = solution.get_samples()
    >>> first_sample = samples[0]
    >>> specific_value = samples[0, 1]  # Row 0, column (var) 1
    >>> all_samples = samples.tolist()

    See Also
    --------
    Sample : Individual sample protocol.
    Solution : Solution class containing samples.
    """

    def tolist(self) -> list[list[int | float]]:
        """Convert all samples to a list of lists.
        
        Returns
        -------
        list[list[int | float]]
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


class SamplesIter(Protocol):
    """Iterator over multiple samples.

    See Also
    --------
    Samples : The samples collection that uses this iterator.
    """

    def __iter__(self) -> SamplesIter:
        """Return the iterator object itself."""
        ...

    def __next__(self) -> Sample:
        """Get the next sample."""
        ...


class SampleIter(Protocol):
    """Iterator over values in a single sample.

    See Also
    --------
    Sample : The sample that uses this iterator.
    """

    def __iter__(self) -> SampleIter:
        """Return the iterator object itself."""
        ...

    def __next__(self) -> int | float:
        """Get the next variable value."""
        ...
