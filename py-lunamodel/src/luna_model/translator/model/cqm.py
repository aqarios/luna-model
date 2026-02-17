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

# type: ignore[reportPossiblyUnboundVariable]
from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import ConstrainedQuadraticModel
    from dimod import lp as dimod_lp

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False


class CqmTranslator:
    r"""Translator for Constrained Quadratic Model format.

    Converts between LunaModel and ConstrainedQuadraticModel (CQM) format.

    Requires the ``dimod`` extra.

    Examples
    --------
    >>> from dimod import ConstrainedQuadraticModel, Binary
    >>> from luna_model.translator import CqmTranslator
    >>> cqm = ConstrainedQuadraticModel()
    >>> x = Binary("x")
    >>> y = Binary("y")
    >>> cqm.set_objective(-x - y + 2 * x * y)
    >>> _ = cqm.add_constraint(x + y <= 1, label="c1")
    >>> model = CqmTranslator.to_lm(cqm, name="my_model")
    >>> print(model)
    Model: my_model
    Minimize
      2 * x * y - x - y
    Subject To
      c1: x + y <= 1
    Binary
      x y

    >>> from luna_model import Model, Vtype
    >>> from luna_model.translator import CqmTranslator
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y - 2 * x + y
    >>> model.constraints += x + y <= 1
    >>> cqm = CqmTranslator.from_lm(model)
    >>> print(cqm)
    Constrained quadratic model: 2 variables, 1 constraints, 5 biases
    <BLANKLINE>
    Objective
      -2*Binary('x') + Binary('y') + Binary('x')*Binary('y')
    <BLANKLINE>
    Constraints
      c0: Binary('x') + Binary('y') <= 1.0
    <BLANKLINE>
    Bounds
    <BLANKLINE>
    """

    @staticmethod
    def to_lm(cqm: "ConstrainedQuadraticModel", *, name: str | None = None) -> Model:
        """Convert D-Wave CQM to LunaModel.

        Converts a D-Wave ConstrainedQuadraticModel to a LunaModel Model.

        Parameters
        ----------
        cqm : ConstrainedQuadraticModel
            D-Wave CQM to convert.
        name : str, optional
            Name for the resulting model. If None, uses the CQM's name or
            a default name.

        Returns
        -------
        Model
            LunaModel representation with objective and constraints matching
            the CQM.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.
        TypeError
            If ``cqm`` is not a ConstrainedQuadraticModel.

        Examples
        --------
        >>> from dimod import ConstrainedQuadraticModel, Binary, Integer
        >>> from luna_model.translator import CqmTranslator
        >>> cqm = ConstrainedQuadraticModel()
        >>> x = Binary("x")
        >>> y = Integer("y", lower_bound=0, upper_bound=10)
        >>> cqm.set_objective(x + 2 * y)
        >>> _ = cqm.add_constraint(x + y >= 3, label="min_sum")
        >>> model = CqmTranslator.to_lm(cqm, name="example")
        >>> print(model.objective)
        x + 2 y
        >>> print(len(model.constraints))
        1

        Notes
        -----
        The conversion preserves all variable types, bounds, constraints, and
        the objective function from the CQM.
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        if not isinstance(cqm, ConstrainedQuadraticModel):
            msg = f"Expected cqm to be of type CQM, received: {type(cqm)}"
            raise TypeError(msg)
        cqm_lp = dimod_lp.dumps(cqm)
        model = Model._from_pym(PyLpTranslator.to_lm(cqm_lp))
        if name is not None:
            model.name = name
        return model

    @staticmethod
    def from_lm(model: Model) -> "ConstrainedQuadraticModel":
        """Convert LunaModel to D-Wave CQM.

        Converts a LunaModel Model to a D-Wave ConstrainedQuadraticModel.

        Parameters
        ----------
        model : Model
            The model to convert. Should have:
            - At most quadratic objective and constraints

        Returns
        -------
        ConstrainedQuadraticModel
            D-Wave CQM ready for use with D-Wave hybrid solvers.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.
        TranslationError
            If the model contains constructs not supported by CQM format.

        Examples
        --------
        >>> from luna_model import Model, Variable, Vtype
        >>> from luna_model.translator import CqmTranslator
        >>> model = Model(name="knapsack")
        >>> x = [model.add_variable(f"x{i}", vtype=Vtype.BINARY) for i in range(5)]
        >>> values = [10, 15, 20, 25, 30]
        >>> weights = [2, 3, 4, 5, 6]
        >>> # Maximize value
        >>> model.objective = sum(v * x[i] for i, v in enumerate(values))
        >>> # Weight constraint
        >>> model.constraints += sum(w * x[i] for i, w in enumerate(weights)) <= 10
        >>> cqm = CqmTranslator.from_lm(model)
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        return dimod_lp.loads(PyLpTranslator.from_lm(model._m))
