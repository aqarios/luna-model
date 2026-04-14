from typing import Self

from luna_model._lm import PyPipeline
from luna_model.transformation.typing import Pass
from luna_model.wrapper import wraps


class Pipeline(PyPipeline):
    """
    A pipeline for executing multiple transformation passes in sequence.

    Pipelines organize and execute multiple passes, managing dependencies
    and ensuring they run in the correct order.

    Parameters
    ----------
    passes : list[BasePass]
        The transformation passes to include in the pipeline.
    name : str, optional
        A custom name for the pipeline. If not provided, a default name
        will be generated.
    """

    def __new__(cls, steps: list[Pass], name: str) -> Self:
        """Todo."""
        return super().__new__(cls, name=name, steps=steps)

    @wraps()
    def add(self, new_pass: Pass) -> None:
        """
        Add a new pass to the pipeline.

        Parameters
        ----------
        new_pass : Pass
            The pass to add to the pipeline.
        """
        raise NotImplementedError(f"add({new_pass})")

    @wraps()
    def requires(self) -> list[str]:
        """
        List of passes that must run before this pipeline.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        raise NotImplementedError

    @wraps()
    def invalidates(self) -> list[str]:
        """Todo."""
        raise NotImplementedError

    @wraps()
    def provides(self) -> list[str]:
        """Todo."""
        raise NotImplementedError

    @wraps()
    def __str__(self) -> str:
        """Human readable string."""
        raise NotImplementedError

    @wraps()
    def __repr__(self) -> str:
        """Todo."""
        raise NotImplementedError
