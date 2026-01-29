from luna_model._lm import PyChangeSensePass
from luna_model.model.model import Sense
from luna_model.transformation.transform import ConcreteTransformationPass


class ChangeSensePass(ConcreteTransformationPass):
    """A transformation pass to change the model's Sense to a target Sense."""

    def __init__(self, sense: Sense) -> None:
        """Transform the model's Sense to a target Sense.

        Parameters
        ----------
        sense : Sense
            The target sense of the model after calling the `run` method on it.
        """
        super().__init__(base=PyChangeSensePass(sense._val))

    @property
    def sense(self) -> Sense:
        """Get the specified target sense of this pass."""
        return Sense._from_pysense(self._csp.sense)
