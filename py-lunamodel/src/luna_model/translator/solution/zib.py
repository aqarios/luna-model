"""SCIP/ZIB solution translator for LunaModel.

This module provides translation from SCIP solver results to
LunaModel's Solution format.
"""

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

    Requires the ``pyscipopt`` package.

    Examples
    --------
    >>> from pyscipopt import Model as ScipModel
    >>> from luna_model.translator import ZibTranslator
    >>> scip = ScipModel("example")
    >>> # ... build and solve model ...
    >>> scip.optimize()
    >>> solution = ZibTranslator.to_lm(scip)

    See Also
    --------
    LpTranslator : LP format translator (compatible with SCIP)
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
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment for variable filtering. If provided, only variables
            in the environment are included.

        Returns
        -------
        Solution
            LunaModel Solution with variable values from SCIP.

        Raises
        ------
        RuntimeError
            If ``pyscipopt`` package is not installed.

        Examples
        --------
        >>> from pyscipopt import Model as ScipModel
        >>> scip = ScipModel()
        >>> x = scip.addVar("x", lb=0, ub=10, vtype="I")
        >>> scip.setObjective(3 * x, "maximize")
        >>> scip.optimize()
        >>> solution = ZibTranslator.to_lm(scip)
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
