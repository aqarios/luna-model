import itertools
from typing import Generator
from luna_model import Variable, Vtype, Unbounded, Bounds, Environment
from tests.utils.general import vtypes

_bound_value = [-1.0, 0.0, 1.0, Unbounded, None]


def fuzz_variables(env: Environment) -> Generator[Variable]:
    # binary and spin don't have bounds
    # integer, and real can have bounds
    cnt: int = 0
    for vtype in vtypes():
        match vtype:
            case Vtype.BINARY | Vtype.Binary | Vtype.SPIN | Vtype.Spin:
                yield Variable(f"v{cnt}", vtype, env=env)
                cnt += 1
            case Vtype.INTEGER | Vtype.Integer | Vtype.REAL | Vtype.Real:
                # no bounds set
                yield Variable(f"v{cnt}", vtype, bounds=None, env=env)
                cnt += 1
                for lo, up in itertools.product(_bound_value, _bound_value):
                    yield Variable(f"v{cnt}", vtype, bounds=Bounds(lo, up), env=env)
                    cnt += 1
