from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class Qubo:
    @dispatched
    @property
    def offset(self):
        """The constant offset of the original model passed to the QuboTranslator.

        Returns
        -------
        float
            The constant offset of the model.
        """
        return self

    @dispatched
    @property
    def matrix(self):
        """The actual QUBO matrix.

        Returns
        -------
        NDArray
            A square NumPy array representing the QUBO matrix derived from
            the model's objective.
        """
        return self

    @dispatched
    @property
    def variable_ordering(self):
        """The order in which the variables appear in the QUBO matrix.

        Returns
        -------
        list[Variable]
            The variables in the order they appear in the QUBO.
        """
        return


@export("translator", "top")
class QuboTranslator:
    """
    Utility class for converting between dense QUBO matrices and symbolic models.

    `QuboTranslator` provides methods to:
    - Convert a NumPy-style QUBO matrix into a symbolic `Model`
    - Convert a `Model` (with quadratic objective) into a dense QUBO matrix

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on matrix-based problem definitions.

    Examples
    --------
    >>> import numpy as np
    >>> from aqmodels import QuboTranslator, Vtype
    >>> q = np.array([[1.0, -1.0], [-1.0, 2.0]])

    Create a model from a matrix:

    >>> model = QuboTranslator.to_aq(q, name="qubo_model", vtype=Vtype.Binary)

    Convert it back to a dense matrix:

    >>> recovered = QuboTranslator.from_aq(model)
    >>> assert np.allclose(q, recovered)
    """

    @staticmethod
    @dispatched
    def to_aq(qubo, name, vtype):
        """
        Convert a dense QUBO matrix into a symbolic `Model`.

        Parameters
        ----------
        qubo : NDArray
            A square 2D NumPy array representing the QUBO matrix.
            Diagonal entries correspond to linear coefficients;
            off-diagonal entries represent pairwise quadratic terms.
        name : str, optional
            An optional name to assign to the resulting model.
        vtype : Vtype, optional
            The variable type to assign to all variables (e.g. Binary, Spin).

        Returns
        -------
        Model
            A symbolic model representing the given QUBO structure.
        """
        return qubo, name, vtype

    @staticmethod
    @dispatched
    def from_aq(model):
        """
        Convert a symbolic model to a dense QUBO matrix representation.

        Parameters
        ----------
        model : Model
            The symbolic model to convert. The objective must be quadratic-only
            and unconstrained.

        Returns
        -------
        Qubo
            An object representing a QUBO with additional information additional
            to the square NumPy array representing the QUBO matrix derived from
            the model's objective. This object also include the `variable_ordering`
            as well as the `offset` of the original model.

        Raises
        ------
        TranslationError
            Generally, if the translation fails. Might be specified by one of the
            two following errors.
        ModelNotQuadraticError
            If the objective contains higher-order (non-quadratic) terms.
        ModelNotUnconstrainedError
            If the model contains any constraints.
        """
        return model
