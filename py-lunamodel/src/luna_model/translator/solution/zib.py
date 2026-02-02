"""SCIP/ZIB solution translator for LunaModel.

This module provides translation from SCIP (Solving Constraint Integer Programs)
solver results to LunaModel's Solution format. SCIP is a powerful open-source
solver for mixed-integer programming and constraint programming developed by ZIB.
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

    ZibTranslator converts SCIP solver's Model objects to LunaModel's Solution
    format. SCIP (Solving Constraint Integer Programs) is a state-of-the-art
    solver for mixed-integer programming (MIP), mixed-integer nonlinear programming
    (MINLP), and constraint programming developed by Zuse Institute Berlin (ZIB).

    The translator extracts solution values from a solved SCIP model and converts
    them to LunaModel's Solution format, preserving variable values and the
    optimization sense.

    Requires the ``pyscipopt`` package (Python interface to SCIP).

    Examples
    --------
    Convert SCIP solution to LunaModel solution:

    >>> from pyscipopt import Model as ScipModel
    >>> from luna_model.translator import ZibTranslator
    >>> from luna_model import Model, Timing
    >>> # Create and solve with SCIP
    >>> scip = ScipModel("example")
    >>> x = scip.addVar("x", vtype="B")
    >>> y = scip.addVar("y", vtype="B")
    >>> scip.setObjective(x + 2 * y, "maximize")
    >>> scip.addCons(x + y <= 1)
    >>> scip.optimize()
    >>> # Convert to LunaModel solution
    >>> solution = ZibTranslator.to_lm(scip)
    >>> print(solution.best())

    With LunaModel environment:

    >>> from luna_model import Model, Variable, Vtype
    >>> # Original LunaModel
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x + 2 * y
    >>> model.constraints += x + y <= 1
    >>> # After converting to SCIP and solving...
    >>> timing = Timing(solver=0.05, total=0.12)
    >>> solution = ZibTranslator.to_lm(scip, timing=timing, env=model.env)

    Notes
    -----
    SCIP is one of the fastest non-commercial solvers for mixed-integer programming
    and constraint programming. It can handle:
    - Linear and nonlinear objectives
    - Integer, binary, and continuous variables
    - Complex constraint types
    - Large-scale optimization problems

    The translator only extracts variables that exist in the provided environment.
    Variables in the SCIP model but not in the environment are ignored.

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

        Extracts the solution from a solved SCIP model and converts it to
        a LunaModel Solution object.

        Parameters
        ----------
        model : ScipModel
            A solved SCIP model (from pyscipopt) containing the solution.
            Should have been optimized before calling this method.
        timing : Timing | None, optional
            Timing information for the solution process.
        env : Environment | None, optional
            Environment containing variable information. If provided, only
            variables present in the environment are included in the solution.
            If None, uses the current context environment.

        Returns
        -------
        Solution
            LunaModel Solution object with variable values from SCIP.

        Raises
        ------
        RuntimeError
            If ``pyscipopt`` package is not installed.

        Examples
        --------
        Basic usage:

        >>> from pyscipopt import Model as ScipModel
        >>> scip = ScipModel()
        >>> x = scip.addVar("x", lb=0, ub=10, vtype="I")
        >>> y = scip.addVar("y", lb=0, ub=10, vtype="I")
        >>> scip.setObjective(3 * x + 2 * y, "maximize")
        >>> scip.addCons(x + y <= 5)
        >>> scip.optimize()
        >>> solution = ZibTranslator.to_lm(scip)
        >>> print(solution.best())

        With environment filtering:

        >>> from luna_model import Model
        >>> lm_model = Model()
        >>> x = lm_model.add_variable("x")
        >>> y = lm_model.add_variable("y")
        >>> # After converting to SCIP and solving...
        >>> solution = ZibTranslator.to_lm(scip, env=lm_model.env)
        >>> # Only variables x and y from lm_model.env are included

        Extracting SCIP timing:

        >>> scip.optimize()
        >>> scip_time = scip.getSolvingTime()
        >>> timing = Timing(solver=scip_time, total=scip_time)
        >>> solution = ZibTranslator.to_lm(scip, timing=timing)

        Notes
        -----
        The SCIP model must have been successfully optimized before calling
        this translator. The method extracts values using ``model.getVal(var)``
        for each variable.

        The optimization sense (minimize/maximize) is automatically detected
        from the SCIP model and preserved in the solution.
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
