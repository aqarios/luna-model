from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class MatrixTranslator:
    """
    This is the MatrixTranslator documentation.
    """

    @dispatched
    @staticmethod
    def to_model(qubo, name, vtype):
        """ """
        return qubo, name, vtype

    @dispatched
    @staticmethod
    def to_dense(model):
        """ """
        return model
