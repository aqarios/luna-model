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

# type: ignore[reportPossiblyUnboundVariable]
from typing import TYPE_CHECKING

from luna_model.environment.env import Environment
from luna_model.model.sense import Sense
from luna_model.solution.sol import Solution
from luna_model.timer import Timing

_SCIP_AVAILABLE: bool = False
try:
    from pyscipopt import Model as ScipModel

    _SCIP_AVAILABLE = True
except ImportError:
    _SCIP_AVAILABLE = False

if TYPE_CHECKING:
    from pyscipopt import Model as ScipModel


class ZibTranslator:
    """Translator for SCIP/ZIB solution format.

    Converts SCIP Model objects to LunaModel Solutions.

    Requires the ``scip`` extra.
    """

    @staticmethod
    def to_lm(
        model: "ScipModel",
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert SCIP solution to LunaModel solution.

        Parameters
        ----------
        model : ScipModel
            A solved SCIP model (from pyscipopt).
        timing : Timing, optional
            Timing information for the solution process.
        env : Environment, optional
            Environment for variable filtering. Required either as parameter or active context.
            Only variables in the environment are included in the solution.

        Returns
        -------
        Solution
            LunaModel Solution with variable values from SCIP.

        Raises
        ------
        RuntimeError
            If ``pyscipopt`` package is not installed.
        """
        if not _SCIP_AVAILABLE:
            msg = "scip is required for the ZibTranslator. You can install it using the 'scip' extra."
            raise RuntimeError(msg)
        env = env if env is not None else Environment._from_ctx()
        sample = {x.name: model.getVal(x) for x in model.getVars() if x.name in env}
        sense = Sense.MAX if model.getObjectiveSense() == "maximize" else Sense.MIN
        return Solution.from_dict(
            sample,
            timing=timing,
            env=env,
            sense=sense,
        )
