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
    def variable_names(self):
        """The name of the variables in the same order as in the QUBO matrix.

        Returns
        -------
        list[Variable]
            The variable names in the order they appear in the QUBO.
        """
        return

    @dispatched
    @property
    def name(self):
        """The name of the model the QUBO matrix was generated from.

        Returns
        -------
        str
            The model name.
        """
        return

    @dispatched
    @property
    def vtype(self):
        """The type of the model variables. Can be `Binary` or `Spin`.

        Returns
        -------
        Vtype
            The variable type.
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
    >>> from luna_quantum import QuboTranslator, Vtype
    >>> q = np.array([[1.0, -1.0], [-1.0, 2.0]])

    Create a model from a matrix:

    >>> model = QuboTranslator.to_aq(q, offset=4.2, name="qubo_model", vtype=Vtype.Binary)

    Convert it back to a dense matrix:

    >>> recovered = QuboTranslator.from_aq(model)
    >>> assert np.allclose(q, recovered.matrix)
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

        Raises
        ------
        TranslationError
            Generally if the translation fails. Might be specified by the following
            error.
        VariableNamesError
            If a list of variable names is provided but contains duplicates or has an
            incorrect length.
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
            the model's objective. This object also includes the `variable_ordering`
            as well as the `offset` of the original model.

        Raises
        ------
        TranslationError
            Generally if the translation fails. Might be specified by one of the
            four following errors.
        ModelNotQuadraticError
            If the objective contains higher-order (non-quadratic) terms.
        ModelNotUnconstrainedError
            If the model contains any constraints.
        ModelSenseNotMinimizeError
            If the model's optimization sense is 'maximize'.
        ModelVtypeError
            If the model contains different vtypes or vtypes other than binary and
            spin.
        """
        return model
