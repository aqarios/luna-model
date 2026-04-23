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

from enum import Enum

from luna_model._lm import PyValueSource


class ValueSource(Enum):
    """Source of solution values.

    Specifies whether values come from the objective function evaluation
    or from raw solver output.

    Attributes
    ----------
    OBJ : str
        Values from objective function evaluation.
    RAW : str
        Raw values from solver output.
    """

    OBJ = "Obj"
    RAW = "Raw"

    @property
    def _val(self) -> PyValueSource:
        """Convert Python ValueSource to internal representation."""
        match self:
            case ValueSource.OBJ:
                return PyValueSource.Obj
            case ValueSource.RAW:
                return PyValueSource.Raw

    @classmethod
    def _from_pysrc(cls, py_src: PyValueSource) -> ValueSource:
        match py_src:
            case PyValueSource.RAW:
                return ValueSource.RAW
            case PyValueSource.OBJ:
                return ValueSource.OBJ
        msg = f"unknown sense: {py_src}"
        raise RuntimeError(msg)
