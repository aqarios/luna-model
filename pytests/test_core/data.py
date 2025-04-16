from __future__ import annotations

from itertools import combinations, product
from math import prod
from typing import Callable, Protocol, Sequence

from aqmodels import (
    Bounds,
    Comparator,
    Constraint,
    Constraints,
    Environment,
    Expression,
    Model,
    Variable,
    Vtype,
)

from .utils import make_seed, random


class T(Protocol):
    def encode(self, *args, **kwargs) -> bytes: ...
    def serialize(self, *args, **kwargs) -> bytes: ...
    @staticmethod
    def decode(*args, **kwargs) -> T: ...
    @staticmethod
    def deserialize(*args, **kwargs) -> T: ...


def serializable_objects(items: list[type[T]]) -> list[T]:
    data: list[T] = list()
    for item in items:
        data.extend(TO_SER_OBJECTS[item]())
    return data


def serialized_objects(items: list[type[T]]) -> list[tuple[T, bytes, type[T]]]:
    data: list[tuple[T, bytes, type[T]]] = list()
    for item in items:
        data.extend(SER_OBJECTS[item]())
    return data


def serialized_objects_with_env(
    items: list[type[T]],
) -> list[tuple[T, bytes, type[T], Environment]]:
    data: list[tuple[T, bytes, type[T], Environment]] = list()
    for item in items:
        data.extend(SER_WITH_ENV_OBJECTS[item]())
    return data


def make_env_with_vars() -> tuple[Environment, list[Variable]]:
    env = Environment()
    with env:
        variables: list[Variable] = [
            Variable("x", vtype=Vtype.Binary),
            Variable("s", vtype=Vtype.Spin),
            Variable("i", vtype=Vtype.Integer),
            Variable("r", vtype=Vtype.Real),
            Variable("ibl", vtype=Vtype.Integer, bounds=Bounds(lower=0.0)),
            Variable("ibu", vtype=Vtype.Integer, bounds=Bounds(upper=1.0)),
            Variable("ib", vtype=Vtype.Integer, bounds=Bounds(lower=1.0, upper=2.0)),
            Variable("rbl", vtype=Vtype.Real, bounds=Bounds(lower=0.0)),
            Variable("rbu", vtype=Vtype.Real, bounds=Bounds(upper=1.0)),
            Variable("rb", vtype=Vtype.Real, bounds=Bounds(lower=1.0, upper=2.0)),
        ]

    return env, variables


def environments() -> list[Environment]:
    variables_gen: list[Callable[[Environment], Variable]] = [
        lambda env: Variable("x", env=env, vtype=Vtype.Binary),
        lambda env: Variable("s", env=env, vtype=Vtype.Spin),
        lambda env: Variable("i", env=env, vtype=Vtype.Integer),
        lambda env: Variable("r", env=env, vtype=Vtype.Real),
        lambda env: Variable(
            "ibl", env=env, vtype=Vtype.Integer, bounds=Bounds(lower=0.0)
        ),
        lambda env: Variable(
            "ibu", env=env, vtype=Vtype.Integer, bounds=Bounds(upper=1.0)
        ),
        lambda env: Variable(
            "ib", env=env, vtype=Vtype.Integer, bounds=Bounds(lower=1.0, upper=2.0)
        ),
        lambda env: Variable(
            "rbl", env=env, vtype=Vtype.Real, bounds=Bounds(lower=0.0)
        ),
        lambda env: Variable(
            "rbu", env=env, vtype=Vtype.Real, bounds=Bounds(upper=1.0)
        ),
        lambda env: Variable(
            "rb", env=env, vtype=Vtype.Real, bounds=Bounds(lower=1.0, upper=2.0)
        ),
    ]

    envs: list[Environment] = list()
    for r in range(1, len(variables_gen) + 1):
        var_gen_combs = combinations(variables_gen, r)
        for var_gens in var_gen_combs:
            env = Environment()
            for var_gen in var_gens:
                var_gen(env)
            envs.append(env)

    return envs


def expressions(
    params: tuple[Environment, list[Variable]] | None = None, seed: int | None = None
) -> list[Expression]:
    if not seed:
        seed = make_seed()

    if not params:
        params = make_env_with_vars()

    const = constant_expression(*params, seed)
    linear = linear_expression(*params)
    quadratic = quadratic_expression(*params)
    higher_order = higher_order_expression(*params)

    items = [const, linear, quadratic, higher_order]
    item_cominations: list[Expression] = list()
    for r in range(2, len(items) + 1):
        combs = combinations(items, r)
        item_cominations.extend(
            [sum([random(seed) * v for v in comb]) for comb in combs]
        )  # type: ignore

    return [*items, *item_cominations]


def expressions_with_env(
    params: tuple[Environment, list[Variable]] | None = None, seed: int | None = None
) -> tuple[list[Expression], Environment]:
    if not seed:
        seed = make_seed()
    if not params:
        params = make_env_with_vars()
    return expressions(params, seed), params[0]


def constant_expression(env: Environment, _: list[Variable], seed: int) -> Expression:
    """ """
    return Expression(env) + random(seed)


def linear_expression(_: Environment, variables: list[Variable]) -> Expression:
    """ """
    return sum(variables, 0)  # type: ignore


def quadratic_expression(env: Environment, variables: list[Variable]) -> Expression:
    """ """
    expr = Expression(env)

    quadratic_combinations = combinations(variables, 2)
    for comb in quadratic_combinations:
        expr += comb[0] * comb[1]

    return expr


def higher_order_expression(env: Environment, variables: list[Variable]) -> Expression:
    """ """
    expr = Expression(env)

    higher_order_combinations: list[tuple[Variable, ...]] = []
    for r in range(3, len(variables) + 1):
        higher_order_combinations.extend(combinations(variables, r))
    for comb in higher_order_combinations:
        expr += prod(comb)  # type: ignore

    return expr


def constraints(
    params: tuple[Environment, list[Variable]] | None = None,
    seed: int | None = None,
) -> list[Constraints]:
    if not seed:
        seed = make_seed()
    if not params:
        params = make_env_with_vars()

    env, variables = params

    def lin_expr():
        return linear_expression(env, variables)

    def quad_expr():
        return quadratic_expression(env, variables)

    def ho_expr():
        return higher_order_expression(env, variables)

    linears = [
        Constraint(lin_expr(), random(seed), Comparator.Le),
        Constraint(lin_expr(), random(seed), Comparator.Eq),
        Constraint(lin_expr(), random(seed), Comparator.Ge),
    ]

    quadratics = [
        Constraint(quad_expr(), random(seed), Comparator.Le),
        Constraint(quad_expr(), random(seed), Comparator.Eq),
        Constraint(quad_expr(), random(seed), Comparator.Ge),
    ]

    higher_orders = [
        Constraint(ho_expr(), random(seed), Comparator.Le),
        Constraint(ho_expr(), random(seed), Comparator.Eq),
        Constraint(ho_expr(), random(seed), Comparator.Ge),
    ]

    items: list[list[Constraint]] = [linears, quadratics, higher_orders]

    constraints_collection: list[Constraints] = list()
    for r in range(1, len(items) + 1):
        combs: combinations[tuple[list[Constraint], ...]] = combinations(items, r)

        for comb in combs:
            constraints = Constraints()
            for constr_col in comb:
                for constr in constr_col:
                    constraints.add_constraint(constr)
            constraints_collection.append(constraints)

    return constraints_collection


def constraints_with_env(
    params: tuple[Environment, list[Variable]] | None = None,
    seed: int | None = None,
) -> tuple[list[Constraints], Environment]:
    if not seed:
        seed = make_seed()
    if not params:
        params = make_env_with_vars()
    return constraints(params, seed), params[0]


def models() -> list[Model]:
    """ """
    seed = make_seed()
    params = make_env_with_vars()
    expression_collection = expressions(params, seed)
    constraints_collection = constraints(params, seed)

    model_collection: list[Model] = list()

    # Model, no constraints, just objective.
    for expr in expression_collection:
        model = Model(env=params[0])
        model.objective += expr
        model_collection.append(model)
    # Model, no objective, just constraints
    for constr in constraints_collection:
        model = Model(env=params[0])
        model.constraints = constr
        model_collection.append(model)
    # Model, mixed objective and constraints
    combs = product(expression_collection, constraints_collection)
    for expr, constr in combs:
        model = Model(env=params[0])
        model.objective += expr
        model.constraints = constr
        model_collection.append(model)

    return model_collection


def to_serialized(
    f: Callable[[], Sequence[T]], t: type[T]
) -> list[tuple[T, bytes, type[T]]]:
    return [(e, e.encode(), t) for e in f()]


def to_serialized_with_env(
    f: Callable[[], tuple[Sequence[T], Environment]], t: type[T]
) -> list[tuple[T, bytes, type[T], Environment]]:
    items, env = f()
    return [(e, e.encode(), t, env) for e in items]


TO_SER_OBJECTS: dict[type[T], Callable[[], Sequence[T]]] = {
    Expression: expressions,
    Constraints: constraints,
    Model: models,
    Environment: environments,
}

SER_OBJECTS: dict[type[T], Callable[[], Sequence[tuple[T, bytes, type[T]]]]] = {
    Model: lambda: to_serialized(models, Model),
    Environment: lambda: to_serialized(environments, Environment),
}

SER_WITH_ENV_OBJECTS: dict[
    type[T], Callable[[], Sequence[tuple[T, bytes, type[T], Environment]]]
] = {
    Expression: lambda: to_serialized_with_env(expressions_with_env, Expression),
    Constraints: lambda: to_serialized_with_env(constraints_with_env, Constraints),
}
