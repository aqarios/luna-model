from __future__ import annotations

from collections.abc import Callable
from typing import overload

from luna_model._lm import PyIfElsePass

from .base import BasePass
from .cache import AnalysisCache
from .pipeline import Pipeline


class IfElsePass(PyIfElsePass, BasePass):
    _ifelse: PyIfElsePass

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
        name: str | None = None,
    ) -> None:
        self._ifelse = PyIfElsePass(
            requires=requires,
            condition=lambda cache: condition(AnalysisCache._from_pyac(cache)),
            then=then._pipeline,
            otherwise=otherwise._pipeline,
            name=name,
        )

    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._ifelse.name

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._ifelse.requires
