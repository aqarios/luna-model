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
from luna_model.solution.sol import Solution

from .analysis import AnalysisPass
from .base import BasePass
from .ir import IR
from .transform import TransformationPass


class PassManager:
    """Manage and execute a sequence of passes on a model.

    The PassManager implements a compiler-style pass pattern, enabling both
    general-purpose and algorithm-specific manipulations of optimization
    models. Each pass is an atomic operation (for example, ChangeSensePass)
    that transforms the model or its intermediate representation (IR). The
    PassManager runs each pass in order and produces a rich IR that records
    the transformations applied and supports back-transformations.
    """

    _pm: PyPassManager

    def __init__(self, passes: Sequence[BasePass | TransformationPass | AnalysisPass] | None = None) -> None:
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
        self._pm = PyPassManager(passes)

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
        return IR._from_pyir(self._pm.run(model._m))

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
        return Solution._from_pys(self._pm.backwards(solution._s, ir._ir))

    def __str__(self) -> str:
        """Human readable string."""
        return self._pm.__str__()

    def __repr__(self) -> str:
        """Debug string."""
        return self._pm.__repr__()
