from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class MatrixTranslator:
    """
    Utility class for converting between dense QUBO matrices and symbolic models.

    `MatrixTranslator` provides methods to:
    - Convert a NumPy-style QUBO matrix into a symbolic `Model`
    - Convert a `Model` (with quadratic objective) into a dense QUBO matrix

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on matrix-based problem definitions.

    Examples
    --------
    >>> import numpy as np
    >>> from aqmodels import MatrixTranslator, Vtype
    >>> q = np.array([[1.0, -1.0], [-1.0, 2.0]])

    Create a model from a matrix:

    >>> model = MatrixTranslator.to_aq(q, name="qubo_model", vtype=Vtype.Binary)

    Convert it back to a dense matrix:

    >>> recovered = MatrixTranslator.from_aq(model)
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
        NDArray
            A square NumPy array representing the QUBO matrix derived from
            the model's objective.

        Raises
        ------
        ModelNotQuadraticError
            If the objective contains higher-order (non-quadratic) terms.
        ModelNotUnconstrainedError
            If the model contains any constraints.
        """
        return model
