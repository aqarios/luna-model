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

import numpy as np

from luna_model.environment.env import Environment
from luna_model.solution.sol import Solution
from luna_model.timer import Timing

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import SampleSet

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False

if TYPE_CHECKING:
    from dimod import SampleSet


class DwaveTranslator:
    """Translator for D-Wave solution format.

    Converts D-Wave SampleSet objects to LunaModel Solutions.
    Automatically aggregates duplicate solutions.

    Requires the ``dimod`` extra.
    """

    @staticmethod
    def to_lm(
        sample_set: "SampleSet",
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert D-Wave SampleSet to LunaModel solution.

        Parameters
        ----------
        sample_set : SampleSet
            D-Wave SampleSet returned by a sampler.
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for variable mapping. Required either as parameter or active context.

        Returns
        -------
        Solution
            LunaModel Solution with aggregated samples.

        Raises
        ------
        RuntimeError
            If ``dimod`` package is not installed.
        """
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the DwaveTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        sampleset = sample_set.aggregate()
        variables = sampleset.variables
        record = sampleset.record

        samples = record.sample.astype(np.float64, order="C")
        counts = record.num_occurrences.astype(np.int64, order="C")
        energies = record.energy.astype(np.float64, order="C")

        return Solution.from_arrays(
            data=samples,
            variables=variables,
            counts=counts,
            energies=energies,
            timing=timing,
            env=env,
        )
