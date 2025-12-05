from __future__ import annotations
from pathlib import Path
from typing import TYPE_CHECKING

from luna_model.environment.environment import Environment
from luna_model.constraint.constr import Constraint
from luna_model.constraint.collection import ConstraintCollection
from luna_model.expression.expr import Expression
from luna_model.solution.sol import Solution
from luna_model.solution.res import Result
from luna_model.solution.sample import Sample
from luna_model.variable.var import Variable
from luna_model.variable.vtype import Vtype
from luna_model.variable.bounds import Unbounded
from luna_model.model.sense import Sense
from luna_model.model.specs import ModelSpecs
from luna_model.translator.ttarget import TranslationTarget

from luna_model._lm import PyModel

if TYPE_CHECKING:
    from dimod import BinaryQuadraticModel, ConstrainedQuadraticModel  # type: ignore[import]
    from numpy.typing import NDArray  # type: ignore[import]
    from luna_model.translator.qubo import Qubo


class Model:
    _m: PyModel

    def __init__(
        self,
        name: str | None = None,
        sense: Sense = Sense.MIN,
        env: Environment | None = None,
    ):
        self._m = PyModel(name=name, sense=sense.value, env=env._env if env else None)

    @classmethod
    def _from_pym(cls, py_m: PyModel) -> Model:
        m = cls.__new__(cls)
        m._m = py_m
        return m

    @property
    def name(self) -> str:
        return self._m.name

    @name.setter
    def name(self, name: str) -> None:
        self._m.name = name

    @property
    def sense(self) -> Sense:
        return Sense(self._m.sense)

    @property
    def objective(self) -> Expression:
        return Expression._from_pyexpr(self._m.objective)

    @objective.setter
    def objective(self, value: Expression) -> None:
        self._m.objective = value._expr

    @property
    def constraints(self) -> ConstraintCollection:
        return ConstraintCollection._from_pycc(self._m.constraints)

    @constraints.setter
    def constraints(self, value: ConstraintCollection) -> None:
        self._m.constraints = value._cc

    @property
    def environment(self) -> Environment:
        return Environment._from_pyenv(self._m.environment)

    @property
    def num_variables(self) -> int:
        return self._m.num_variables

    @property
    def num_constraints(self) -> int:
        return self._m.num_constraints

    def variables(self) -> list[Variable]:
        return [Variable._from_pyvar(v) for v in self._m.variables]

    def vtypes(self) -> list[Vtype]:
        return [Vtype(t) for t in self._m.vtypes()]

    # todo: deprecate for property setter
    def set_sense(self, sense: Sense) -> None:
        self._m.set_sense(sense.value)

    def add_variable(
        self,
        name: str,
        vtype: Vtype = Vtype.BINARY,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> Variable:
        return Variable._from_pyvar(
            self._m.add_variable(name=name, vtype=vtype.value, lower=lower, upper=upper)
        )

    # todo: deprecate this and make it param in add_variable.
    def add_variable_with_fallback(
        self,
        name: str,
        vtype: Vtype = Vtype.BINARY,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> Variable:
        return Variable._from_pyvar(
            self._m.add_variable_with_fallback(
                name=name, vtype=vtype.value, lower=lower, upper=upper
            )
        )

    def get_variable(self, name: str) -> Variable:
        return Variable._from_pyvar(self._m.get_variable(name))

    def get_specs(self) -> ModelSpecs:
        return ModelSpecs._from_pysp(self._m.get_specs())

    def add_constraint(self, constraint: Constraint, name: str | None = None) -> None:
        self._m.add_constraint(constraint._c, name)

    def set_objective(self, expression: Expression, sense: Sense | None = None) -> None:
        self._m.set_objective(expression._expr, sense.value if sense else None)

    def evaluate(self, solution: Solution) -> Solution:
        return Solution._from_pys(self._m.evaluate(solution._s))

    def evaluate_sample(self, sample: Sample) -> Result:
        return self._m.evaluate(sample)

    def violated_constraints(self, sample: Sample) -> ConstraintCollection:
        return ConstraintCollection._from_pycc(self._m.violated_constraints(sample))

    def substitute(
        self, /, target: Variable, replacement: Expression | Variable
    ) -> None:
        if isinstance(replacement, Expression):
            self._m.substitute(target._v, replacement._expr)
        elif isinstance(replacement, Variable):
            self._m.substitute(target._v, replacement._v)
        else:
            raise TypeError(
                f"cannot use '{type(replacement)}' as a replacement in substitution"
            )

    def satisfies(self, specs: ModelSpecs) -> bool:
        return self._m.satisfies(specs._sp)

    def encode(self, /, compress: bool | None = True, level: int | None = 3) -> bytes:
        return self._m.encode(compress, level)

    def serialize(
        self, /, compress: bool | None = True, level: int | None = 3
    ) -> bytes:
        return self.encode(compress, level)

    @classmethod
    def decode(cls, data: bytes) -> Model:
        return cls._from_pym(PyModel.decode(data))

    @classmethod
    def deserialize(cls, data: bytes) -> Model:
        return cls.decode(data)

    def deep_clone(self) -> Model:
        return Model._from_pym(self._m.deep_clone())

    @classmethod
    def from_(
        cls,
        other: ConstrainedQuadraticModel | BinaryQuadraticModel | str | Path | NDArray,
        name: str | None = None,
        **kwargs,
    ) -> Model:
        return cls._from_pym(cls._m.from_(other, name=name, **kwargs))

    def to(
        self,
        target: TranslationTarget,
        filepath: Path | None = None,
    ) -> Qubo | str | BinaryQuadraticModel | ConstrainedQuadraticModel | None:
        return self._m.to(target.value, filepath)

    def equal_contents(self, other: Model) -> bool:
        return self._m.equal_contents(other._m)

    def __eq__(self, other: Model) -> bool:  # type: ignore[override]
        return self._m.__eq__(other._m)

    def __hash__(self) -> int:
        return self._m.__hash__()
