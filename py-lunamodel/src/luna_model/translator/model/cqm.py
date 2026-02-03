"""Constrained Quadratic Model translator for LunaModel.

This module provides translation between LunaModel's internal representation
and Constrained Quadratic Model (CQM) format for constrained optimization.
"""

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

    A CQM represents a constrained optimization problem:

    .. math::
        \\min \\sum_{i} h_i x_i + \\sum_{i<j} J_{ij} x_i x_j

    subject to linear and quadratic constraints with binary, integer, and
    continuous variables.

    Requires the ``dimod`` package.

    Examples
    --------
    >>> from dimod import ConstrainedQuadraticModel, Binary
    >>> from luna_model.translator import CqmTranslator
    >>> cqm = ConstrainedQuadraticModel()
    >>> x = Binary("x")
    >>> y = Binary("y")
    >>> cqm.set_objective(-x - y + 2 * x * y)
    >>> cqm.add_constraint(x + y <= 1, label="c1")
    >>> model = CqmTranslator.to_lm(cqm, name="my_model")

    >>> from luna_model import Model, Vtype
    >>> from luna_model.translator import CqmTranslator
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y - 2 * x + y
    >>> model.constraints += x + y <= 1
    >>> cqm = CqmTranslator.from_lm(model)

    Notes
    -----
    The translator uses LP format as an intermediate representation.

    See Also
    --------
    BqmTranslator : Binary Quadratic Model format
    LpTranslator : LP file format translator
    DwaveTranslator : D-Wave solution translator
    """

    @staticmethod
    def to_lm(cqm: "ConstrainedQuadraticModel", *, name: str | None = None) -> Model:
        """Convert D-Wave CQM to LunaModel.

        Converts a D-Wave Ocean SDK ConstrainedQuadraticModel to a LunaModel Model.

        Parameters
        ----------
        cqm : ConstrainedQuadraticModel
            D-Wave CQM to convert.
        name : str | None, optional
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
        >>> cqm.add_constraint(x + y >= 3, label="min_sum")
        >>> model = CqmTranslator.to_lm(cqm, name="example")
        >>> print(model.objective)
        >>> print(len(model.constraints))

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

        Converts a LunaModel Model to a D-Wave Ocean SDK ConstrainedQuadraticModel.

        Parameters
        ----------
        model : Model
            The model to convert. Should have:
            - At most quadratic objective and constraints
            - Supported variable types (binary, integer, continuous)

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
        >>> # Submit to D-Wave Leap hybrid solver
        >>> # from dwave.system import LeapHybridCQMSampler
        >>> # sampler = LeapHybridCQMSampler()
        >>> # sampleset = sampler.sample_cqm(cqm, label="Knapsack")

        Notes
        -----
        The translator uses LP format as an intermediate representation,
        ensuring all model components are properly converted to CQM format.
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        return dimod_lp.loads(PyLpTranslator.from_lm(model._m))
