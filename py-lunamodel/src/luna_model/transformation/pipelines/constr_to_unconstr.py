from __future__ import annotations
import sys
from typing import TYPE_CHECKING, Protocol, overload

if sys.version_info < (3, 12):
    from typing_extensions import override
else:
    from typing import override

from luna_model._lm import PyConstrainedToUnconstrainedPipeline

if TYPE_CHECKING:
    from luna_model.transformation.base import BasePass


class ConstrainedToUnconstrainedPipeline:
    """todo."""

    def __new__(cls, penalty_factor: float = 10.0) -> Pipeline:
        """Todo."""
        return PyConstrainedToUnconstrainedPipeline.create(penalty_factor)

    def __init__(self, penalty_factor: float = 10.0) -> None: ...


class Pipeline(Protocol):
    """todo."""

    @property
    def name(self) -> str:
        """Todo."""
        ...

    @property
    def requires(self) -> list[str]:
        """Todo."""
        ...

    @property
    def satisfies(self) -> set[str]:
        """
        Get the set of pass requirements that this pipeline satisfies.

        Returns
        -------
        set of str
            Names of pass requirements satisfied by executing this pipeline.
        """
        ...

    @property
    def passes(self) -> list[BasePass]:
        """
        Get all passes that are part of the pipeline.

        Returns
        -------
        list of BasePass
            The transformation passes in this pipeline.
        """
        ...

    def __str__(self) -> str:
        """
        Get a string representation of the pipeline.

        Returns
        -------
        str
            A human-readable string describing the pipeline.
        """
        ...

    def __repr__(self) -> str:
        """
        Get a detailed string representation of the pipeline for debugging.

        Returns
        -------
        str
            A detailed string representation suitable for debugging.
        """
        ...
