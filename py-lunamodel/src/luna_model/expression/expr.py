from __future__ import annotations

from typing import TYPE_CHECKING, Self

from luna_model._lm import PyExpression
from luna_model._utils import wrap_c, wrap_env, wrap_var
from luna_model.environment.env import Environment
from luna_model.expression.iter import ExprIter

if TYPE_CHECKING:
    from collections.abc import Callable

    from numpy.typing import NDArray

    from luna_model._lm import PyConstraint, PyVariable
    from luna_model.constraint import Constraint
    from luna_model.solution.sol import Solution
    from luna_model.variable.var import Variable


class Expression:
    """The expression."""

    _expr: PyExpression

    def __init__(self, env: Environment | None = None) -> None:
        """Init the expression."""
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
        """Create a constant expression."""
        return cls._from_pyexpr(PyExpression.const(value, env._env if env else None))

    @property
    def environment(self) -> Environment:
        """Get the environment of the expression."""
        return wrap_env(self._expr.environment)

    @property
    def num_variables(self) -> int:
        """Get the number of variables in the model.

        Only includes the variables that are contributing to the expression.
        I.e., anything oped that is zero biased or results in zero biased stuff will not
        be respected here.
        """
        return self._expr.num_variables

    def get_offset(self) -> float:
        """Get the offset of the expression."""
        return self._expr.get_offset()

    def get_linear(self, variable: Variable) -> float:
        """Get the linear bias for the variable."""
        return self._expr.get_linear(variable._v)

    def get_quadratic(self, u: Variable, v: Variable) -> float:
        """Get the quadratic bias for the variables."""
        return self._expr.get_quadratic(u._v, v._v)

    def get_higher_order(self, *variables: Variable) -> float:
        """Get the higher order bias for the variables."""
        return self._expr.get_higher_order([v._v for v in variables])

    def items(self) -> ExprIter:
        """Get the expression's terms as an iterator."""
        return ExprIter._from_pyei(self._expr.items())

    def variables(self) -> list[Variable]:
        """Get all the variables in the expression."""
        return [wrap_var(v) for v in self._expr.variables()]

    def degree(self) -> int:
        """Get the degree of the expression."""
        return self._expr.degree()

    def linear_items(self) -> list[tuple[Variable, float]]:
        """Get the linear items of the expression."""
        return [(wrap_var(v), b) for v, b in self._expr.linear_items()]

    def quadratic_items(self) -> list[tuple[Variable, Variable, float]]:
        """Get the quadratic items of the expression."""
        return [(wrap_var(u), wrap_var(v), b) for u, v, b in self._expr.quadratic_items()]

    def higher_order_items(self) -> list[tuple[list[Variable], float]]:
        """Get the higher_order_items items of the expression."""
        return [([wrap_var(v) for v in variables], b) for variables, b in self._expr.higher_order_items()]

    def is_constant(self) -> bool:
        """Check if the expression is constant."""
        return self._expr.is_constant()

    def has_quadratic(self) -> bool:
        """Check if the expression has at least one quadratic term."""
        return self._expr.has_quadratic()

    def has_higher_order(self) -> bool:
        """Check if the expression has at least one higher order term."""
        return self._expr.has_higher_order()

    def is_equal(self, other: Expression) -> bool:
        """Check if two expressions are equal."""
        return self._expr.is_equal(other._expr)

    def equal_contents(self, other: Expression) -> bool:
        """Check if two expressions have equal contents."""
        return self._expr.equal_contents(other._expr)

    def separate(self, variables: list[Variable]) -> tuple[Expression, Expression]:
        """Separate the expression into two expressions such that all the given variables are only in the first."""
        lhs, rhs = self._expr.separate([v._v for v in variables])
        return (self._from_pyexpr(lhs), self._from_pyexpr(rhs))

    def substitute(self, target: Variable, replacement: Expression | Variable) -> Expression:
        """Substitute the target variable by the given replacement expression or variable."""
        from luna_model.variable import Variable  # noqa: PLC0415

        if isinstance(replacement, Variable):
            return self._from_pyexpr(self._expr.substitute(target._v, replacement._v))
        if isinstance(replacement, Expression):
            return self._from_pyexpr(self._expr.substitute(target._v, replacement._expr))
        msg = f"type '{type(replacement)}' not supported in substitution"
        raise TypeError(msg)

    def evaluate(self, solution: Solution) -> NDArray:
        """Evaluate the solution."""
        return self._expr.evaluate(solution._s)

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Encode the expression."""
        return self._expr.encode(compress, level)

    def serialize(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Serialize the expression."""
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes, env: Environment) -> Expression:
        """Decode to the expression."""
        return cls._from_pyexpr(PyExpression.decode(data, env._env))

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> Expression:
        """Deserialize to the expression."""
        return cls.decode(data, env)

    @classmethod
    def deep_clone_many(cls, exprs: list[Expression]) -> list[Expression]:
        """Deep clone many expressions (creating new envs for each expr)."""
        return [cls._from_pyexpr(cloned) for cloned in PyExpression.deep_clone_many([e._expr for e in exprs])]

    def __add__(self, other: Expression | Variable | float) -> Expression:
        """Add other with this expression producing a new one."""
        return self._from_pyexpr(self._op(other, self._expr.__add__))

    def __sub__(self, other: Expression | Variable | float) -> Expression:
        """Sub other from this expression producing a new one."""
        return self._from_pyexpr(self._op(other, self._expr.__sub__))

    def __mul__(self, other: Expression | Variable | float) -> Expression:
        """Mul other with this expression producing a new one."""
        return self._from_pyexpr(self._op(other, self._expr.__mul__))

    def __radd__(self, other: Expression | Variable | float) -> Expression:
        """Right add other with this expression producing a new one."""
        return self._from_pyexpr(self._op(other, self._expr.__radd__))

    def __rsub__(self, other: Expression | Variable | float) -> Expression:
        """Right sub other with this expression producing a new one."""
        return self._from_pyexpr(self._op(other, self._expr.__rsub__))

    def __rmul__(self, other: Expression | Variable | float) -> Expression:
        """Right mul other with this expression producing a new one."""
        return self._from_pyexpr(self._op(other, self._expr.__rmul__))

    def __iadd__(self, other: Expression | Variable | float) -> Self:
        """Add other to this expression."""
        self._op(other, self._expr.__iadd__)
        return self

    def __isub__(self, other: Expression | Variable | float) -> Self:
        """Sub other from this expression."""
        self._op(other, self._expr.__isub__)
        return self

    def __imul__(self, other: Expression | Variable | float) -> Self:
        """Mul other to this expression."""
        self._op(other, self._expr.__imul__)
        return self

    def __pow__(self, value: int) -> Expression:
        """Raise this expression by the value producing a new Expression."""
        return self._from_pyexpr(self._op(value, self._expr.__pow__))

    def __ipow__(self, other: int) -> Self:
        """Raise this expression by the value."""
        self._op(other, self._expr.__ipow__)
        return self

    def __neg__(self) -> Expression:
        """Negate this expression producing a new Expression."""
        return self._from_pyexpr(self._expr.__neg__())

    def __eq__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create an equality constraint."""
        return self._cmp(other, self._expr.__eq__)

    def __le__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create a le constraint."""
        return self._cmp(other, self._expr.__le__)

    def __ge__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create a ge constraint."""
        return self._cmp(other, self._expr.__ge__)

    def __reduce__(self) -> tuple[Callable[[bytes, bytes], Expression], tuple[bytes, ...]]:
        """Reduce this expression. Used by pickle."""
        data = self.encode()
        env_data = self.environment.encode()

        return Expression._unreduce, (data, env_data)

    def __str__(self) -> str:
        """Expression to string."""
        return self._expr.__str__()

    def __repr__(self) -> str:
        """Expression to debug string."""
        return self._expr.__repr__()

    def _op(
        self, other: Expression | Variable | float, fn: Callable[[PyExpression | PyVariable | float], PyExpression]
    ) -> PyExpression:
        from luna_model.variable import Variable  # noqa: PLC0415

        if isinstance(other, Expression):
            res = fn(other._expr)
        elif isinstance(other, Variable):
            res = fn(other._v)
        else:
            res = fn(other)
        return res

    @classmethod
    def _cmp(
        cls, other: Expression | Variable | float, fn: Callable[[PyExpression | PyVariable | float], PyConstraint]
    ) -> Constraint:
        from luna_model.variable import Variable  # noqa: PLC0415

        if isinstance(other, Expression):
            pyc = fn(other._expr)
        elif isinstance(other, Variable):
            pyc = fn(other._v)
        else:
            pyc = fn(other)
        return wrap_c(pyc)

    @staticmethod
    def _unreduce(data: bytes, data_env: bytes) -> Expression:
        env = Environment.decode(data_env)
        return Expression.decode(data, env)
