from collections.abc import Sequence
from typing import Self, TypeAlias

from luna_model._lm import PyPipeline
from luna_model.transformation.analysis import AnalysisPass
from luna_model.transformation.composite import CompositePass
from luna_model.transformation.control_flow import ControlFlowPass
from luna_model.transformation.meta_analysis import MetaAnalysisPass
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis
from luna_model.transformation.passes.composite.builtin import BuiltinComposite
from luna_model.transformation.passes.control_flow.builtin import BuiltinControlFlow
from luna_model.transformation.passes.meta_analysis.builtin import BuiltinMetaAnalysis
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation
from luna_model.transformation.transformation import TransformationPass
from luna_model.wrapper import wraps

Pass: TypeAlias = (
    AnalysisPass
    | CompositePass
    | ControlFlowPass
    | MetaAnalysisPass
    | TransformationPass
    | BuiltinAnalysis
    | BuiltinComposite
    | BuiltinControlFlow
    | BuiltinMetaAnalysis
    | BuiltinTransformation
)


class _PipelineMeta(type(PyPipeline)):
    def __instancecheck__(self, instance: object, /) -> bool:
        return isinstance(instance, PyPipeline) or super().__instancecheck__(instance)


class Pipeline(PyPipeline, metaclass=_PipelineMeta):
    """
    A pipeline for executing multiple transformation passes in sequence.

    Pipelines organize and execute multiple passes, managing dependencies
    and ensuring they run in the correct order.
    """

    def __new__(cls, steps: Sequence[Pass | Self], name: str) -> Self:
        """Create a new pipeline from a sequence of passes.

        Parameters
        ----------
        steps : Sequence[Pass | Self]
            Ordered passes/pipelines to execute.
        name : str
            Human-readable pipeline name.

        Returns
        -------
        Self
            New pipeline instance.
        """
        return super().__new__(cls, name=name, steps=steps)

    @wraps()
    def name(self) -> str:
        """Get the name of this pipeline."""
        raise NotImplementedError

    @wraps()
    def add(self, new_pass: Pass | Self) -> None:
        """
        Add a new pass to the pipeline.

        Parameters
        ----------
        new_pass : Pass | Self
            The pass to add to the pipeline.
        """
        raise NotImplementedError(f"add({new_pass})")

    @wraps()
    def requires(self) -> list[str]:
        """
        List of passes that must run before this pipeline.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        raise NotImplementedError

    @wraps()
    def invalidates(self) -> list[str]:
        """Get analysis keys invalidated by this pipeline.

        Returns
        -------
        list[str]
            Analysis/pass keys invalidated by at least one step.
        """
        raise NotImplementedError

    @wraps()
    def provides(self) -> list[str]:
        """Get analysis keys provided by this pipeline.

        Returns
        -------
        list[str]
            Analysis/pass keys produced by at least one step.
        """
        raise NotImplementedError

    @wraps()
    def clear(self) -> None:
        """
        Clear all passes from the pipeline.

        Removes all transformation passes, leaving an empty pipeline.
        """
        raise NotImplementedError

    @wraps()
    def passes(self) -> list[Pass | Self]:
        """
        Get all passes that are part of the pipeline.

        Returns
        -------
        list[Pass | Self]
            The transformation passes in this pipeline.
        """
        raise NotImplementedError

    @wraps()
    def __str__(self) -> str:
        """Human readable string."""
        raise NotImplementedError

    @wraps()
    def __repr__(self) -> str:
        """Debug representation string."""
        raise NotImplementedError
