"""Mathematical expressions for optimization models.

This module provides the Expression class for representing mathematical expressions
composed of variables, constants, and arithmetic operations. Expressions form the
building blocks of objective functions and constraints in optimization models.
"""

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
    """Mathematical expression combining variables and constants.

    An Expression represents a mathematical formula built from variables,
    constants, and arithmetic operations (+, -, *, **). Expressions can be
    linear, quadratic, or higher-order polynomial forms.

    Expressions are used to define:
    - Objective functions to minimize or maximize
    - Left-hand and right-hand sides of constraints
    - Substitution formulas for variable transformations

    Parameters
    ----------
    env : Environment | None, optional
        The environment for this expression. If None, creates or uses the
        current active environment.

    Attributes
    ----------
    environment : Environment
        The environment containing this expression.
    num_variables : int
        Number of variables with non-zero coefficients in the expression.

    Examples
    --------
    Create expressions from variables:

    >>> from luna_model import Variable
    >>> x = Variable("x")
    >>> y = Variable("y")
    >>> expr = 3*x + 2*y - 5

    Create quadratic expression:

    >>> z = Variable("z")
    >>> quad_expr = x*y + z**2

    Create constant expression:

    >>> from luna_model.expression import Expression
    >>> const_expr = Expression.const(42.0)

    Access expression terms:

    >>> for variables, bias in expr.items():
    ...     print(f"{variables}: {bias}")

    Notes
    -----
    Expressions are immutable by default. Arithmetic operations create new
    Expression objects unless using in-place operators (+=, -=, *=).

    See Also
    --------
    Variable : Variables that compose expressions.
    Constraint : Constraints created by comparing expressions.
    Model : Models that use expressions as objectives.
    """

    _expr: PyExpression

    def __init__(self, env: Environment | None = None) -> None:
        """Initialize an empty expression.
        
        Parameters
        ----------
        env : Environment | None, optional
            The environment for this expression.
        """
        if env is None:
            self._expr = PyExpression()
        else:
            self._expr = PyExpression(env._env)

    @classmethod
    def _from_pyexpr(cls, py_expr: PyExpression) -> Expression:
        """Construct Expression from internal PyExpression object.
        
        Parameters
        ----------
        py_expr : PyExpression
            Internal expression representation.
            
        Returns
        -------
        Expression
            New Expression wrapping the PyExpression.
        """
        expr = cls.__new__(cls)
        expr._expr = py_expr
        return expr

    @classmethod
    def const(cls, value: float, /, env: Environment | None = None) -> Expression:
        """Create a constant expression.
        
        Parameters
        ----------
        value : float
            The constant value.
        env : Environment | None, optional
            The environment for this expression.
            
        Returns
        -------
        Expression
            An expression representing the constant value.
            
        Examples
        --------
        >>> from luna_model.expression import Expression
        >>> const = Expression.const(5.0)
        """
        return cls._from_pyexpr(PyExpression.const(value, env._env if env else None))

    @property
    def environment(self) -> Environment:
        """Get the environment containing this expression.
        
        Returns
        -------
        Environment
            The environment this expression belongs to.
        """
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
        """Get the constant offset of the expression.
        
        Returns
        -------
        float
            The constant term in the expression.
        """
        return self._expr.get_offset()

    def get_linear(self, variable: Variable) -> float:
        """Get the linear coefficient for a variable.
        
        Parameters
        ----------
        variable : Variable
            The variable to query.
            
        Returns
        -------
        float
            The coefficient of the variable, or 0 if not present.
        """
        return self._expr.get_linear(variable._v)

    def get_quadratic(self, u: Variable, v: Variable) -> float:
        """Get the quadratic coefficient for a variable pair.
        
        Parameters
        ----------
        u : Variable
            The first variable.
        v : Variable
            The second variable.
            
        Returns
        -------
        float
            The coefficient of the u*v term, or 0 if not present.
        """
        return self._expr.get_quadratic(u._v, v._v)

    def get_higher_order(self, *variables: Variable) -> float:
        """Get the higher-order coefficient for a variable tuple.
        
        Parameters
        ----------
        *variables : Variable
            The variables in the higher-order term.
            
        Returns
        -------
        float
            The coefficient of the higher-order term, or 0 if not present.
        """
        return self._expr.get_higher_order([v._v for v in variables])

    def items(self) -> ExprIter:
        """Get an iterator over all terms in the expression.
        
        Returns
        -------
        ExprIter
            Iterator yielding (variables, coefficient) tuples for each term.
            
        Examples
        --------
        >>> x, y = Variable("x"), Variable("y")
        >>> expr = 3*x + 2*x*y + 5
        >>> for vars, coeff in expr.items():
        ...     print(f"{vars}: {coeff}")
        """
        return ExprIter._from_pyei(self._expr.items())

    def variables(self) -> list[Variable]:
        """Get all variables in the expression.
        
        Returns
        -------
        list[Variable]
            List of all variables appearing in the expression.
        """
        return [wrap_var(v) for v in self._expr.variables()]

    def degree(self) -> int:
        """Get the degree of the expression.
        
        Returns
        -------
        int
            The highest degree of any term (0=constant, 1=linear, 2=quadratic, etc.).
        """
        return self._expr.degree()

    def linear_items(self) -> list[tuple[Variable, float]]:
        """Get all linear terms in the expression.
        
        Returns
        -------
        list[tuple[Variable, float]]
            List of (variable, coefficient) tuples for linear terms.
        """
        return [(wrap_var(v), b) for v, b in self._expr.linear_items()]

    def quadratic_items(self) -> list[tuple[Variable, Variable, float]]:
        """Get all quadratic terms in the expression.
        
        Returns
        -------
        list[tuple[Variable, Variable, float]]
            List of (var1, var2, coefficient) tuples for quadratic terms.
        """
        return [(wrap_var(u), wrap_var(v), b) for u, v, b in self._expr.quadratic_items()]

    def higher_order_items(self) -> list[tuple[list[Variable], float]]:
        """Get all higher-order terms in the expression.
        
        Returns
        -------
        list[tuple[list[Variable], float]]
            List of (variables_list, coefficient) tuples for higher-order terms.
        """
        return [([wrap_var(v) for v in variables], b) for variables, b in self._expr.higher_order_items()]

    def is_constant(self) -> bool:
        """Check if the expression is a constant (no variables).
        
        Returns
        -------
        bool
            True if the expression contains no variables.
        """
        return self._expr.is_constant()

    def has_quadratic(self) -> bool:
        """Check if the expression has quadratic terms.
        
        Returns
        -------
        bool
            True if the expression contains at least one quadratic term.
        """
        return self._expr.has_quadratic()

    def has_higher_order(self) -> bool:
        """Check if the expression has higher-order terms.
        
        Returns
        -------
        bool
            True if the expression contains at least one term of degree > 2.
        """
        return self._expr.has_higher_order()

    def is_equal(self, other: Expression) -> bool:
        """Check if two expressions are structurally equal.
        
        Parameters
        ----------
        other : Expression
            The expression to compare with.
            
        Returns
        -------
        bool
            True if expressions are structurally equal.
        """
        return self._expr.is_equal(other._expr)

    def equal_contents(self, other: Expression) -> bool:
        """Check if two expressions have equal terms and coefficients.
        
        Parameters
        ----------
        other : Expression
            The expression to compare with.
            
        Returns
        -------
        bool
            True if expressions have the same terms with same coefficients.
        """
        return self._expr.equal_contents(other._expr)

    def separate(self, variables: list[Variable]) -> tuple[Expression, Expression]:
        """Separate expression into two parts based on variables.
        
        Splits the expression so that all specified variables appear only in
        the first returned expression.
        
        Parameters
        ----------
        variables : list[Variable]
            Variables to isolate in the first expression.
            
        Returns
        -------
        tuple[Expression, Expression]
            Two expressions: (terms_with_variables, terms_without_variables).
        """
        lhs, rhs = self._expr.separate([v._v for v in variables])
        return (self._from_pyexpr(lhs), self._from_pyexpr(rhs))

    def substitute(self, target: Variable, replacement: Expression | Variable) -> Expression:
        """Substitute a variable with an expression or another variable.
        
        Parameters
        ----------
        target : Variable
            The variable to replace.
        replacement : Expression | Variable
            The expression or variable to substitute in place of target.
            
        Returns
        -------
        Expression
            New expression with the substitution applied.
            
        Examples
        --------
        >>> x, y, z = Variable("x"), Variable("y"), Variable("z")
        >>> expr = 2*x + 3*y
        >>> new_expr = expr.substitute(x, z + 1)  # Replace x with z+1
        """
        from luna_model.variable import Variable  # noqa: PLC0415

        if isinstance(replacement, Variable):
            return self._from_pyexpr(self._expr.substitute(target._v, replacement._v))
        if isinstance(replacement, Expression):
            return self._from_pyexpr(self._expr.substitute(target._v, replacement._expr))
        msg = f"type '{type(replacement)}' not supported in substitution"
        raise TypeError(msg)

    def evaluate(self, solution: Solution) -> NDArray:
        """Evaluate the expression using variable values from a solution.
        
        Parameters
        ----------
        solution : Solution
            The solution containing variable assignments.
            
        Returns
        -------
        NDArray
            The evaluated value(s) of the expression.
        """
        return self._expr.evaluate(solution._s)

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Encode the expression to bytes for serialization.
        
        Parameters
        ----------
        compress : bool | None, optional
            Whether to compress the data, by default True.
        level : int | None, optional
            Compression level (0-9), by default 3.
            
        Returns
        -------
        bytes
            Encoded expression data.
        """
        return self._expr.encode(compress, level)

    def serialize(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Serialize the expression to bytes. Alias for encode.
        
        Parameters
        ----------
        compress : bool | None, optional
            Whether to compress the data, by default True.
        level : int | None, optional
            Compression level (0-9), by default 3.
            
        Returns
        -------
        bytes
            Serialized expression data.
        """
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes, env: Environment) -> Expression:
        """Decode an expression from bytes.
        
        Parameters
        ----------
        data : bytes
            Encoded expression data.
        env : Environment
            The environment to decode the expression into.
            
        Returns
        -------
        Expression
            The decoded expression.
        """
        return cls._from_pyexpr(PyExpression.decode(data, env._env))

    @classmethod
    def deserialize(cls, data: bytes, env: Environment) -> Expression:
        """Deserialize an expression from bytes. Alias for decode.
        
        Parameters
        ----------
        data : bytes
            Serialized expression data.
        env : Environment
            The environment to deserialize into.
            
        Returns
        -------
        Expression
            The deserialized expression.
        """
        return cls.decode(data, env)

    @classmethod
    def deep_clone_many(cls, exprs: list[Expression]) -> list[Expression]:
        """Deep clone multiple expressions, each with its own new environment.
        
        Parameters
        ----------
        exprs : list[Expression]
            The expressions to clone.
            
        Returns
        -------
        list[Expression]
            List of cloned expressions, each in a new environment.
        """
        return [cls._from_pyexpr(cloned) for cloned in PyExpression.deep_clone_many([e._expr for e in exprs])]

    def __add__(self, other: Expression | Variable | float) -> Expression:
        """Add another term to this expression.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to add.

        Returns
        -------
        Expression
            A new expression representing the sum.

        Examples
        --------
        >>> from luna_model import Expression, Variable
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> expr = Expression()
        >>> expr = expr + x  # Add variable
        >>> expr = expr + y  # Add another variable
        >>> expr = expr + 5.0  # Add constant
        """
        return self._from_pyexpr(self._op(other, self._expr.__add__))

    def __sub__(self, other: Expression | Variable | float) -> Expression:
        """Subtract another term from this expression.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to subtract.

        Returns
        -------
        Expression
            A new expression representing the difference.
        """
        return self._from_pyexpr(self._op(other, self._expr.__sub__))

    def __mul__(self, other: Expression | Variable | float) -> Expression:
        """Multiply this expression by another term.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to multiply by.

        Returns
        -------
        Expression
            A new expression representing the product.

        Examples
        --------
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> expr = 2 * x
        >>> expr = expr * y  # Creates quadratic term
        >>> expr = expr * 3  # Scale by constant
        """
        return self._from_pyexpr(self._op(other, self._expr.__mul__))

    def __radd__(self, other: Expression | Variable | float) -> Expression:
        """Add this expression to another term (right operand).

        Parameters
        ----------
        other : Expression | Variable | float
            The term to add this expression to.

        Returns
        -------
        Expression
            A new expression representing the sum.
        """
        return self._from_pyexpr(self._op(other, self._expr.__radd__))

    def __rsub__(self, other: Expression | Variable | float) -> Expression:
        """Subtract this expression from another term (right operand).

        Parameters
        ----------
        other : Expression | Variable | float
            The term to subtract this expression from.

        Returns
        -------
        Expression
            A new expression representing the difference.
        """
        return self._from_pyexpr(self._op(other, self._expr.__rsub__))

    def __rmul__(self, other: Expression | Variable | float) -> Expression:
        """Multiply another term by this expression (right operand).

        Parameters
        ----------
        other : Expression | Variable | float
            The term to multiply by this expression.

        Returns
        -------
        Expression
            A new expression representing the product.
        """
        return self._from_pyexpr(self._op(other, self._expr.__rmul__))

    def __iadd__(self, other: Expression | Variable | float) -> Self:
        """Add another term to this expression in-place.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to add.

        Returns
        -------
        Expression
            This expression modified in-place.

        Examples
        --------
        >>> expr = Expression()
        >>> expr += x
        >>> expr += y
        """
        self._op(other, self._expr.__iadd__)
        return self

    def __isub__(self, other: Expression | Variable | float) -> Self:
        """Subtract another term from this expression in-place.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to subtract.

        Returns
        -------
        Expression
            This expression modified in-place.
        """
        self._op(other, self._expr.__isub__)
        return self

    def __imul__(self, other: Expression | Variable | float) -> Self:
        """Multiply this expression by another term in-place.

        Parameters
        ----------
        other : Expression | Variable | float
            The term to multiply by.

        Returns
        -------
        Expression
            This expression modified in-place.
        """
        self._op(other, self._expr.__imul__)
        return self

    def __pow__(self, value: int) -> Expression:
        """Raise this expression to an integer power.

        Parameters
        ----------
        value : int
            The exponent (must be a non-negative integer).

        Returns
        -------
        Expression
            A new expression representing this expression raised to the power.

        Examples
        --------
        >>> x = Variable("x")
        >>> expr = x ** 2  # Quadratic
        >>> expr = x ** 3  # Cubic
        """
        return self._from_pyexpr(self._op(value, self._expr.__pow__))

    def __ipow__(self, other: int) -> Self:
        """Raise this expression to an integer power in-place.

        Parameters
        ----------
        other : int
            The exponent (must be a non-negative integer).

        Returns
        -------
        Expression
            This expression modified in-place.
        """
        self._op(other, self._expr.__ipow__)
        return self

    def __neg__(self) -> Expression:
        """Negate this expression.

        Returns
        -------
        Expression
            A new expression representing the negation.

        Examples
        --------
        >>> x = Variable("x")
        >>> expr = -x
        >>> expr = -(x + y)
        """
        return self._from_pyexpr(self._expr.__neg__())

    def __eq__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create an equality constraint.

        Parameters
        ----------
        other : Expression | Variable | float
            The right-hand side of the equality.

        Returns
        -------
        Constraint
            A constraint representing ``self == other``.

        Examples
        --------
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> constraint = (x + y) == 10
        """
        return self._cmp(other, self._expr.__eq__)

    def __le__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create a less-than-or-equal-to constraint.

        Parameters
        ----------
        other : Expression | Variable | float
            The right-hand side of the inequality.

        Returns
        -------
        Constraint
            A constraint representing ``self <= other``.

        Examples
        --------
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> constraint = (x + y) <= 100
        """
        return self._cmp(other, self._expr.__le__)

    def __ge__(self, other: Expression | Variable | float) -> Constraint:  # type: ignore[override]
        """Create a greater-than-or-equal-to constraint.

        Parameters
        ----------
        other : Expression | Variable | float
            The right-hand side of the inequality.

        Returns
        -------
        Constraint
            A constraint representing ``self >= other``.

        Examples
        --------
        >>> x = Variable("x")
        >>> constraint = (x + y) >= 0
        """
        return self._cmp(other, self._expr.__ge__)

    def __reduce__(self) -> tuple[Callable[[bytes, bytes], Expression], tuple[bytes, ...]]:
        """Support for pickle serialization.

        Returns
        -------
        tuple
            A tuple of (decoder_function, encoded_data) for pickle.

        Notes
        -----
        This method is called automatically by Python's pickle module.
        """
        data = self.encode()
        env_data = self.environment.encode()

        return Expression._unreduce, (data, env_data)

    def __str__(self) -> str:
        """Get human-readable string representation.

        Returns
        -------
        str
            A string showing the expression structure.

        Examples
        --------
        >>> x = Variable("x")
        >>> y = Variable("y")
        >>> expr = 3*x + 2*y + 5
        >>> str(expr)
        '3*x + 2*y + 5'
        """
        return self._expr.__str__()

    def __repr__(self) -> str:
        """Get detailed debug representation.

        Returns
        -------
        str
            A string representation suitable for debugging.
        """
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
