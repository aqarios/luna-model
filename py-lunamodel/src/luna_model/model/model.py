from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Literal, overload

from dimod import BinaryQuadraticModel, ConstrainedQuadraticModel
from numpy import ndarray
from typing_extensions import deprecated

from luna_model._lm import PyModel
from luna_model._utils import wrap_cc, wrap_env, wrap_expr, wrap_s, wrap_sp, wrap_var
from luna_model.expression.expr import Expression
from luna_model.model.sense import Sense
from luna_model.ttarget import TranslationTarget
from luna_model.variable.var import Variable
from luna_model.variable.vtype import Vtype

if TYPE_CHECKING:
    from collections.abc import Callable

    from numpy.typing import NDArray

    from luna_model.constraint.collection import ConstraintCollection
    from luna_model.constraint.constr import Constraint
    from luna_model.environment.env import Environment
    from luna_model.model.specs import ModelSpecs
    from luna_model.solution.res import Result, Sample
    from luna_model.solution.sol import Solution
    from luna_model.translator.model.qubo import Qubo
    from luna_model.variable.bounds import Unbounded


class Model:
    """Model docstring."""

    _m: PyModel

    def __init__(
        self,
        name: str | None = None,
        sense: Sense = Sense.MIN,
        env: Environment | None = None,
    ) -> None:
        """Create a new model."""
        self._m = PyModel(name=name, sense=sense._val, env=env._env if env else None)

    @classmethod
    def _from_pym(cls, py_m: PyModel) -> Model:
        m = cls.__new__(cls)
        m._m = py_m
        return m

    @property
    def name(self) -> str:
        """Get the model's name."""
        return self._m.name

    @name.setter
    def name(self, name: str) -> None:
        """Set the model's name."""
        self._m.name = name

    @property
    def sense(self) -> Sense:
        """Get the model's sense."""
        return Sense._from_pysense(self._m.sense)

    @sense.setter
    def sense(self, sense: Sense) -> None:
        """Set the model's sense."""
        self._m.sense = sense._val

    @property
    def objective(self) -> Expression:
        """Get the model's objective."""
        return wrap_expr(self._m.objective)

    @objective.setter
    def objective(self, value: Expression) -> None:
        """Set the model's objective."""
        self._m.objective = value._expr

    @property
    def constraints(self) -> ConstraintCollection:
        """Get the model's constraints."""
        return wrap_cc(self._m.constraints)

    @constraints.setter
    def constraints(self, value: ConstraintCollection) -> None:
        """Set the model's constraints."""
        self._m.constraints = value._cc

    @property
    def environment(self) -> Environment:
        """Get the model's environment."""
        return wrap_env(self._m.environment)

    @property
    def num_variables(self) -> int:
        """Get the model's number of variables."""
        return self._m.num_variables

    @property
    def num_constraints(self) -> int:
        """Get the model's number of constraints."""
        return self._m.num_constraints

    def variables(self) -> list[Variable]:
        """Get the model's variables."""
        return [wrap_var(v) for v in self._m.variables()]

    def vtypes(self) -> list[Vtype]:
        """Get the model's vtypes."""
        return [Vtype._from_pyvtype(t) for t in self._m.vtypes()]

    @deprecated(
        "This method is deprecated in favor of the direct attribute setter. Will be removed in the next release."
    )
    def set_sense(self, sense: Sense) -> None:
        """Set the model's sense.

        Deprecated in favor of the direct attribute setter. Will be removed in the next release.
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
        """Add a variable to the model."""
        if with_fallback:
            return wrap_var(self._m.add_variable_with_fallback(name=name, vtype=vtype._val, lower=lower, upper=upper))
        return wrap_var(self._m.add_variable(name=name, vtype=vtype._val, lower=lower, upper=upper))

    @deprecated("This method is deprecated in favor of the add_variable(..., with_fallback=True) method.")
    def add_variable_with_fallback(
        self,
        name: str,
        vtype: Vtype = Vtype.BINARY,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> Variable:
        """Add a variable to the model with a fallback name in case it already exists.

        Deprecated in favor of the add_variable(..., with_fallback=True) method.
        """
        return self.add_variable(name, vtype, lower, upper, with_fallback=True)

    def get_variable(self, name: str) -> Variable:
        """Get a model's variable by its name."""
        return wrap_var(self._m.get_variable(name))

    def get_specs(self) -> ModelSpecs:
        """Get a model's specs."""
        return wrap_sp(self._m.get_specs())

    def add_constraint(self, constraint: Constraint, name: str | None = None) -> None:
        """Add a constraint to the model."""
        self._m.add_constraint(constraint._c, name)

    def set_objective(self, expression: Expression, sense: Sense | None = None) -> None:
        """Set the model's objective."""
        self._m.set_objective(expression._expr, sense._val if sense else None)

    def evaluate(self, solution: Solution) -> Solution:
        """Evaluate a solution producing a new solution."""
        return wrap_s(self._m.evaluate(solution._s))

    def evaluate_sample(self, sample: Sample) -> Result:
        """Evaluate a sample."""
        return self._m.evaluate_sample(sample)

    def violated_constraints(self, sample: Sample) -> ConstraintCollection:
        """Get all constraints violated by a sample."""
        return wrap_cc(self._m.violated_constraints(sample))

    def substitute(self, /, target: Variable, replacement: Expression | Variable) -> None:
        """Get all constraints violated by a sample."""
        if isinstance(replacement, Expression):  # type: ignore[attribute]
            self._m.substitute(target._v, replacement._expr)  # type: ignore[attribute]
        elif isinstance(replacement, Variable):  # type: ignore[attribute]
            self._m.substitute(target._v, replacement._v)  # type: ignore[attribute]
        else:
            msg = f"cannot use '{type(replacement)}' as a replacement in substitution"
            raise TypeError(msg)

    def satisfies(self, specs: ModelSpecs) -> bool:
        """Check if the model satisfies the specs."""
        return self._m.satisfies(specs._sp)

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Encode the model."""
        return self._m.encode(compress, level)

    def serialize(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        """Serialize the model."""
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes) -> Model:
        """Decode the model."""
        return cls._from_pym(PyModel.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Model:
        """Deserialize the model."""
        return cls.decode(data)

    def deep_clone(self) -> Model:
        """Deep clone the model."""
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
    def from_(
        cls,
        other: ConstrainedQuadraticModel | BinaryQuadraticModel | str | Path | NDArray,
        name: str | None = None,
        **kwargs,
    ) -> Model:
        """Create a model from other."""
        if isinstance(other, ConstrainedQuadraticModel):
            from luna_model.translator.model.cqm import CqmTranslator  # noqa: PLC0415

            return CqmTranslator.to_lm(other, name=name)
        if isinstance(other, BinaryQuadraticModel):
            from luna_model.translator.model.bqm import BqmTranslator  # noqa: PLC0415

            return BqmTranslator.to_lm(other, name=name)
        if isinstance(other, str | Path):
            from luna_model.translator.model.lp import LpTranslator  # noqa: PLC0415

            return LpTranslator.to_lm(other)
        if isinstance(other, ndarray):
            from luna_model.translator.model.qubo import QuboTranslator  # noqa: PLC0415

            return QuboTranslator.to_lm(other, name=name, **kwargs)
        msg = f"Unexpected type of other: '{type(other)}'"
        raise ValueError(msg)

    @overload
    def to(
        self,
        target: Literal[TranslationTarget.LP],
        filepath: Path,
    ) -> None: ...
    @overload
    def to(self, target: Literal[TranslationTarget.LP]) -> str: ...
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
        if target != TranslationTarget.LP and filepath is not None:
            msg = "filepath can only be used with target 'LP'"
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

    def equal_contents(self, other: Model) -> bool:
        """Check if two models have equal contents."""
        return self._m.equal_contents(other._m)

    def __eq__(self, other: Model) -> bool:  # type: ignore[override]
        """Check if two models are equal."""
        return self._m.__eq__(other._m)

    def __hash__(self) -> int:
        """Compute model hash."""
        return self._m.__hash__()

    def __reduce__(self) -> tuple[Callable[[bytes], Model], tuple[bytes]]:
        """Compute model hash."""
        return (Model.decode, (self.encode(),))

    def __str__(self) -> str:
        """Model as string."""
        return self._m.__str__()

    def __repr__(self) -> str:
        """Model as debug string."""
        return self._m.__repr__()
