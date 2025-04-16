from dimod import BinaryQuadraticModel

from aqmodels._model import Model

class BqmTranslator:
    @staticmethod
    def to_model(bqm: BinaryQuadraticModel, name: str | None = None) -> Model: ...
    @staticmethod
    def to_bqm(model: Model) -> BinaryQuadraticModel: ...
