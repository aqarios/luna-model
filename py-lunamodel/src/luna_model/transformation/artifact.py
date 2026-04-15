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

import pickle
from abc import ABC
from typing import Protocol, Self, cast


class TransformationPassArtifact(Protocol):
    """Protocol all artifacts produced by a TransformationPass have to adhere to."""

    def serialize(self) -> bytes:
        """Serialize this artifact to a bytes representation."""
        ...

    @classmethod
    def deserialize(cls, buf: bytes) -> Self:
        """Deserialize this artifact from its bytes representation."""
        ...


class NothingArtifact(TransformationPassArtifact):
    """An artifact implementation that can be used as a placeholder artifact.

    Use this artifact if your transformation pass does not require additional (stateful)
    information to implement the ``backward`` function.
    """

    def serialize(self) -> bytes:
        """Serialize this artifact to a bytes representation.

        The NothingArtifact serialization produces empty bytes.
        """
        return b""

    @classmethod
    def deserialize(cls, buf: bytes) -> Self:
        """Deserialize this artifact from its bytes representation.

        The NothingArtifact deserialization can be produced from empty bytes.
        """
        if len(buf) == 0:
            return cls()

        msg = f"A NothingArtifact cannot be built from non-empty bytes. Bytes length is: {len(buf)}"
        raise ValueError(msg)


class PickleArtifact(TransformationPassArtifact, ABC):
    """An artifact implementation using pickle for serialization that can be used as a base artifact.

    !!! danger "DANGER"
        Do not use PickleArtifact by default.
        Use it only if you fully understand Python pickle internals and the associated security risks. Unpickling
        data from untrusted or unauthenticated sources can execute arbitrary code and compromise your system.
        Prefer safer formats whenever possible
    """

    def serialize(self) -> bytes:
        """Serialize this artifact to a bytes representation.

        The NothingArtifact serialization produces empty bytes.
        """
        return pickle.dumps(self)

    @classmethod
    def deserialize(cls, buf: bytes) -> Self:
        """Deserialize this artifact from its bytes representation.

        The NothingArtifact deserialization can be produced from empty bytes.
        """
        return cast("Self", pickle.loads(buf))  # noqa: S301
