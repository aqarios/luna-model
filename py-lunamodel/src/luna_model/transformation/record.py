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

from typing import Generic, TypeAlias, TypeVar

from luna_model._lm import PyPassEntry, PyTransformationRecord
from luna_model.solution.sol import Solution
from luna_model.transformation.artifact import TransformationPassArtifact


class TransformationRecord:
    """The transformation record contains all information required to back transform a solution."""

    _tr: PyTransformationRecord

    @classmethod
    def _from_pytr(cls, pytr: PyTransformationRecord) -> TransformationRecord:
        tr = cls.__new__(cls)
        tr._tr = pytr
        return tr

    @property
    def entries(self) -> list[PassEntry]:
        """Get all recorded pass entries in forward execution order.

        Returns
        -------
        list[PassEntry]
            Recorded pass entries converted to Python-facing entry wrappers.
        """
        return [_from_pyentry(e) for e in self._tr.entries]

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
        """Decode a transformation record from bytes.

        Parameters
        ----------
        data : bytes
            Encoded transformation record data.

        Returns
        -------
        TransformationRecord
            Decoded transformation record.
        """
        return cls._from_pytr(PyTransformationRecord.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> TransformationRecord:
        """Deserialize a transformation record from bytes.

        This is an alias for :meth:`decode`.

        Parameters
        ----------
        data : bytes
            Serialized transformation record data.

        Returns
        -------
        TransformationRecord
            Deserialized transformation record.
        """
        return cls.decode(data)

    def find(self, query: str, *, exact: bool = False) -> PassEntry:
        """Find the first pass entry whose name/id matches the query.

        Searches this record and nested pipeline/control-flow records in forward order.

        Parameters
        ----------
        query : str
            Full pass name/id or partial fragment.
        exact : bool, optional
            If True, require exact equality. Otherwise use case-insensitive
            substring matching. Defaults to False.

        Returns
        -------
        PassEntry
            The first matching pass entry.

        Raises
        ------
        ValueError
            If query is empty.
        LookupError
            If no matching entry is found.
        """
        return _from_pyentry(self._tr.find(query, exact))


A = TypeVar("A", bound=TransformationPassArtifact)


class TransformEntry(Generic[A]):
    """Transform entry containing pass identity and serialized artifact."""

    _t: PyPassEntry.Transform

    @classmethod
    def _from_pyt(cls, py_t: PyPassEntry.Transform) -> TransformEntry:
        t = cls.__new__(cls)
        t._t = py_t
        return t

    @property
    def pass_id(self) -> str:
        """Unique pass identifier used by backward dispatch."""
        return self._t.pass_id

    @property
    def pass_name(self) -> str:
        """Human-readable pass name."""
        return self._t.pass_name

    @property
    def artifact(self) -> A:
        """Artifact produced by the transform pass."""
        return self._t.artifact

    def __str__(self) -> str:
        """Human readable string."""
        return self._t.__str__()


class AnalysisEntry:
    """Analysis entry representing a non-reversible analysis execution."""

    _a: PyPassEntry.Analysis

    @classmethod
    def _from_pya(cls, py_a: PyPassEntry.Analysis) -> AnalysisEntry:
        a = cls.__new__(cls)
        a._a = py_a
        return a

    @property
    def pass_name(self) -> str:
        """Human-readable analysis pass name."""
        return self._a.pass_name

    def __str__(self) -> str:
        """Human readable string."""
        return self._a.__str__()


class PipelineEntry:
    """Nested pipeline entry containing a sub-record."""

    _p: PyPassEntry.Pipeline

    @classmethod
    def _from_pyp(cls, py_p: PyPassEntry.Pipeline) -> PipelineEntry:
        p = cls.__new__(cls)
        p._p = py_p
        return p

    @property
    def name(self) -> str:
        """Nested pipeline name."""
        return self._p.name

    @property
    def record(self) -> TransformationRecord:
        """Nested transformation record for this pipeline step."""
        return TransformationRecord._from_pytr(self._p.record)

    def __str__(self) -> str:
        """Human readable string."""
        return self._p.__str__()


class ControlFlowEntry:
    """Control-flow entry containing the selected branch record."""

    _cf: PyPassEntry.ControlFlow

    @classmethod
    def _from_pycf(cls, py_cf: PyPassEntry.ControlFlow) -> ControlFlowEntry:
        cf = cls.__new__(cls)
        cf._cf = py_cf
        return cf

    @property
    def pass_name(self) -> str:
        """Name of the control-flow pass that selected the branch."""
        return self._cf.pass_name

    @property
    def name(self) -> str:
        """Name of the selected control-flow branch plan."""
        return self._cf.name

    @property
    def record(self) -> TransformationRecord:
        """Nested transformation record executed for the selected branch."""
        return TransformationRecord._from_pytr(self._cf.record)

    def __str__(self) -> str:
        """Human readable string."""
        return self._cf.__str__()


PassEntry: TypeAlias = TransformEntry | AnalysisEntry | ControlFlowEntry | PipelineEntry


def _from_pyentry(py_e: PyPassEntry) -> PassEntry:
    """Convert a raw PyPassEntry variant into a Python-facing entry wrapper."""
    match type(py_e):
        case PyPassEntry.Transform:
            return TransformEntry._from_pyt(py_e)
        case PyPassEntry.Analysis:
            return AnalysisEntry._from_pya(py_e)
        case PyPassEntry.Pipeline:
            return PipelineEntry._from_pyp(py_e)
        case PyPassEntry.ControlFlow:
            return ControlFlowEntry._from_pycf(py_e)
    raise NotImplementedError
