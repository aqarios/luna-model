from luna_model.model.sense import Sense
from luna_model.solution.sol import Solution
from luna_model.solution.timer import Timing
from luna_model.environment.env import Environment
from luna_model.errors import NoActiveEnvironmentFoundError

# TODO: try, else default and error...
from pyscipopt import Model as ScipModel


class ZibTranslator:
    @staticmethod
    def to_lm(
        model: ScipModel,
        timing: Timing | None = None,
        *,
        env: Environment | None = None,
    ) -> Solution:
        env = env if env is not None else Environment._from_ctx()
        sample = {x.name: model.getVal(x) for x in model.getVars() if x.name in env}
        sense = Sense.Max if model.getObjectiveSense() == "maximize" else Sense.Min
        return Solution.from_dict(
            sample,
            timing=timing,
            env=env,
            sense=sense,
        )
