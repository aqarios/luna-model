from pathlib import Path
from typing import Any, overload

from dimod import BinaryQuadraticModel, ConstrainedQuadraticModel, SampleSet
from numpy.typing import NDArray
from pyscipopt import Model as SciModel
from qiskit.primitives import PrimitiveResult, PubResult
from qiskit_optimization import QuadraticProgram

from ._core import Environment, Model, Solution, Timing, Vtype

class ZibTranslator:
    """
    Utility class for converting between a Zib solution and our solution format.

    `ZibTranslator` provides methods to:

        - Convert a Zib-style solution into our solution `Solution`.

    The conversions are especially required when interacting with external zib
    solvers/samplers or libraries that operate on zib-based problem-solving/sampling.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> from pyscipopt import Model
    >>> model = Model()
    >>> model.readProblem("./path/to/my/model.lp")
    >>> model.optimize()
    >>> aqs = lq.translator.ZibTranslator.to_aq(model)
    """

    @overload
    @staticmethod
    def to_aq(model: SciModel) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(model: SciModel, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(model: SciModel, *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(model: SciModel, timing: Timing, *, env: Environment) -> Solution: ...
    @staticmethod
    def to_aq(
        model: SciModel, timing: Timing | None = ..., *, env: Environment | None = ...
    ) -> Solution:
        """
        Extract a solution from a ZIB model.

        Parameters
        ----------
        model : pyscipopt.Model
            The Model that ran the optimization.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of
            the two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

class Qubo:
    """The result of the QuboTranslator.

    A wrapper around qubo matrices that holds all relevant metadata,
    e.g., the model offset.
    """

    @property
    def matrix(self, /) -> NDArray:
        """
        The actual QUBO matrix.

        Returns
        -------
        NDArray
            A square NumPy array representing the QUBO matrix derived from
            the model's objective.
        """
        ...

    @property
    def variable_names(self, /) -> list[str]:
        """
        The name of the variables in the same order as in the QUBO matrix.

        Returns
        -------
        list[Variable]
            The variable names in the order they appear in the QUBO.
        """
        ...

    @property
    def name(self, /) -> str:
        """
        The name of the model the QUBO matrix was generated from.

        Returns
        -------
        str
            The model name.
        """
        ...

    @property
    def offset(self, /) -> float:
        """
        The constant offset of the original model passed to the QuboTranslator.

        Returns
        -------
        float
            The constant offset of the model.
        """
        ...

    @property
    def vtype(self, /) -> Vtype:
        """
        The type of the model variables. Can be `Binary` or `Spin`.

        Returns
        -------
        Vtype
            The variable type.
        """
        ...

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
    >>> from luna_quantum.translator import QuboTranslator, Vtype
    >>> q = np.array([[1.0, -1.0], [-1.0, 2.0]])

    Create a model from a matrix:

    >>> model = QuboTranslator.to_aq(
    ...     q, offset=4.2, name="qubo_model", vtype=Vtype.Binary
    ... )

    Convert it back to a dense matrix:

    >>> recovered = QuboTranslator.from_aq(model)
    >>> assert np.allclose(q, recovered.matrix)
    """

    @staticmethod
    def to_aq(
        qubo: NDArray,
        *,
        offset: float | None = ...,
        variable_names: list[str] | None = ...,
        name: str | None = ...,
        vtype: Vtype | None = ...,
    ) -> Model:
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
        ...

    @staticmethod
    def from_aq(model: Model) -> Qubo:
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
            An object representing a QUBO with information additional to the square
            NumPy array representing the QUBO matrix derived from the model's objective.
            This object also includes the `variable_ordering` as well as the `offset`
            of the original model.

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

class QctrlTranslator:
    """
    Utility class for converting between a QCTRL solution and our solution format.

    `QctrlTranslator` provides methods to:
    - Convert a Qctrl-style solution into our solution `Solution`.

    The conversions are especially required when interacting with external qctrl
    solvers/samplers or libraries that operate on qctrl-based problem-solving/sampling.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> ...
    >>> qctrl_result = ...
    >>> aqs = lq.translator.QctrlTranslator.to_aq(qctrl_result)
    """

    @overload
    @staticmethod
    def to_aq(result: dict[str, Any]) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any], timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any], timing: Timing, *, env: Environment
    ) -> Solution: ...
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
        *,
        env: Environment | None = ...,
    ) -> Solution:
        """
        Convert a QCTRL result to our solution format.

        Parameters
        ----------
        result : dict[str, Any]
            The qctrl result as a dictionary.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of
            the two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

class NumpyTranslator:
    """Translate between numpy arrays and our solution format.

    Utility class for converting between a result consisting of numpy arrays and our
    solution format.

    `NumpyTranslator` provides methods to:
    - Convert a numpy-array result into our solution `Solution`.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> from numpy.typing import NDArray
    >>> result: NDArray = ...
    >>> energies: NDArray = ...
    >>> aqs = lq.translator.NumpyTranslator.to_aq(result, energies)
    """

    @overload
    @staticmethod
    def to_aq(result: NDArray, energies: NDArray) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: NDArray, energies: NDArray, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: NDArray, energies: NDArray, *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: NDArray, energies: NDArray, timing: Timing, *, env: Environment
    ) -> Solution: ...
    @staticmethod
    def to_aq(
        result: NDArray,
        energies: NDArray,
        timing: Timing | None = ...,
        *,
        env: Environment | None = ...,
    ) -> Solution:
        """Convert a solution in the format of numpy arrays to our solution format.

        Note that the optimization sense is always assumed to be minimization.

        Parameters
        ----------
        result : NDArray
            The samples as a 2D array where each row corresponds to one sample.
        energies : NDArray
            The energies of the single samples as a 1D array.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of
            the two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

class LpTranslator:
    """
    Utility class for converting between LP files and symbolic models.

    `LpTranslator` provides methods to:
    - Convert an LP file into a symbolic `Model`
    - Convert a `Model` into an Lp file.

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on LP-based problem definitions.

    Examples
    --------
    >>> from pathlib import Path
    >>> from luna_quantum.translator import LpTranslator
    >>> lp_filepath = Path("path/to/the/lp_file")

    >>> model = LpTranslator.to_aq(lp_filepath)

    Convert it back to an LP file:

    >>> recovered = LpTranslator.to_file(model)
    """

    @overload
    @staticmethod
    def to_aq(file: Path) -> Model: ...
    @overload
    @staticmethod
    def to_aq(file: str) -> Model: ...
    @staticmethod
    def to_aq(file: str | Path) -> Model:
        """
        Convert an LP file into a symbolic `Model`.

        Parameters
        ----------
        file: Path | String
            An LP file representing a symbolic model, either given as a
            Path object to the LP file or its contents as a string.
            If you pass the path as a string, it will be interpreted as a
            model and thus fail to be parsed to a Model.

        Returns
        -------
        Model
            A symbolic model representing the given lp file structure.

        Raises
        ------
        TypeError
            If `file` is not of type `str` or `Path`.
        TranslationError
            If the translation fails for a different reason.
        """
        ...

    @overload
    @staticmethod
    def from_aq(model: Model) -> str: ...
    @overload
    @staticmethod
    def from_aq(model: Model, *, filepath: Path) -> None: ...
    @staticmethod
    def from_aq(model: Model, *, filepath: Path | None = ...) -> None:
        """
        Convert a symbolic model to an LP file representation.

        Parameters
        ----------
        model : Model
            The symbolic model to convert.
        file : Path, optional
            The filepath to write the model contents to.

        Returns
        -------
        str
            If no file to write to is given, i.e., the file is None.

        Raises
        ------
        TranslationError
            If the translation fails for some reason.
        """
        ...

class IbmTranslator:
    """Utility class for converting between an IBM solution and our solution format.

    `IbmTranslator` provides methods to:
    - Convert an IBM-style solution into our solution `Solution`.

    The conversions are especially required when interacting with external ibm
    solvers/samplers oe libraries that operate on ibm-based problem-solving/sampling.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> ...
    >>> ibm_result = ...
    >>> aqs = lq.translator.IbmTranslator.to_aq(ibm_result)
    """

    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult], quadratic_program: QuadraticProgram
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        *,
        env: Environment,
    ) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing,
        *,
        env: Environment,
    ) -> Solution: ...
    @staticmethod
    def to_aq(
        result: PrimitiveResult[PubResult],
        quadratic_program: QuadraticProgram,
        timing: Timing | None = ...,
        *,
        env: Environment | None = ...,
    ) -> Solution:
        """
        Convert an IBM solution to our solution format.

        Parameters
        ----------
        result : PrimitiveResult[PubResult]
            The ibm result.
        quadratic_program : QuadraticProgram
            The quadratic program defining the optimization problem.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of
            the two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

class DwaveTranslator:
    """Utility class for converting between a DWAVE solution and our solution format.

    `DWaveSolutionTranslator` provides methods to:
    - Convert a dimod-style solution into our solution `Solution`.

    The conversions are especially required when interacting with external dwave/dimod
    solvers/samplers or libraries that operate on dwave/dimod-based problem-solving/
    sampling.

    Examples
    --------
    >>> import dimod
    >>> import luna_quantum as lq
    >>> dwave_sampleset = ...
    >>> aqs = lq.translator.DwaveTranslator.to_aq(dwave_sampleset)
    """

    @overload
    @staticmethod
    def to_aq(sample_set: SampleSet) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(sample_set: SampleSet, timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(sample_set: SampleSet, *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        sample_set: SampleSet, timing: Timing, *, env: Environment
    ) -> Solution: ...
    @staticmethod
    def to_aq(
        sample_set: SampleSet,
        timing: Timing | None = ...,
        *,
        env: Environment | None = ...,
    ) -> Solution:
        """
        Convert a DWave SampleSet to our solution format.

        Parameters
        ----------
        sample_set : SampleSet
            The SampleSet returned by a DWave solver.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of
            the two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        SampleUnexpectedVariableError
            If the sample_set contains variables that are not contained in the passed
            environment.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

class CqmTranslator:
    """CQM to AQM translator.

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
    >>> from luna_quantum.translator import CqmTranslator, Vtype
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

class BqmTranslator:
    """BQM to AQM translator.

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
    >>> from luna_quantum.translator import BqmTranslator, Vtype
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
    def to_aq(bqm: BinaryQuadraticModel, *, name: str) -> Model: ...
    @staticmethod
    def to_aq(bqm: BinaryQuadraticModel, *, name: str | None = ...) -> Model:
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

class AwsTranslator:
    """
    Utility class for converting between an AWS result and our solution format.

    `AwsTranslator` provides methods to:
    - Convert an AWS-style result into our solution `Solution`.

    The conversions are especially required when interacting with external aws
    solvers/samplers or libraries that operate on aws-based problem-solving/sampling.

    Examples
    --------
    >>> import luna_quantum as lq
    >>> aws_result = ...
    >>> aqs = lq.translator.AwsTranslator.to_aq(aws_result)
    """

    @overload
    @staticmethod
    def to_aq(result: dict[str, Any]) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any], timing: Timing) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(result: dict[str, Any], *, env: Environment) -> Solution: ...
    @overload
    @staticmethod
    def to_aq(
        result: dict[str, Any], timing: Timing, *, env: Environment
    ) -> Solution: ...
    @staticmethod
    def to_aq(
        result: dict[str, Any],
        timing: Timing | None = ...,
        *,
        env: Environment | None = ...,
    ) -> Solution:
        """
        Convert an AWS Braket result to our solution format.

        Parameters
        ----------
        result : dict[str, Any]
            The aws braket result.
        timing : Timing, optional
            The timing object produced while generating the result.
        env : Environment, optional
            The environment of the model for which the result is produced.

        Raises
        ------
        NoActiveEnvironmentFoundError
            If no environment is passed to the method or available from the context.
        SolutionTranslationError
            Generally if the solution translation fails. Might be specified by one of
            the two following errors.
        SampleIncorrectLengthError
            If a solution's sample has a different number of variables than the model
            environment passed to the translator.
        ModelVtypeError
            If the result's variable types are incompatible with the model environment's
            variable types.
        """
        ...

__all__ = [
    "AwsTranslator",
    "BqmTranslator",
    "CqmTranslator",
    "DwaveTranslator",
    "IbmTranslator",
    "LpTranslator",
    "NumpyTranslator",
    "QctrlTranslator",
    "Qubo",
    "QuboTranslator",
    "ZibTranslator",
]
