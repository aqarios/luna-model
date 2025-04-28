from dimod import ConstrainedQuadraticModel

from aqmodels import Model

class CqmTranslator:
    @staticmethod
    def to_aq(cqm: ConstrainedQuadraticModel) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> ConstrainedQuadraticModel: ...
