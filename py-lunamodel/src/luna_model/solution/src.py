from enum import Enum

from luna_model._lm import PyValueSource


class ValueSource(Enum):
    """Value Source."""

    OBJ = "Obj"
    RAW = "Raw"

    @property
    def _val(self) -> PyValueSource:
        match self:
            case ValueSource.OBJ:
                return PyValueSource.Obj
            case ValueSource.RAW:
                return PyValueSource.Raw
