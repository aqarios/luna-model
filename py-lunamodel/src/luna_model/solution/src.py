from enum import Enum

from luna_model._lm import PyValueSource


class ValueSource(Enum):
    OBJ = "Obj"
    RAW = "Raw"
    # todo: remove this
    Obj = "Obj"
    Raw = "Raw"

    @property
    def name(self) -> str:
        match self:
            case ValueSource.OBJ | ValueSource.Obj:
                return "Obj"
            case ValueSource.RAW | ValueSource.Raw:
                return "Raw"

    @property
    def _val(self) -> PyValueSource:
        match self:
            case ValueSource.OBJ | ValueSource.Obj:
                return PyValueSource.Obj
            case ValueSource.RAW | ValueSource.Raw:
                return PyValueSource.Raw
