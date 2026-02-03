"""Value source enumeration for solution values.

This module defines where solution values originate from (objective or raw).
"""

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

    See Also
    --------
    Solution : Solution class that uses value sources.
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
