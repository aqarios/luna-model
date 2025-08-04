from abc import abstractmethod
from collections.abc import Callable
from enum import Enum
from typing import Any, Literal, overload

from aqmodels import Model, Sense, Solution, Timing, Vtype

class BasePass:
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        ...
    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...

class Pipeline(BasePass):
    @overload
    def __init__(self, passes: list[BasePass]) -> None: ...
    @overload
    def __init__(self, passes: list[BasePass], name: str) -> None: ...
    def __init__(self, passes: list[BasePass], name: str | None = ...) -> None: ...
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        ...
    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...

    @property
    def satisfies(self) -> set[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...

    def add(self, new_pass: BasePass) -> None:
        """Add new pass to pipeline."""
        ...

    def clear(self) -> None:
        """Clear pipeline."""
        ...

    @property
    def passes(self) -> list[BasePass]:
        """Get all passes that are part of the pipeline."""
        ...

    def __len__(self) -> int: ...

class IfElsePass(BasePass):
    @overload
    def __init__(
        self,
        requires: list[str],
        condition: Callable[[AnalysisCache], bool],
        then: Pipeline,
        otherwise: Pipeline,
    ) -> None: ...
    @overload
    def __init__(
        self,
        requires: list[str],
        condition: Callable[[AnalysisCache], bool],
        then: Pipeline,
        otherwise: Pipeline,
        name: str,
    ) -> None: ...
    def __init__(
        self,
        requires: list[str],
        condition: Callable[[AnalysisCache], bool],
        then: Pipeline,
        otherwise: Pipeline,
        name: str | None = ...,
    ) -> None: ...
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        ...
    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...

class TransformationPass(BasePass):
    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        ...
    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...
    @property
    def invalidates(self) -> list[str]:
        """Get a list of passes that are invalidated by this pass."""
        ...
    @abstractmethod
    def run(
        self, model: Model, cache: AnalysisCache
    ) -> (
        TransformationOutcome | tuple[Model, ActionType] | tuple[Model, ActionType, Any]
    ):
        """Run/Execute this transformation pass."""
        ...
    @abstractmethod
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        """Convert a solution back to fit this pass' input.

        Convert a solution from a representation fitting this pass' output to
        a solution representation fitting this pass' input.
        """
        ...

class TransformationOutcome:
    """Output object for transformation pass."""

    model: Model
    action: ActionType
    analysis: ...

    def __init__(
        self, model: Model, action: ActionType, analysis: ... = None
    ) -> None: ...
    @staticmethod
    def nothing(model: Model) -> TransformationOutcome:
        """Easy nothing action return."""
        ...

class AnalysisCache:
    @overload
    def __getitem__(  # type: ignore[reportOverlappingOverload]
        self, key: Literal["max-bias"]
    ) -> MaxBias: ...
    @overload
    def __getitem__(self, key: str) -> ...: ...
    def __getitem__(self, key: str) -> Any:
        """Get the analysis result for a specific analysis pass."""
        ...

class AnalysisPass(BasePass):
    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        ...
    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...
    @abstractmethod
    def run(self, model: Model, cache: AnalysisCache) -> ...:
        """Run/Execute this analysis pass."""
        ...

class MetaAnalysisPass(BasePass):
    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        ...
    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        ...
    @abstractmethod
    def run(self, passes: list[BasePass], cache: AnalysisCache) -> ...:
        """Run/Execute this analysis pass."""
        ...

class ActionType(Enum):
    DidTransform = ...
    """Indicate that the pass did transform the model."""
    DidAnalysis = ...
    """Indicate that the pass did analyse the model."""
    DidAnalysisTransform = ...
    """Indicate that the pass did analyse and transfrom the model."""
    DidNothing = ...
    """Indicate that the pass did NOT do anything."""

class ChangeSensePass(BasePass):
    """A transformation pass to change the model's Sense to a target Sense."""

    def __init__(self, sense: Sense) -> None:
        """Transform the model's Sense to a target Sense.

        Parameters
        ----------
        sense : Sense
            The target sense of the model after calling the `run` method on it.
        """
        ...
    @property
    def sense(self) -> Sense:
        """Get the specified target sense of this pass."""
        ...

class MaxBias:
    """An analysis pass result storing the max bias (coefficient) of a model."""

    @property
    def val(self) -> float:
        """Get the value of the maxium bias."""
        ...

class MaxBiasAnalysis(BasePass):
    """An analysis pass computing the maximum bias contained in the model."""

    def __init__(self) -> None: ...

class BinarySpinPass(BasePass):
    """An transformation pass changing the binary/spin variables to spin/binary."""

    def __init__(
        self, vtype: Literal[Vtype.Binary, Vtype.Spin], prefix: str | None
    ) -> None: ...
    @property
    def vtype(self) -> Vtype:
        """Get the target vtype."""
        ...

    @property
    def prefix(self) -> str | None:
        """Get the naming prefix."""
        ...

class BinarySpinInfo:
    @property
    def old_vtype(self) -> Vtype:
        """Get the source vtype."""
        ...

    @property
    def new_vtype(self) -> Vtype:
        """Get the variable name mapping."""
        ...

class LogElement:
    """An element of the execution log of an intermediate representation (IR)."""

    @property
    def pass_name(self) -> str:
        """The name of the pass."""
        ...

    @property
    def timing(self) -> Timing:
        """Timing information for this log element."""
        ...

    @property
    def kind(self) -> ActionType | None:
        """Transformation type information for this log element, if available."""
        ...

    # @property
    # def components(self) -> list[LogElement] | None:
    #     """Components of this log-element."""
    #     ...

class IR:
    """The intermediate representation (IR) of a model after transformation.

    The IR contains the resulting model after transformation (`ir.model`) as well
    as the analysis cache (`ir.cache`) and an execution log (`ir.execution_log`).
    """

    @property
    def model(self) -> Model:
        """Get the model stored in the IR."""
        ...

    @property
    def cache(self) -> AnalysisCache:
        """Get the analysis cache stored the IR."""
        ...

    @property
    def execution_log(self) -> list[LogElement]:
        """Get the analysis cache stored the IR."""
        ...

class PassManager:
    """Manage and execute a sequence of passes on a model.

    The PassManager implements a compiler-style pass pattern, enabling both
    general-purpose and algorithm-specific manipulations of optimization
    models. Each pass is an atomic operation (for example, ChangeSensePass)
    that transforms the model or its intermediate representation (IR). The
    PassManager runs each pass in order and produces a rich IR that records
    the transformations applied and supports back-transformations.
    """

    def __init__(
        self, passes: list[BasePass | TransformationPass | AnalysisPass] | None = ...
    ) -> None:
        """Manage and execute a sequence of passes on a model.

        The PassManager implements a compiler-style pass pattern, enabling both
        general-purpose and algorithm-specific manipulations of optimization
        models. Each pass is an atomic operation (for example, ChangeSensePass)
        that transforms the model or its intermediate representation (IR). The
        PassManager runs each pass in order and produces a rich IR that records
        the transformations applied and supports back-transformations.

        Parameters
        ----------
        passes : list[TransformationPass | AnalysisPass] | None
            An ordered sequence of Pass instances to apply. Each Pass must conform to
            the `TransformationPass` or `AnalysisPass` interface, default None.
        """
        ...

    def run(self, model: Model) -> IR:
        """Apply all configures passes.

        Apply all configured passes to the given model and return the
        resulting intermediate representation.

        Parameters
        ----------
        model : Model
            The model to be transformed.

        Returns
        -------
        IR
            The intermediate representation of the model after transformation.
        """
        ...

    def backwards(self, solution: Solution, ir: IR) -> Solution:
        """Apply the back transformation to the given solution.

        Parameters
        ----------
        solution : Solution
            The solution to transform back to a representation fitting the original
            (input) model of this `PassManager`.
        ir : IR
            The intermediate representation (IR) resulted from the `run` call.

        Returns
        -------
        Solution
            A solution object representing a solution to the original problem passed
            to this `PassManager`'s run method.
        """
        ...

__all__ = [
    "IR",
    "ActionType",
    "AnalysisCache",
    "AnalysisPass",
    "BasePass",
    "ChangeSensePass",
    "LogElement",
    "MaxBias",
    "MaxBiasAnalysis",
    "PassManager",
    "TransformationPass",
]
