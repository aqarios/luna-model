from enum import Enum

from luna_model._lm import PyValueSource

class ValueSource(Enum):
    OBJ = PyValueSource.Obj
    RAW = PyValueSource.Raw
    # todo: remove this
    Obj = PyValueSource.Obj
    Raw = PyValueSource.Raw
