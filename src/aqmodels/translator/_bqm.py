from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class BqmTranslator:
    """
    Utility class for converting between dimod.BianryQuadraticModel (BQM) and symbolic
    models.

    `MatrixTranslator` provides methods to:
    - Convert a BQM into a symbolic `Model`
    - Convert a `Model` (with quadratic objective) into a BQM

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on BQMs.

    Examples
    --------
    >>> import dimod
    >>> import numpy as np
    >>> from aqmodels import MatrixTranslator, Vtype
    >>> bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")

    Create a model from a matrix:

    >>> model = BqmTranslator.to_aq(bqm, name="bqm_model")

    Convert it back to a dense matrix:

    >>> recovered = BqmTranslator.from_aq(model)
    """

    @staticmethod
    @dispatched
    def to_aq(bqm, name):
        """
        Convert a BQM into a symbolic `Model`.

        Parameters
        ----------
        bqm : BinaryQuadraticModel
            The BQM.
        name : str, optional
            An optional name to assign to the resulting model.

        Returns
        -------
        Model
            A symbolic model representing the given BQM.
        """
        return bqm, name

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
        BinaryQuadraticModel
            The resulting BQM.

        Raises
        ------
        ModelNotQuadraticError
            If the objective contains higher-order (non-quadratic) terms.
        ModelNotUnconstrainedError
            If the model contains any constraints.
        ModelVtypeError
            If the model contains different vtypes or vtypes other than binary and
            spin.
        """
        return model
