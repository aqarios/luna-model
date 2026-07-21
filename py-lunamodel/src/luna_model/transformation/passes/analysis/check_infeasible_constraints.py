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

from typing import Self

from luna_model._lm import PyCheckInfeasibleConstraintsAnalysis
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis


class CheckInfeasibleConstraintsAnalysis(PyCheckInfeasibleConstraintsAnalysis, BuiltinAnalysis[None]):
    """Analysis pass that checks the model's linear constraints for infeasibility.

    For each constraint, the minimum and maximum value that the left-hand side (LHS) expression can take is computed
    from the bounds of the variables involved. This achievable range is then compared against the right-hand side (RHS)
    to decide whether the constraint can be satisfied:

    - ``lhs <= rhs`` is infeasible if ``min(lhs) > rhs``
    - ``lhs >= rhs`` is infeasible if ``max(lhs) < rhs``
    - ``lhs == rhs`` is infeasible if ``rhs < min(lhs)`` or ``rhs > max(lhs)``

    Raises
    ------
    InfeasibleError
        If any constraint is provably infeasible (for example ``x1 + x2 <= -1`` or ``x1 + x2 >= 3``
        for binary ``x1``, ``x2``).

    Notes
    -----
    This is a conservative bounds-based check: it is *sound* (every constraint it flags is genuinely infeasible)
    but *incomplete* (it does not detect every infeasibility, e.g. integrality gaps such as
    ``2 * x == 1``). Passing this pass therefore does not prove that the whole model is feasible.
    """

    def __new__(cls) -> Self:
        """Create a new infeasible constraints analysis pass."""
        return super().__new__(cls)

    def __init__(self) -> None: ...
