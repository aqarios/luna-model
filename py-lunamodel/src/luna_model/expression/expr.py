from __future__ import annotations
from typing import TYPE_CHECKING, Any, Callable, Self

from luna_model._utils import wrap_env, wrap_var, wrap_c
from luna_model._lm import PyExpression
from luna_model.expression.iter import ExprIter

if TYPE_CHECKING:
    from luna_model.constraint import Constraint
    from luna_model.variable.var import Variable
    from luna_model.solution.sol import Solution
    from luna_model.environment.environment import Environment
    from numpy.typing import NDArray


class Expression:
    _expr: PyExpression

    def __init__(self, env: Environment | None = None) -> None:
        if env is None:
            self._expr = PyExpression()
        else:
            self._expr = PyExpression(env._env)

    @classmethod
    def _from_pyexpr(cls, py_expr: PyExpression) -> Expression:
        """Construct LunaModel Expression from FFI PyExpression object."""
        expr = cls.__new__(cls)
        expr._expr = py_expr
        return expr

    @classmethod
    def const(cls, value: float, /, env: Environment | None = None) -> Expression:
        return cls._from_pyexpr(PyExpression.const(value, env._env if env else None))

    @property
    def environment(self) -> Environment:
        return wrap_env(self._expr.environment)

    @property
    def num_variables(self) -> int:
        return self._expr.num_variables

    def get_offset(self) -> float:
        return self._expr.get_offset()

    def get_linear(self, variable: Variable) -> float:
        return self._expr.get_linear(variable._v)

    def get_quadratic(self, u: Variable, v: Variable) -> float:
        return self._expr.get_quadratic(u._v, v._v)

    def get_higher_order(self, *variables: Variable) -> float:
        return self._expr.get_higher_order([v._v for v in variables])

    def items(self) -> ExprIter:
        return ExprIter._from_pyei(self._expr.items())

    def variables(self) -> list[Variable]:
        return [wrap_var(v) for v in self._expr.variables()]

    def degree(self) -> int:
        return self._expr.degree()

    def linear_items(self) -> list[tuple[Variable, float]]:
        return [(wrap_var(v), b) for v, b in self._expr.linear_items()]

    def quadratic_items(self) -> list[tuple[Variable, Variable, float]]:
        return [
            (wrap_var(u), wrap_var(v), b) for u, v, b in self._expr.quadratic_items()
        ]

    def higher_order_items(self) -> list[tuple[list[Variable], float]]:
        return [
            ([wrap_var(v) for v in vars], b)
            for vars, b in self._expr.higher_order_items()
        ]

    def is_constant(self) -> bool:
        return self._expr.is_constant()

    def has_quadratic(self) -> bool:
        return self._expr.has_quadratic()

    def has_higher_order(self) -> bool:
        return self._expr.has_higher_order()

    def is_equal(self, other: Expression) -> bool:
        return self._expr.is_equal(other._expr)

    def is_equal_contents(self, other: Expression) -> bool:
        return self._expr.is_equal_contents(other._expr)

    def separate(self, variables: list[Variable]) -> tuple[Expression, Expression]:
        lhs, rhs = self._expr.separate([v._v for v in variables])
        return (self._from_pyexpr(lhs), self._from_pyexpr(rhs))

    def subsitute(
        self, target: Variable, replacement: Expression | Variable
    ) -> Expression:
        return self._from_pyexpr(self._expr.subsitute(target, replacement))

    def evaluate(self, solution: Solution) -> NDArray:
        return self._expr.evaluate(solution._s)  # type: ignore[attribute]

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        return self._expr.encode(compress, level)

    def serialize(
        self, /, compress: bool | None = True, level: int | None = 3
    ) -> bytes:
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes, env: Environment) -> Expression:
        return cls._from_pyexpr(PyExpression.decode(data, env._env))

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> Expression:
        return cls.decode(data, env)

    @classmethod
    def deep_clone_many(cls, exprs: list[Expression]) -> list[Expression]:
        return [
            cls._from_pyexpr(cloned)
            for cloned in PyExpression.deep_clone_many([e._expr for e in exprs])
        ]

    def __add__(self, other: Expression | Variable | int | float) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__add__))

    def __sub__(self, other: Expression | Variable | int | float) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__sub__))

    def __mul__(self, other: Expression | Variable | int | float) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__mul__))

    def __radd__(self, other: Expression | Variable | int | float) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__radd__))

    def __rsub__(self, other: Expression | Variable | int | float) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__rsub__))

    def __rmul__(self, other: Expression | Variable | int | float) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__rmul__))

    def __iadd__(self, other: Expression | Variable | int | float) -> Self:
        self._op(other, self._expr.__iadd__)
        return self

    def __isub__(self, other: Expression | Variable | int | float) -> Self:
        self._op(other, self._expr.__isub__)
        return self

    def __imul__(self, other: Expression | Variable | int | float) -> Self:
        self._op(other, self._expr.__imul__)
        return self

    def __pow__(self, other: int) -> Expression:
        return self._from_pyexpr(self._op(other, self._expr.__pow__))

    def __ipow__(self, other: int) -> Self:
        self._op(other, self._expr.__ipow__)
        return self

    def __neg__(self) -> Expression:
        return self._from_pyexpr(self._expr.__neg__())

    def __eq__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._expr.__eq__)

    def __le__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._expr.__le__)

    def __ge__(self, other: Expression | Variable | int | float) -> Constraint:  # type: ignore[override]
        return self._cmp(other, self._expr.__ge__)

    def __reduce__(self) -> tuple[Callable, Any]:
        data = self.encode()
        env_data = self.environment.encode()
        return Expression.decode, (data, env_data)

    def __str__(self) -> str:
        return self._expr.__str__()

    def __repr__(self) -> str:
        return self._expr.__repr__()

    def _op(self, other: Expression | Variable | int | float, fn) -> PyExpression:
        from luna_model.expression import Expression
        from luna_model.variable import Variable

        if isinstance(other, Expression):  # type: ignore[attribute]
            res = fn(other._expr)  # type: ignore[attribute]
        elif isinstance(other, Variable):  # type: ignore[attribute]
            res = fn(other._v)  # type: ignore[attribute]
        else:
            res = fn(other)
        return res

    @classmethod
    def _cmp(cls, other: Expression | Variable | int | float, fn) -> Constraint:
        from luna_model.expression import Expression
        from luna_model.variable import Variable

        if isinstance(other, Expression):  # type: ignore[attribute]
            pyc = fn(other._expr)  # type: ignore[attribute]
        elif isinstance(other, Variable):  # type: ignore[attribute]
            pyc = fn(other._v)  # type: ignore[attribute]
        else:
            pyc = fn(other)
        return wrap_c(pyc)
