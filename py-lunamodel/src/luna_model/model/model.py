# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
from __future__ import annotations

import sys

if sys.version_info < (3, 13):
    from typing_extensions import deprecated
else:
    from warnings import deprecated

from pathlib import Path
from typing import TYPE_CHECKING, Any, Literal, overload

from numpy import ndarray

from luna_model._lm import PyModel, PyModelMetadata
from luna_model._utils import wrap_cc, wrap_env, wrap_expr, wrap_s, wrap_sp, wrap_var
from luna_model.errors import TranslationError
from luna_model.expression.expr import Expression
from luna_model.matrix import NDLmArray
from luna_model.model.sense import Sense
from luna_model.ttarget import TranslationTarget
from luna_model.variable.var import Variable
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from collections.abc import Callable

    from dimod import BinaryQuadraticModel, ConstrainedQuadraticModel
    from numpy.typing import NDArray

    from luna_model.constraint.collection import ConstraintCollection
    from luna_model.constraint.constr import Constraint
    from luna_model.environment.env import Environment
    from luna_model.model.specs import ModelSpecs
    from luna_model.solution.res import Result, Sample
    from luna_model.solution.sol import Solution
    from luna_model.translator.model.qubo import Qubo
    from luna_model.variable.bounds import Unbounded


_msg = (
    "dimod is required for the translation from a BinaryQuadraticModel or ConstrainedQuadraticModel. "
    "You can install it using the 'dimod' extra."
)


def _bqm_type() -> type[BinaryQuadraticModel]:
    try:
        from dimod import BinaryQuadraticModel  # noqa: PLC0415

    except ImportError as e:
        raise RuntimeError(_msg) from e
    else:
        return BinaryQuadraticModel


def _cqm_type() -> type[ConstrainedQuadraticModel]:
    try:
        from dimod import ConstrainedQuadraticModel  # noqa: PLC0415

    except ImportError as e:
        raise RuntimeError(_msg) from e
    else:
        return ConstrainedQuadraticModel


class Model:
    """A symbolic optimization model combining an objective and constraints.

    The ``Model`` class represents a structured symbolic optimization problem.
    It contains an objective, a collection of ``Constraint`` objects, and an
    ``Environment`` that scopes all variables used in the model.

    Models can be constructed implicitly by allowing the model to create its
    own private environment, or explicitly by passing an environment.
    If constructed inside an active `Environment` context, that context is
    used automatically.

    Parameters
    ----------
    name : str, optional
        An optional name assigned to the model for identification and debugging.
    sense : Sense, default=Sense.MIN
        The optimization sense - `Sense.MIN` to minimize or `Sense.MAX` to maximize
        the objective function.
    env : Environment, optional
        The environment in which variables and expressions are created. If not
        provided, the model will either use the current context (if active), or
        create a new private environment.

    Attributes
    ----------
    name : str
        The name of the model.
    sense : Sense
        The optimization sense (MIN or MAX).
    objective : Expression
        The objective function to optimize.
    constraints : ConstraintCollection
        Collection of constraints that must be satisfied.
    environment : Environment
        The environment containing all variables.
    num_variables : int
        Number of variables in the model.
    num_constraints : int
        Number of constraints in the model.

    Examples
    --------
    Basic usage:

    >>> from luna_model import Model, Variable, Sense
    >>> model = Model("MyModel", sense=Sense.MAX)
    >>> x = model.add_variable("x")
    >>> y = model.add_variable("y")
    >>> model.objective = x * y + x
    >>> model.constraints += x >= 0
    >>> model.constraints += y <= 5
    >>> print(model)
    Model: MyModel
    Maximize
      x * y + x
    Subject To
      c0: x >= 0
      c1: y <= 5
    Binary
      x y

    With explicit environment:

    >>> from luna_model import Environment
    >>> env = Environment()
    >>> model = Model("ScopedModel", env=env)
    >>> x = model.add_variable("x")
    >>> model.objective = x * x
    >>> print(model)
    Model: ScopedModel
    Minimize
      x
    Binary
      x

    Serialization:

    >>> blob = model.encode()
    >>> restored = Model.decode(blob)
    >>> print(restored.name)
    ScopedModel
    """

    _m: PyModel

    def __init__(
        self,
        name: str | None = None,
        sense: Sense = Sense.MIN,
        env: Environment | None = None,
    ) -> None:
        self._m = PyModel(name=name, sense=sense._val, env=env._env if env else None)

    @classmethod
    def _from_pym(cls, py_m: PyModel) -> Model:
        m = cls.__new__(cls)
        m._m = py_m
        return m

    @property
    def name(self) -> str:
        """Get or set the model's name.

        Returns
        -------
        str
            The name of the model.

        Examples
        --------
        >>> model = Model("MyModel")
        >>> model.name
        'MyModel'
        >>> model.name = "UpdatedModel"
        >>> model.name
        'UpdatedModel'
        """
        return self._m.name

    @name.setter
    def name(self, name: str) -> None:
        self._m.name = name

    @property
    def sense(self) -> Sense:
        """Get or set the model's optimization sense.

        The sense indicates whether the objective function is to be minimized or maximized.

        Returns
        -------
        Sense
            The optimization sense (Sense.MIN or Sense.MAX).

        Examples
        --------
        >>> model = Model(sense=Sense.MIN)
        >>> model.sense
        <Sense.MIN: 'Minimize'>
        >>> model.sense = Sense.MAX
        >>> model.sense
        <Sense.MAX: 'Maximize'>
        """
        return Sense._from_pysense(self._m.sense)

    @sense.setter
    def sense(self, sense: Sense) -> None:
        self._m.sense = sense._val

    @property
    def objective(self) -> Expression:
        """Get or set the model's objective function.

        The objective function is the expression to be optimized (minimized or maximized).

        Returns
        -------
        Expression
            The objective function expression.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.objective = 2 * x + 3 * y
        >>> model.objective.degree()
        1
        """
        return wrap_expr(self._m.objective)

    @objective.setter
    def objective(self, value: Expression) -> None:
        if not isinstance(value, Expression):
            msg = f"cannot set value of type '{type(value)}' as model's objective"
            raise TypeError(msg)
        self._m.objective = value._expr

    @property
    def constraints(self) -> ConstraintCollection:
        """Get or set the model's constraint collection.

        Returns
        -------
        ConstraintCollection
            The collection of constraints in the model.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> model.constraints += x <= 5
        >>> len(model.constraints)
        1
        """
        return wrap_cc(self._m.constraints)

    @constraints.setter
    def constraints(self, value: ConstraintCollection) -> None:
        """Set the model's constraints."""
        self._m.constraints = value._cc

    @property
    def environment(self) -> Environment:
        """Get the model's environment.

        Returns
        -------
        Environment
            The environment containing all variables in this model.
        """
        return wrap_env(self._m.environment)

    @property
    def num_variables(self) -> int:
        """Get the number of variables in the model.

        Returns
        -------
        int
            The total number of variables.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.objective += x + y
        >>> model.num_variables
        2
        """
        return self._m.num_variables

    @property
    def num_constraints(self) -> int:
        """Get the number of constraints in the model.

        Returns
        -------
        int
            The total number of constraints.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> model.constraints += x <= 5
        >>> model.num_constraints
        1
        """
        return self._m.num_constraints

    def variables(self) -> list[Variable]:
        """Get all variables in the model.

        Returns
        -------
        list[Variable]
            A list of all variables in the model.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.objective += x + y
        >>> vars = model.variables()
        >>> len(vars)
        2
        """
        return [wrap_var(v) for v in self._m.variables()]

    def vtypes(self) -> list[Vtype]:
        """Get the types of all variables in the model.

        Returns
        -------
        list[Vtype]
            A list of variable types in the same order as variables().

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x", vtype=Vtype.BINARY)
        >>> y = model.add_variable("y", vtype=Vtype.INTEGER)
        >>> model.objective += x + y
        >>> model.vtypes()
        [<Vtype.BINARY: 'Binary'>, <Vtype.INTEGER: 'Integer'>]
        """
        return [Vtype._from_pyvtype(t) for t in self._m.vtypes()]

    @deprecated(
        "This method is deprecated in favor of the direct attribute setter. Will be removed in the next release."
    )
    def set_sense(self, sense: Sense) -> None:
        """Set the model's sense.

        Deprecated in favor of the direct attribute setter. Will be removed in a following release.
        """
        self._m.set_sense(sense._val)

    def add_variable(
        self,
        name: str,
        vtype: Vtype = Vtype.BINARY,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
        with_fallback: bool = False,
    ) -> Variable:
        """Add a variable to the model.

        Creates a new variable and adds it to the model's environment. If the variable
        name already exists and ``with_fallback=True``, a unique name will be generated.

        Parameters
        ----------
        name : str
            The name of the variable.
        vtype : Vtype, default=Vtype.BINARY
            The type of the variable (BINARY, SPIN, INTEGER, or REAL).
        lower : float or Unbounded, optional
            The lower bound for the variable. Only applicable for INTEGER and REAL types.
        upper : float or Unbounded, optional
            The upper bound for the variable. Only applicable for INTEGER and REAL types.
        with_fallback : bool, default=False
            If True and the name exists, a unique fallback name is generated.

        Returns
        -------
        Variable
            The newly created variable.

        Raises
        ------
        VariableExistsError
            If a variable with the same name already exists and `with_fallback=False`.
        VariableNameInvalidError
            If the variable name is invalid.
        InvalidBoundsError
            If bounds are invalid or incompatible with the variable type.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x", vtype=Vtype.BINARY)
        >>> y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=10)
        >>> z = model.add_variable("z", vtype=Vtype.REAL, lower=-1.5, upper=1.5)

        Using fallback for duplicate names:

        >>> x1 = model.add_variable("x", with_fallback=True)
        """
        if with_fallback:
            return wrap_var(self._m.add_variable_with_fallback(name=name, vtype=vtype._val, lower=lower, upper=upper))
        return wrap_var(self._m.add_variable(name=name, vtype=vtype._val, lower=lower, upper=upper))

    def add_variables(  # noqa: PLR0913
        self,
        name: str,
        shape: tuple[int, ...] | int,
        vtype: Vtype = Vtype.BINARY,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
        with_fallback: bool = False,
        delimiter: str | None = None,
    ) -> NDLmArray:
        """Add many variables to the model.

        Creates new variables and adds them to the model's environment. If the variables'
        base name already exists and ``with_fallback=True``, a unique base name will be generated.

        Parameters
        ----------
        name : str
            The base name of the variables.
        shape : tuple[int, ...]
            The shape of the returned variables array.
        vtype : Vtype, default=Vtype.BINARY
            The type of the variables (BINARY, SPIN, INTEGER, or REAL).
        lower : float or Unbounded, optional
            The lower bound for the variablse. Only applicable for INTEGER and REAL types.
        upper : float or Unbounded, optional
            The upper bound for the variables. Only applicable for INTEGER and REAL types.
        with_fallback : bool, default=False
            If True and the name exists, a unique fallback base name is generated.
        delimiter : str, optional
            A delimiter used for separation of the indicies in the variable name.

        Returns
        -------
        NDArray[Variable]
            The newly created variable array.

        Raises
        ------
        VariableExistsError
            If a variable with the same name already exists and `with_fallback=False`.
        VariableNameInvalidError
            If the variable name is invalid.
        InvalidBoundsError
            If bounds are invalid or incompatible with the variable type.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variables("x", shape=(2))
        >>> x.shape
        (2,)

        >>> model = Model()
        >>> x = model.add_variables("x", shape=(2, 2))
        >>> x.shape
        (2, 2)

        Using fallback for duplicate names:

        >>> x = model.add_variables("x", shape=(2, 2, 3), with_fallback=True)
        >>> x.shape
        (2, 2, 3)
        """
        return NDLmArray(
            self._m.add_variables(
                name=name,
                shape=shape,
                vtype=vtype._val,
                lower=lower,
                upper=upper,
                with_fallback=with_fallback,
                delimiter=delimiter,
            )
        )

    @deprecated("This method is deprecated in favor of the add_variable(..., with_fallback=True) method.")
    def add_variable_with_fallback(
        self,
        name: str,
        vtype: Vtype = Vtype.BINARY,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> Variable:
        """Add a variable to the model with a fallback name in case it already exists.

        Deprecated in favor of the :meth:add_variable(..., with_fallback=True) method.
        """
        return self.add_variable(name, vtype, lower, upper, with_fallback=True)

    def get_variable(self, name: str) -> Variable:
        """Get a variable from the model by its name.

        Parameters
        ----------
        name : str
            The name of the variable to retrieve.

        Returns
        -------
        Variable
            The variable with the specified name.

        Raises
        ------
        VariableNotExistingError
            If no variable with the given name exists in the model.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> retrieved = model.get_variable("x")
        >>> retrieved.name
        'x'
        """
        return wrap_var(self._m.get_variable(name))

    def get_specs(self) -> ModelSpecs:
        """Get the model's specifications.

        Returns the specifications describing the model's structure, including
        variable types, degree of expressions, and constraint properties.

        Returns
        -------
        ModelSpecs
            The model's specifications.

        Examples
        --------
        >>> model = Model()
        >>> specs = model.get_specs()
        >>> specs.max_num_variables
        0
        """
        return wrap_sp(self._m.get_specs())

    def add_constraint(self, constraint: Constraint, name: str | None = None) -> None:
        """Add a constraint to the model.

        Parameters
        ----------
        constraint : Constraint
            The constraint to add to the model.
        name : str, optional
            An optional name for the constraint. If not provided, an automatic
            name will be generated.

        Raises
        ------
        DuplicateConstraintNameError
            If a constraint with the same name already exists.
        ConstraintNameInvalidError
            If the constraint name is invalid. Constraint names cannot be empty
            strings and must start with an alphabetical character. Additionally,
            constraint names cannot start with ``inf`` or ``nan`` due to
            limitations of other modeling software.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> constraint = x <= 5
        >>> model.add_constraint(constraint, name="x_upper_bound")
        """
        self._m.add_constraint(constraint._c, name)

    def set_objective(self, expression: Expression, sense: Sense | None = None) -> None:
        """Set the model's objective function.

        Parameters
        ----------
        expression : Expression
            The expression to use as the objective function.
        sense : Sense, optional
            The optimization sense. If provided, also updates the model's sense.
            If None, the model's current sense is used.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> model.set_objective(x + 2 * y, sense=Sense.MAX)
        """
        self._m.set_objective(expression._expr, sense._val if sense else None)

    def evaluate(self, solution: Solution) -> Solution:
        """Evaluate a solution against the model.

        Computes objective values and checks constraint satisfaction for all
        samples in the solution.

        Parameters
        ----------
        solution : Solution
            The solution to evaluate.

        Returns
        -------
        Solution
            A new solution with updated objective values and feasibility information.

        Raises
        ------
        EvaluationError
            If evaluation fails due to incompatible solution format.
        SampleIncorrectLengthError
            If samples have incorrect number of variables.
        SampleUnexpectedVariableError
            If samples contain variables not in the model.

        Examples
        --------
        >>> from luna_model import Model, Solution, Sense, Variable, Vtype
        >>> model = Model(sense=Sense.MAX)
        >>> x = model.add_variable("x", vtype=Vtype.INTEGER)
        >>> y = model.add_variable("y", vtype=Vtype.INTEGER)
        >>> model.objective = x + 2 * y
        >>> model.constraints += x + y <= 5
        >>> samples = [{"x": 1, "y": 2}]
        >>> with model.environment:
        ...     solution = Solution(samples=samples)
        >>> solution = model.evaluate(solution)
        >>> solution.obj_values[0]  # Value of x + 2*y = 1 + 4 = 5
        np.float64(5.0)
        >>> solution[0].feasible  # x + y = 3 <= 5, so True
        True
        """
        return wrap_s(self._m.evaluate(solution._s))

    def evaluate_sample(self, sample: Sample) -> Result:
        """Evaluate a single sample against the model.

        Computes the objective value and checks constraint satisfaction for
        a single variable assignment.

        Parameters
        ----------
        sample : Sample
            A dictionary mapping variable names to their values.

        Returns
        -------
        Result
            Evaluation result containing objective value and feasibility information.

        Raises
        ------
        SampleIncorrectLengthError
            If the sample has incorrect number of variables.
        SampleUnexpectedVariableError
            If the sample contains variables not in the model.
        """
        return self._m.evaluate_sample(sample)

    def violated_constraints(self, sample: Sample) -> ConstraintCollection:
        """Get all constraints violated by a sample.

        Parameters
        ----------
        sample : Sample
            A dictionary mapping variable names to their values.

        Returns
        -------
        ConstraintCollection
            Collection containing only the constraints that are violated by the sample.
        """
        return wrap_cc(self._m.violated_constraints(sample))

    def substitute(self, /, target: Variable, replacement: Expression | Variable) -> None:
        """Substitute a variable with an expression or another variable.

        Replaces all occurrences of the target variable in the objective and
        constraints with the replacement expression or variable.

        Parameters
        ----------
        target : Variable
            The variable to be replaced.
        replacement : Expression or Variable
            The expression or variable to substitute in place of the target.

        Raises
        ------
        TypeError
            If replacement is neither an Expression nor a Variable.
        DifferentEnvironmentsError
            If the replacement is from a different environment.

        Examples
        --------
        >>> model = Model()
        >>> x = model.add_variable("x")
        >>> y = model.add_variable("y")
        >>> z = model.add_variable("z")
        >>> model.objective = x + 2 * y
        >>> model.substitute(y, z + 1)  # Replace y with (z + 1)
        >>> # Objective becomes: x + 2*(z + 1) = x + 2z + 2

        Notes
        -----
        This operation modifies the model in place. The target variable remains
        in the model's environment but is no longer used.
        """
        if isinstance(replacement, Expression):  # type: ignore[attribute]
            self._m.substitute(target._v, replacement._expr)  # type: ignore[attribute]
        elif isinstance(replacement, Variable):  # type: ignore[attribute]
            self._m.substitute(target._v, replacement._v)  # type: ignore[attribute]
        else:
            msg = f"cannot use '{type(replacement)}' as a replacement in substitution"
            raise TypeError(msg)

    def satisfies(self, specs: ModelSpecs) -> bool:
        """Check if the model satisfies given specifications.

        Parameters
        ----------
        specs : ModelSpecs
            The specifications to check against.

        Returns
        -------
        bool
            True if the model satisfies all specifications, False otherwise.

        Examples
        --------
        >>> model = Model()
        >>> specs = model.get_specs()
        >>> model.satisfies(specs)
        True
        """
        return self._m.satisfies(specs._sp)

    def encode(self) -> bytes:
        """Serialize the model into a compact binary format.

        Returns
        -------
        bytes
            Encoded model representation.

        Examples
        --------
        >>> model = Model("MyModel")
        >>> blob = model.encode()
        >>> restored = Model.decode(blob)
        >>> restored.name
        'MyModel'
        """
        return self._m.encode()

    def serialize(self) -> bytes:
        """Serialize the model into a compact binary format.

        This is an alias for :meth:`encode`.

        Returns
        -------
        bytes
            Encoded model representation.
        """
        return self.encode()

    @classmethod
    def decode(cls, data: bytes) -> Model:
        """Reconstruct a model from encoded bytes.

        Parameters
        ----------
        data : bytes
            Binary blob returned by :meth:`encode` or :meth:`serialize`.

        Returns
        -------
        Model
            Deserialized model object.

        Raises
        ------
        DecodingError
            If decoding fails due to corruption or incompatibility.

        Examples
        --------
        >>> original = Model("MyModel")
        >>> blob = original.encode()
        >>> restored = Model.decode(blob)
        >>> restored.name == original.name
        True
        """
        return cls._from_pym(PyModel.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Model:
        """Reconstruct a model from encoded bytes.

        This is an alias for :meth:`decode`.

        Parameters
        ----------
        data : bytes
            Binary blob returned by encode().

        Returns
        -------
        Model
            Deserialized model object.
        """
        return cls.decode(data)

    def deep_clone(self) -> Model:
        """Create a deep copy of the model.

        Returns
        -------
        Model
            A new model that is an independent copy of this model, including
            its own environment, variables, constraints, and objective.

        Examples
        --------
        >>> from luna_model import Model
        >>> original = Model("Original")
        >>> x = original.add_variable("x")
        >>> original.objective = 2 * x
        >>> clone = original.deep_clone()
        >>> clone.name = "Clone"
        >>> original.name
        'Original'
        """
        return self._from_pym(self._m.deep_clone())

    @overload
    @classmethod
    def from_(
        cls,
        other: ConstrainedQuadraticModel | BinaryQuadraticModel | str | Path,
        name: str | None = None,
    ) -> Model: ...
    @overload
    @classmethod
    def from_(
        cls,
        other: NDArray,
        name: str | None = None,
        *,
        offset: float | None = None,
        variable_names: list[str] | None = None,
        vtype: Vtype | None = None,
    ) -> Model: ...
    @classmethod
    def from_(  # noqa: PLR0911
        cls,
        other: ConstrainedQuadraticModel | BinaryQuadraticModel | str | Path | NDArray,
        name: str | None = None,
        **kwargs,
    ) -> Model:
        """Create a model from other."""
        if isinstance(other, str):
            # Either LP or MPS as string
            lp_error = None
            mps_error = None
            try:
                from luna_model.translator.model.lp import LpTranslator  # noqa: PLC0415

                return LpTranslator.to_lm(other)
            except TranslationError as e:
                lp_error = e

            try:
                from luna_model.translator.model.mps import MpsTranslator  # noqa: PLC0415

                return MpsTranslator.to_lm(other)
            except TranslationError as e:
                mps_error = e

            msg = "the string contents cannot be parsed to a Model."
            msg += " "
            msg += f"Encountered the following errors:\n\tLP: {lp_error}\n\tMPS: {mps_error}"
            raise ValueError(msg)

        if isinstance(other, Path):
            # Either LP or MPS file
            if other.suffix == ".lp":
                from luna_model.translator.model.lp import LpTranslator  # noqa: PLC0415

                return LpTranslator.to_lm(other)

            if other.suffix == ".mps":
                from luna_model.translator.model.mps import MpsTranslator  # noqa: PLC0415

                return MpsTranslator.to_lm(other)
            msg = f"unknown file type '{other.suffix}', only '.mps' and '.lp' files are supported"
            raise ValueError(msg)

        if isinstance(other, ndarray):
            from luna_model.translator.model.qubo import QuboTranslator  # noqa: PLC0415

            return QuboTranslator.to_lm(other, name=name, **kwargs)

        if isinstance(other, _cqm_type()):
            from luna_model.translator.model.cqm import CqmTranslator  # noqa: PLC0415

            return CqmTranslator.to_lm(other, name=name)
        if isinstance(other, _bqm_type()):
            from luna_model.translator.model.bqm import BqmTranslator  # noqa: PLC0415

            return BqmTranslator.to_lm(other, name=name)
        msg = f"Unexpected type of other: '{type(other)}'"
        raise ValueError(msg)

    @overload
    def to(
        self,
        target: Literal[TranslationTarget.LP],
        filepath: Path,
    ) -> None: ...
    @overload
    def to(
        self,
        target: Literal[TranslationTarget.MPS],
        filepath: Path,
    ) -> None: ...
    @overload
    def to(self, target: Literal[TranslationTarget.LP]) -> str: ...
    @overload
    def to(self, target: Literal[TranslationTarget.MPS]) -> str: ...
    @overload
    def to(self, target: Literal[TranslationTarget.CQM]) -> ConstrainedQuadraticModel: ...
    @overload
    def to(self, target: Literal[TranslationTarget.BQM]) -> BinaryQuadraticModel: ...
    @overload
    def to(self, target: Literal[TranslationTarget.QUBO]) -> Qubo: ...
    def to(
        self,
        target: TranslationTarget,
        filepath: Path | None = None,
    ) -> Qubo | str | BinaryQuadraticModel | ConstrainedQuadraticModel | None:
        """Translate model to target."""
        if target not in (TranslationTarget.LP, TranslationTarget.MPS) and filepath is not None:
            msg = "filepath can only be used with target 'LP' and 'MPS'"
            raise ValueError(msg)
        match target:
            case TranslationTarget.BQM:
                from luna_model.translator.model.bqm import BqmTranslator  # noqa: PLC0415

                return BqmTranslator.from_lm(self)
            case TranslationTarget.CQM:
                from luna_model.translator.model.cqm import CqmTranslator  # noqa: PLC0415

                return CqmTranslator.from_lm(self)
            case TranslationTarget.QUBO:
                from luna_model.translator.model.qubo import QuboTranslator  # noqa: PLC0415

                return QuboTranslator.from_lm(self)
            case TranslationTarget.LP:
                from luna_model.translator.model.lp import LpTranslator  # noqa: PLC0415

                return LpTranslator.from_lm(self, filepath)
            case TranslationTarget.MPS:
                from luna_model.translator.model.mps import MpsTranslator  # noqa: PLC0415

                return MpsTranslator.from_lm(self, filepath)

    def equal_contents(self, other: Model) -> bool:
        """Check if two models have equal contents."""
        return self._m.equal_contents(other._m)

    def __eq__(self, other: Model) -> bool:  # type: ignore[override]
        """Check if two models are exactly equal.

        Two models are equal if they have the same structure, variables,
        constraints, and objective.

        Parameters
        ----------
        other : Model
            The model to compare with.

        Returns
        -------
        bool
            True if models are equal, False otherwise.

        Examples
        --------
        >>> model1 = Model("A")
        >>> model2 = Model("B")
        >>> model1 == model2
        False
        """
        return self._m.__eq__(other._m)

    def __hash__(self) -> int:
        """Compute hash value for the model.

        Returns
        -------
        int
            Hash value based on the model's structure.

        Examples
        --------
        >>> model = Model("MyModel")
        >>> hash(model)  # doctest: +SKIP
        4726318758077234822
        """
        return self._m.__hash__()

    def __reduce__(self) -> tuple[Callable[[bytes], Model], tuple[bytes]]:
        """Support for pickle serialization.

        Returns
        -------
        tuple
            A tuple of (decoder_function, encoded_data) for pickle.

        Examples
        --------
        >>> import pickle
        >>> model = Model("MyModel")
        >>> pickled = pickle.dumps(model)
        >>> restored = pickle.loads(pickled)
        >>> restored.name
        'MyModel'

        Notes
        -----
        This method is called automatically by Python's pickle module.
        It uses the model's encode/decode methods internally.
        """
        return (Model.decode, (self.encode(),))

    def __str__(self) -> str:
        """Get human-readable string representation of the model.

        Returns
        -------
        str
            A formatted string showing the model's name, sense, objective,
            constraints, and variables.

        Examples
        --------
        >>> from luna_model import Model, Sense
        >>> model = Model("MyModel", sense=Sense.MAX)
        >>> x = model.add_variable("x")
        >>> model.objective = 2 * x
        >>> print(model)
        Model: MyModel
        Maximize
          2 * x
        Binary
          x
        """
        return self._m.__str__()

    def __repr__(self) -> str:
        """Get detailed debug representation of the model.

        Returns
        -------
        str
            A string representation suitable for debugging.

        Examples
        --------
        >>> model = Model("MyModel")
        >>> repr(model)  # doctest: +SKIP
        'Model(name=MyModel, sense=Minimize, objective=0, constraints={})'

        Notes
        -----
        This representation may include internal details useful for debugging
        but is not guaranteed to be stable across versions.
        """
        return self._m.__repr__()

    class _ModelMetadata(PyModelMetadata):
        def __len__(self) -> int:
            return super().__len__()

        def __contains__(self, key: str) -> bool:
            return super().__contains__(key)

        def __getitem__(self, key: str) -> Any:  # noqa: ANN401
            return super().__getitem__(key)

        def __setitem__(self, key: str, value: Any) -> None:  # noqa: ANN401
            return super().__setitem__(key, value)

        def __delitem__(self, key: str) -> None:
            return super().__delitem__(key)

        def __str__(self) -> str:
            return super().__str__()

        def __repr__(self) -> str:
            return super().__repr__()

        def get_item(self, key: str) -> Any:  # noqa: ANN401
            return super().get_item(key)

        def set_item(self, key: str, value: Any) -> None:  # noqa: ANN401
            return super().set_item(key, value)

        def del_item(self, key: str) -> None:
            return super().del_item(key)

        def to_dict(self) -> dict[str, Any]:
            return super().to_dict()

    @property
    def _metadata(self) -> _ModelMetadata:
        return self._m._metadata

    @_metadata.setter
    def _metadata(self, value: _ModelMetadata) -> None:
        self._m._metadata = value
