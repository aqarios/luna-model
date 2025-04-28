from dimod import ConstrainedQuadraticModel

from aqmodels import Model

class CqmTranslator:
    @staticmethod
    def to_model(cqm: ConstrainedQuadraticModel) -> Model: ...
    @staticmethod
    def from_model(model: Model) -> ConstrainedQuadraticModel: ...
