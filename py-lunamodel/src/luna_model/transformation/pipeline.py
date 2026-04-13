from typing import Self

from luna_model._lm import PyPipeline
from luna_model.transformation.typing import Pass
from luna_model.wrapper import wraps


class Pipeline(PyPipeline):
    """Todo."""

    def __new__(cls, name: str, steps: list[Pass]) -> Self:
        """Todo."""
        return super().__new__(cls, name=name, steps=steps)

    @wraps()
    def requires(self) -> list[str]:
        """Todo."""
        raise NotImplementedError

    @wraps()
    def invalidates(self) -> list[str]:
        """Todo."""
        raise NotImplementedError

    @wraps()
    def provides(self) -> list[str]:
        """Todo."""
        raise NotImplementedError
