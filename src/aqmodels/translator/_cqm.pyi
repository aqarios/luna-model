from dimod import ConstrainedQuadraticModel

from aqmodels import Model

class CqmTranslator:
    """
    Utility class for converting between dimod.BinaryQuadraticModel (CQM) and symbolic
    models.

    `CqmTranslator` provides methods to:
    - Convert a CQM into a symbolic `Model`
    - Convert a `Model` (with quadratic objective) into a CQM

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on CQMs.

    Examples
    --------
    >>> import dimod
    >>> import numpy as np
    >>> from luna_quantum import CqmTranslator, Vtype
    >>> bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")

    Create a model from a matrix:

    >>> model = CqmTranslator.to_aq(bqm, name="bqm_model")

    Convert it back to a dense matrix:

    >>> recovered = CqmTranslator.from_aq(model)
    """
    @staticmethod
    def to_aq(cqm: ConstrainedQuadraticModel) -> Model:
        """
        Convert a CQM into a symbolic `Model`.

        Parameters
        ----------
        cqm : ConstrainedQuadraticModel
            The CQM.

        Returns
        -------
        Model
            A symbolic model representing the given CQM.

        Raises
        ------
        TypeError
            If `cqm` is not of type `ConstrainedQuadraticModel`.
        TranslationError
            If the translation fails for some reason.
        """
        ...
    @staticmethod
    def from_aq(model: Model) -> ConstrainedQuadraticModel:
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
            The resulting CQM.

        Raises
        ------
        TranslationError
            If the translation fails for some reason.
        """
        ...
