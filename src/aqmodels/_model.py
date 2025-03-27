from aqmodels._api_utils import export, dispatched


@export
class Model:
    """ """

    @dispatched
    def __init__(self, env, name):
        """ """
        return env, name

    @property
    @dispatched
    def name(self):
        """ """
        return

    @property
    @dispatched
    def objective(self):
        """ """
        return

    @objective.setter
    @dispatched
    def objective(self, value):
        """ """
        return value

    @property
    @dispatched
    def constraints(self):
        """ """
        return

    @constraints.setter
    @dispatched
    def constraints(self, value):
        """ """
        return value

    @property
    @dispatched
    def environment(self):
        """ """
        return

    @dispatched
    def num_constraints(self):
        """ """
        return

    @dispatched
    def encode(self, compress, level):
        """ """
        return compress, level

    @dispatched
    def serialize(self, compress, level):
        """ """
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
        """ """
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """ """
        return data
