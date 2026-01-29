from typing import Literal
from luna_model._lm import PyBinarySpinPass

from luna_model.model.model import Vtype
from luna_model.transformation.transform import ConcreteTransformationPass


class BinarySpinPass(ConcreteTransformationPass):
    """An transformation pass changing the binary/spin variables to spin/binary."""

    def __init__(self, vtype: Literal[Vtype.BINARY, Vtype.SPIN], prefix: str | None) -> None:
        super().__init__(base=PyBinarySpinPass(vtype._val, prefix))

    @property
    def vtype(self) -> Vtype:
        """Get the target vtype."""
        return Vtype._from_pyvtype(self._base.vtype)

    @property
    def prefix(self) -> str | None:
        """Get the naming prefix."""
        return self._base.prefix
