from typing import overload

from dimod import BinaryQuadraticModel

from aqmodels import Model

class BqmTranslator:
    """
    Utility class for converting between dimod.BinaryQuadraticModel (BQM) and symbolic
    models.

    `BqmTranslator` provides methods to:
    - Convert a BQM into a symbolic `Model`
    - Convert a `Model` (with quadratic objective) into a BQM

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on BQMs.

    Examples
    --------
    >>> import dimod
    >>> import numpy as np
    >>> from luna_quantum import BqmTranslator, Vtype
    >>> bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")

    Create a model from a matrix:

    >>> model = BqmTranslator.to_aq(bqm, name="bqm_model")

    Convert it back to a dense matrix:

    >>> recovered = BqmTranslator.from_aq(model)
    """
    @overload
    @staticmethod
    def to_aq(bqm: BinaryQuadraticModel) -> Model: ...
    @overload
    @staticmethod
    def to_aq(bqm: BinaryQuadraticModel, *, name: str) -> Model:
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
        ...
    @staticmethod
    def from_aq(model: Model) -> BinaryQuadraticModel:
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
        ...
