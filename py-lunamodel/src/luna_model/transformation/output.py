from __future__ import annotations

from luna_model._lm import PyTransformationOutput
from luna_model.model.model import Model
from luna_model.transformation.record import TransformationRecord


class TransformationOutput:
    """The result of the PassManager's run method.

    It contains the transformed model, the TransformationRecord and the final PassContext.
    """

    _to: PyTransformationOutput

    @classmethod
    def _from_pyto(cls, pyto: PyTransformationOutput) -> TransformationOutput:
        to = cls.__new__(cls)
        to._to = pyto
        return to

    @property
    def model(self) -> Model:
        """
        Get the transformed model.

        Returns
        -------
        Model
            The model after execution of the PassManager.
        """
        return Model._from_pym(self._to.model)

    @property
    def record(self) -> TransformationRecord:
        """
        Get the transformation record produced during the PassManager execution.

        Returns
        -------
        TransformationRecord
            The transformation record after execution of the PassManager.
        """
        return TransformationRecord._from_pytr(self._to.record)
