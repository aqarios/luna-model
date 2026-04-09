from __future__ import annotations

from luna_model._lm import PyTransformationRecord
from luna_model.solution.sol import Solution


class TransformationRecord:
    """The transformation record contains all information required to back transform a solution."""

    _tr: PyTransformationRecord

    @classmethod
    def _from_pytr(cls, pytr: PyTransformationRecord) -> TransformationRecord:
        tr = cls.__new__(cls)
        tr._tr = pytr
        return tr

    def backward(self, solution: Solution) -> Solution:
        """Apply the back transformation to the given solution.

        !!! warning "Disclaimer"
            When multiple samples are condensed into a single record (e.g., by omitting
            slack variables), only the first sample's `raw_energy` is retained. As a
            result, the `raw_energy` value may no longer accurately represent the
            condensed group.

        Parameters
        ----------
        solution : Solution
            The solution to transform back to a representation fitting the original model.

        Returns
        -------
        Solution
            A solution object representing a solution to the original problem.
        """
        return Solution._from_pys(self._tr.backward(solution._s))

    def encode(self) -> bytes:
        """Encode the transformation record to bytes.

        Returns
        -------
        bytes
            Encoded transformation record
        """
        return self._tr.encode()

    def serialize(self) -> bytes:
        """Serialize the transformation record to bytes.

        Returns
        -------
        bytes
            Serialized transformation record.
        """
        return self.encode()

    @classmethod
    def decode(cls, data: bytes) -> TransformationRecord:
        """Decode into a TransformationRecord based on the bytes data given an environment. Same as deserialize."""
        return cls._from_pytr(PyTransformationRecord.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> TransformationRecord:
        """Deserialize into a TransformationRecord based on the bytes data given an environment."""
        return cls.decode(data)
