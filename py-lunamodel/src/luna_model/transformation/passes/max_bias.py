from luna_model._lm import PyMaxBiasAnalysis
from luna_model.transformation.analysis import ConcreteAnalysisPass


class MaxBiasAnalysis(ConcreteAnalysisPass):
    """An analysis pass computing the maximum bias contained in the model."""

    def __init__(self) -> None:
        """Transform the model's Sense to a target Sense.

        Parameters
        ----------
        sense : Sense
            The target sense of the model after calling the `run` method on it.
        """
        super().__init__(base=PyMaxBiasAnalysis())
