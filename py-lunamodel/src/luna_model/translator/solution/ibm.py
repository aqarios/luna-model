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
from luna_model.solution.sol import Solution
from luna_model.timer import Timing

_QISKIT_AVAILABLE: bool = False
try:
    from qiskit.primitives import PrimitiveResult, PubResult
    from qiskit_optimization import QuadraticProgram

    _QISKIT_AVAILABLE = True
except ImportError:
    _QISKIT_AVAILABLE = False

if TYPE_CHECKING:
    from qiskit.primitives import PrimitiveResult, PubResult
    from qiskit_optimization import QuadraticProgram


class IbmTranslator:
    """Translator for IBM Qiskit solution format.

    Converts IBM Qiskit PrimitiveResult objects to LunaModel Solutions.

    Requires the ``qiskit`` extra.
    """

    @staticmethod
    def to_lm(
        result: "PrimitiveResult[PubResult]",
        quadratic_program: "QuadraticProgram",
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        """Convert IBM Qiskit result to LunaModel solution.

        Parameters
        ----------
        result : PrimitiveResult[PubResult]
            Qiskit primitive result containing measurement outcomes.
        quadratic_program : QuadraticProgram
            Original QuadraticProgram used to evaluate objective values.
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for variable mapping. Required either as parameter or active context.

        Returns
        -------
        Solution
            LunaModel Solution with bitstring counts and energies.

        Raises
        ------
        RuntimeError
            If ``qiskit`` or ``qiskit-optimization`` packages are not installed.
        """
        if not _QISKIT_AVAILABLE:
            msg = (
                "qiskit and qiskit_optimization are required for the IbmTranslator. "
                "You can install it using the 'qiskit' extra."
            )
            raise RuntimeError(msg)
        meas = result[0].data.meas
        counts: dict[str, int] = meas.get_counts()

        energies = []

        for bitstring in counts:
            sample = [int(b) for b in bitstring]
            sample = sample[::-1]  # reverse ordering for correct bitstrings.
            energies.append(float(quadratic_program.objective.evaluate(sample)))

        return Solution.from_counts(
            counts,
            energies=energies,
            timing=timing,
            env=env,
        )
