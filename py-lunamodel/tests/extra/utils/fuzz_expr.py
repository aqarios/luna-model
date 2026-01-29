import itertools
from collections.abc import Generator
from dataclasses import dataclass, field

from luna_model import Environment, Expression, Variable
from tests.utils.fuzz_variable import fuzz_variables


@dataclass
class ExprInfo:
    nvars: int = 0
    offset: int = 0
    deg: int = 0
    vars: list[Variable] = field(default_factory=list)
    lins: list[tuple[Variable, float]] = field(default_factory=list)
    quads: list[tuple[Variable, Variable, float]] = field(default_factory=list)
    hiods: list[tuple[list[Variable], float]] = field(default_factory=list)
    isconst: bool = True


def fuzz_expr(env: Environment) -> Generator[tuple[Expression, ExprInfo]]:
    # it can be empty,
    yield Expression(env), ExprInfo()
    # linear, quadratic, higher-order, or any combination of this
    # it can consist of all combinaations of variables.
    # certain variables have spcific rules.
    # LINEAR
    vars = fuzz_variables(env)
    for cmb in itertools.permutations(vars):
        cmb_list = list(cmb)
        yield (
            _lin_expr(env, cmb_list),
            ExprInfo(
                nvars=len(cmb),
                deg=1,
                vars=cmb_list,
                lins=[(v, 1.0) for v in cmb],
                isconst=False,
            ),
        )


def _lin_expr(
    env: Environment,
    vars: list[Variable],
) -> Expression:
    e = Expression(env)
    for v in vars:
        e += v
    return e
