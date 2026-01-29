from luna_model.environment.env import Environment
from luna_model.model.sense import Sense
from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing

_SCIP_AVAILABLE: bool = False
try:
    from pyscipopt import Model as ScipModel  # type: ignore[reportMissingImports]

    _SCIP_AVAILABLE = True
except ImportError:
    _SCIP_AVAILABLE = False


class ZibTranslator:
    @staticmethod
    def to_lm(
        model: ScipModel,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        if not _SCIP_AVAILABLE:
            raise RuntimeError(
                "scip is required for the ZibTranslator. You can install it using the 'scip' extra.",
            )
        env = env if env is not None else Environment._from_ctx()
        sample = {x.name: model.getVal(x) for x in model.getVars() if x.name in env}
        sense = Sense.Max if model.getObjectiveSense() == "maximize" else Sense.Min
        return Solution.from_dict(
            sample,
            timing=timing,
            env=env,
            sense=sense,
        )
