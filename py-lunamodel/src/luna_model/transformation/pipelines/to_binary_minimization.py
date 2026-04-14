from __future__ import annotations

from typing import Self

from luna_model._lm import PyToBinaryMinimizationPipeline
from luna_model.wrapper import wraps


class ToBinaryMinimizationPipeline(PyToBinaryMinimizationPipeline):
    """Convert a model to an binary model and minimize.

    This pipeline transforms any model with integers and spins to an binary model.

    Raises
    ------
    AnalysisPassError
        If the model has real valued variables.

    Example
    -------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager, pipelines
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.SPIN)
    >>> z = model.add_variable("z", vtype=Vtype.INTEGER, lower=0, upper=12)
    >>> model.objective = x + y + z
    >>> pm = PassManager([pipelines.ToBinaryMinimizationPipeline()])
    >>> ir = pm.run(model)
    """

    def __new__(cls) -> Self:
        """Todo."""
        return super().__new__(cls)

    @wraps()
    def __str__(self) -> str:
        """Human readable string."""
        raise NotImplementedError
