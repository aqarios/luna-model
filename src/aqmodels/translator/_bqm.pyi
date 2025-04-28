from dimod import BinaryQuadraticModel

from aqmodels import Model

class BqmTranslator:
    @staticmethod
    def to_aq(bqm: BinaryQuadraticModel, name: str | None = None) -> Model: ...
    @staticmethod
    def from_aq(model: Model) -> BinaryQuadraticModel: ...
