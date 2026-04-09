from typing import Protocol, Self


class TransformationPassArtifact(Protocol):
    """Protocol all artifacts produced by a TransformationPass have to adhere to."""

    def serialize(self) -> bytes:
        """Serialize this artifact to a bytes representation."""
        ...

    @classmethod
    def deserialize(cls, buf: bytes) -> Self:
        """Deserialize this artifact from its bytes representation."""
        ...
