# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
from __future__ import annotations

from typing import TYPE_CHECKING, overload, override

from luna_model._lm import PyIfElsePass

from .base import BasePass
from .cache import AnalysisCache

if TYPE_CHECKING:
    from collections.abc import Callable

    from .pipeline import Pipeline


class IfElsePass(PyIfElsePass, BasePass):
    """Conditional pass that executes different pipelines based on a runtime condition.

    The ``IfElsePass`` evaluates a condition function against the analysis cache at runtime
    and executes either the ``then`` pipeline or the ``otherwise`` pipeline based on the result.
    This enables branching logic in transformation workflows, allowing different transformation
    strategies based on model properties discovered during analysis.

    Parameters
    ----------
    requires : list[str]
        List of pass names that must be run before this pass. These passes provide the data used
        by the condition function.
    condition : Callable[[AnalysisCache], bool]
        A function that takes an ``AnalysisCache`` and returns a boolean. If ``True``,
        the ``then`` pipeline is executed; otherwise, the ``otherwise`` pipeline runs.
    then : Pipeline
        The pipeline to execute when the condition evaluates to ``True``.
    otherwise : Pipeline
        The pipeline to execute when the condition evaluates to ``False``.
    name : str, optional
        Optional name for this pass. If not provided, a default name is generated.

    Examples
    --------
    Execute different transformations based on the maximum bias:

    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager, Pipeline, IfElsePass
    >>> from luna_model.transformation.passes import MaxBiasAnalysis, BinarySpinPass
    >>> # Create conditional pass
    >>> conditional = IfElsePass(
    ...     requires=["max-bias"],
    ...     condition=lambda cache: cache["max-bias"].val > 10.0,
    ...     then=Pipeline([BinarySpinPass(vtype=Vtype.BINARY)]),
    ...     otherwise=Pipeline([]),
    ...     name="conditional-spin-conversion",
    ... )
    >>> # Use in PassManager
    >>> pm = PassManager([MaxBiasAnalysis(), conditional])
    >>> result = pm.run(model)

    Notes
    -----
    The condition function has access to all analysis results computed by the passes
    listed in ``requires``. Both pipelines can contain arbitrarily complex sequences
    of transformations and analyses.

    The condition is evaluated once per ``IfElsePass`` execution. If you need to
    re-evaluate during pipeline execution, nest multiple ``IfElsePass`` instances.
    """

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
            then=then,
            otherwise=otherwise,
            name=name,
        )

    @property
    @override
    def name(self) -> str:
        return self._ifelse.name

    @property
    @override
    def requires(self) -> list[str]:
        return self._ifelse.requires
