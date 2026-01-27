from typing import Protocol


class BasePass(Protocol):
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        ...

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...
