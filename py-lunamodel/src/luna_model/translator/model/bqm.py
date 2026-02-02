"""D-Wave Binary Quadratic Model translator for LunaModel.

This module provides translation between LunaModel's internal representation
and D-Wave Ocean SDK's Binary Quadratic Model (BQM) format, used by D-Wave
quantum annealers and hybrid solvers.
"""

# type: ignore[reportPossiblyUnboundVariable]
import numpy as np

from luna_model._lm import PyBqmTranslator
from luna_model.model.model import Model
from luna_model.variable.vtype import Vtype

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import BinaryQuadraticModel

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False


class BqmTranslator:
    """Translator for D-Wave Binary Quadratic Model format.

    BqmTranslator provides static methods to convert between LunaModel's internal
    Model representation and D-Wave Ocean SDK's BinaryQuadraticModel (BQM) format.
    BQMs are used to represent optimization problems for D-Wave quantum annealers
    and hybrid solvers.

    A BQM represents an objective function of the form:
        E(x) = sum_i h_i * x_i + sum_{i,j} J_{i,j} * x_i * x_j + offset

    where x is a vector of binary or spin variables, h are linear coefficients,
    J are quadratic coefficients, and offset is a constant term.

    The BQM format supports:
    - Binary (0/1) or spin (-1/+1) variables
    - Linear and quadratic terms
    - No constraints (unconstrained problems)

    Requires the ``dimod`` package from D-Wave Ocean SDK.

    Examples
    --------
    Convert D-Wave BQM to LunaModel:

    >>> from dimod import BinaryQuadraticModel
    >>> from luna_model.translator import BqmTranslator
    >>> # Create a BQM
    >>> bqm = BinaryQuadraticModel({"x": -1, "y": -1}, {("x", "y"): 2}, 0.0, "BINARY")
    >>> model = BqmTranslator.to_lm(bqm, name="my_model")

    Convert LunaModel to D-Wave BQM:

    >>> from luna_model import Model
    >>> model = Model()
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = -x - y + 2 * x * y
    >>> bqm = BqmTranslator.from_lm(model)
    >>> # Use with D-Wave solvers
    >>> # sampler = DWaveSampler()
    >>> # response = sampler.sample(bqm)

    Notes
    -----
    BQMs are specifically designed for D-Wave systems but can also be solved
    using classical algorithms like simulated annealing or tabu search through
    the D-Wave Ocean SDK.

    The model must be unconstrained and have at most quadratic terms. Use
    transformations to convert constrained or higher-order models first.

    See Also
    --------
    QuboTranslator : QUBO matrix format translator
    CqmTranslator : D-Wave Constrained Quadratic Model format
    DwaveTranslator : D-Wave solution translator
    """

    @staticmethod
    def to_lm(bqm: "BinaryQuadraticModel", *, name: str | None = None) -> Model:
        """Convert D-Wave BQM to LunaModel.

        Converts a D-Wave Ocean SDK BinaryQuadraticModel to a LunaModel Model.

        Parameters
        ----------
        bqm : BinaryQuadraticModel
            D-Wave BQM to convert. All variable names must be strings.
        name : str | None, optional
            Name for the resulting model. If None, uses default name.

        Returns
        -------
        Model
            LunaModel representation with objective function matching the BQM.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.
        TypeError
            If ``bqm`` is not a BinaryQuadraticModel or if variable names
            are not strings.

        Examples
        --------
        >>> from dimod import BinaryQuadraticModel
        >>> from luna_model.translator import BqmTranslator
        >>> bqm = BinaryQuadraticModel({"x": -1, "y": 2}, {("x", "y"): 1}, 0.5, "BINARY")
        >>> model = BqmTranslator.to_lm(bqm, name="example")
        >>> print(model.objective)

        Notes
        -----
        The translator preserves variable types (BINARY or SPIN) and the
        constant offset term from the BQM.
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the BqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        if not isinstance(bqm, BinaryQuadraticModel):
            msg = f"Expected bqm to be of type BQM, received: {type(bqm)}"
            raise TypeError(msg)
        bqm_vars_ser = bqm.variables.to_serializable()
        for v in bqm_vars_ser:
            if not isinstance(v, str):
                msg = f"All BQM variables have to be of type str, received: {type(v)}"
                raise TypeError(msg)
        variables = np.array(bqm_vars_ser)
        vars_pos = {var: i for i, var in enumerate(variables)}

        linears = []
        linear_indices = []
        for var, val in bqm.linear.items():
            linears.append(val)
            linear_indices.append(vars_pos[var])
        quads = []
        quad_row = []
        quad_col = []
        for (var1, var2), val in bqm.quadratic.items():
            quads.append(val)
            quad_row.append(vars_pos[var1])
            quad_col.append(vars_pos[var2])

        vartype = Vtype(bqm.vartype.name.title())
        offset = float(bqm.offset)
        return Model._from_pym(
            PyBqmTranslator.to_lm(
                vars=variables,
                vtype=vartype._val,
                offset=offset,
                linears=np.array(linears, dtype=np.float64),
                linear_indices=np.array(linear_indices, dtype=np.uint64),
                quads=np.array(quads, dtype=np.float64),
                quads_rows=np.array(quad_row, dtype=np.uint64),
                quads_cols=np.array(quad_col, dtype=np.uint64),
                name=name,
            )
        )

    @staticmethod
    def from_lm(model: Model) -> "BinaryQuadraticModel":
        """Convert LunaModel to D-Wave BQM.

        Converts a LunaModel Model to a D-Wave Ocean SDK BinaryQuadraticModel.

        Parameters
        ----------
        model : Model
            The model to convert. Must be:
            - Unconstrained (no constraints)
            - Binary or spin variables only
            - At most quadratic (degree ≤ 2)

        Returns
        -------
        BinaryQuadraticModel
            D-Wave BQM ready for use with D-Wave solvers.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.
        TranslationError
            If the model has constraints, non-binary/spin variables, or
            higher-order terms.

        Examples
        --------
        >>> from luna_model import Model
        >>> from luna_model.translator import BqmTranslator
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.objective = x * y - 2 * x + y
        >>> bqm = BqmTranslator.from_lm(model)
        >>> # Submit to D-Wave
        >>> # from dwave.system import DWaveSampler
        >>> # sampler = DWaveSampler()
        >>> # response = sampler.sample(bqm, num_reads=100)

        Notes
        -----
        If your model doesn't meet BQM requirements, apply transformations first:

        >>> from luna_model.transformation import PassManager
        >>> # Add necessary transformations
        >>> # Then translate
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the BqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        offset, linear, quad, rows, cols, vtype, variables = PyBqmTranslator.from_lm(model._m)
        vtype = Vtype._from_pyvtype(vtype).value.upper()
        return BinaryQuadraticModel.from_numpy_vectors(
            linear,
            (rows, cols, quad),
            offset,
            vtype,
            variable_order=variables,
        )
