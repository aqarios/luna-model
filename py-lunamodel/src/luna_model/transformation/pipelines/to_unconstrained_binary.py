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

from typing import Self

from luna_model._lm import PyToUnconstrainedBinaryPipeline
from luna_model.wrapper import wraps


class ToUnconstrainedBinaryPipeline(PyToUnconstrainedBinaryPipeline):
    """Convert a model to an unconstrained binary model.

    This pipeline transforms any model with constraints to an unconstrained binary model.
    It allows the input model to contain binary, spin or integer variables. Spin and integer
    variables are automatically converted to a binary representation. If the input model has
    linear constraints they are added to the model's objective as quadratic penalties scaled by
    the the maximum bias of the input model's objective times the `penalty_scaling` paramter.

    Notes
    -----
    If the model's constraints are not linear, an error is raised.

    Known Limitations
    -----------------
    If the constraints contain real-valued coefficients, the optimal solution may not be reached,
    as the transformation pipeline only creates integer-valued slack variables, not real-valued
    slack variables.

    Paramters
    ---------
    penalty_scaling : float
        The factor used to scale the quadratic penalties with, by default 10.

    Raises
    ------
    AnalysisPassError
        If the model's constraints are not all linear.

    Example
    -------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager, pipelines
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.SPIN)
    >>> z = model.add_variable("z", vtype=Vtype.INTEGER, lower=0, upper=12)
    >>> model.objective = x + y + z
    >>> model.constraints += x + y + z <= 3, "c0"
    >>> model.constraints += x - y - z >= 0, "c1"
    >>> model.constraints += x + y == 2, "c2"
    >>> pm = PassManager([pipelines.ToUnconstrainedBinaryPipeline()])
    >>> ir = pm.run(model)
    """

    def __new__(cls, penalty_scaling: float = 10.0) -> Self:
        """Create the unconstrained-binary conversion pipeline.

        Parameters
        ----------
        penalty_scaling : float, optional
            Multiplier used when adding quadratic penalty terms for converted
            constraints. Defaults to ``10.0``.

        Returns
        -------
        Self
            New pipeline instance.
        """
        return super().__new__(cls, penalty_scaling=penalty_scaling)

    @wraps()
    def __str__(self) -> str:
        """Human readable string."""
        raise NotImplementedError
