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

from collections.abc import Sequence

from luna_model._lm import PyPassManager
from luna_model.model.model import Model
from luna_model.transformation.output import TransformationOutput
from luna_model.transformation.pipeline import Pipeline
from luna_model.transformation.typing import Pass


class PassManager:
    """Manage and execute a sequence of passes on a model.

    The PassManager implements a compiler-style pass pattern, enabling both
    general-purpose and algorithm-specific manipulations of optimization
    models. Each pass is an atomic operation that transforms the model or
    extracts information from the model. The PassManager runs each pass in
    order and produces an IR that records the transformations applied
    and supports back-transformations.

    Parameters
    ----------
    passes : Sequence[BasePass], optional
        An ordered sequence of Pass instances to apply, default None.
    """

    _pm: PyPassManager

    def __init__(self, passes: Sequence[Pass] | Pipeline | None = None) -> None:
        self._pm = PyPassManager(passes)

    def add(self, pass_: Pass) -> None:
        """Append a pass to the configured passes.

        Parameters
        ----------
        pass_ : TransformationPass | AnalysisPass | ControlFlowPass
            The pass to add to this PassManager's configured passes.
        """
        self._pm.add(pass_)

    def run(self, model: Model) -> TransformationOutput:
        """Apply all configured passes.

        Apply all configured passes to the given model and return the
        resulting intermediate representation.

        Parameters
        ----------
        model : Model
            The model to be transformed.

        Returns
        -------
        TransformationOutput
            The transformation ouput after transformation.
        """
        return TransformationOutput._from_pyto(self._pm.run(model._m))

    def __str__(self) -> str:
        """Human readable string."""
        return self._pm.__str__()

    def __repr__(self) -> str:
        """Debug string."""
        return self._pm.__repr__()
