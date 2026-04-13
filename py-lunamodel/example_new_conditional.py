from __future__ import annotations
from collections.abc import Callable

from luna_model import Model, Sense, Vtype
from luna_model.transformation import PassManager, PassContext, ControlFlowPass, Pass
from luna_model.transformation.analysis import AnalysisPass
from luna_model.transformation.control_flow import ControlFlowPlan
from luna_model.transformation.passes import IntegerToBinaryPass, BinarySpinPass
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation
from luna_model.transformation.pipeline import Pipeline
from luna_model.transformation.transformation import TransformationPass

model = Model(sense=Sense.MIN)
x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=2)
y = model.add_variable("y", vtype=Vtype.INTEGER, lower=0, upper=3)
model.objective = x + y


class IfElsePass(ControlFlowPass):
    _condition: Callable[[Model, PassContext], bool]
    _then: list[Pass] | Pipeline
    _otherwise: list[Pass] | Pipeline
    _name: str

    def __init__(self, condition: Callable[[Model, PassContext], bool], then: list[Pass] | Pipeline, otherwise: list[Pass] | Pipeline, name: str) -> None:
        self._condition = condition
        self._then = then
        self._otherwise = otherwise
        self._name = name

    def name(self) -> str:
        return self._name

    def run(self, model: Model, ctx: PassContext) -> ControlFlowPlan:
        if self._condition(model, ctx):
            return ControlFlowPlan(f"{self._name}_then", self._then)
        return ControlFlowPlan(f"{self._name}_else", self._otherwise)

    def requires(self) -> list[str]:
        then_requires = []
        if isinstance(self._then, Pipeline):
            then_requires = self._then.requires()
        else:
            for p in self._then:
                then_requires.extend(p.requires())

        otherwise_requires = []
        if isinstance(self._otherwise, Pipeline):
            otherwise_requires = self._otherwise.requires()
        else:
            for p in self._otherwise:
                otherwise_requires.extend(p.requires())

        out = list(set([*then_requires, *otherwise_requires]))
        return out

    def invalidates(self) -> list[str]:
        then_invalidates = []
        if isinstance(self._then, Pipeline):
            then_invalidates = self._then.invalidates()
        else:
            for p in self._then:
                if not isinstance(p, AnalysisPass | BuiltinAnalysis):
                    then_invalidates.extend(p.invalidates())

        otherwise_invalidates = []
        if isinstance(self._otherwise, Pipeline):
            otherwise_invalidates = self._otherwise.invalidates()
        else:
            for p in self._otherwise:
                if not isinstance(p, AnalysisPass | BuiltinAnalysis):
                    otherwise_invalidates.extend(p.invalidates())

        return list(set([*then_invalidates, *otherwise_invalidates]))

    def provides(self) -> list[str]:
        then_provides = []
        if isinstance(self._then, Pipeline):
            then_provides = self._then.provides()
        else:
            for p in self._then:
                if not isinstance(p, TransformationPass | BuiltinTransformation):
                    then_provides.extend(p.provides())

        otherwise_provides = []
        if isinstance(self._otherwise, Pipeline):
            otherwise_provides = self._otherwise.provides()
        else:
            for p in self._otherwise:
                if not isinstance(p, TransformationPass | BuiltinTransformation):
                    otherwise_provides.extend(p.provides())

        return list(set([*then_provides, *otherwise_provides]))


def has_binary_condition(model: Model, _: PassContext) -> bool:
    return Vtype.BINARY in model.vtypes()

conditional = IfElsePass(has_binary_condition, then=[BinarySpinPass(Vtype.SPIN)], otherwise=[], name="has_binary_condition")

pm = PassManager([IntegerToBinaryPass(), conditional])
out = pm.run(model)
print("---------")
print("OUT MODEL")
print("---------")
print(out.model)
print(out.model.variables())
print(out.model.vtypes())
