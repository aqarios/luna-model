from __future__ import annotations
from aqmodels._api_utils import export, dispatched


@export
class Environment:
    """
    Documentation of Environment
    """

    @dispatched
    def __init__(self):
        """
        Documentation of init
        """
        return

    @dispatched
    def encode(self, compress, level):
        """
        Documentation of encode
        """
        return compress, level

    @dispatched
    def serialize(self, compress, level):
        """
        Documentation of serialize
        """
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
        """
        Documentation of add_constraint
        """
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """
        Documentation of deserialize
        """
        return data
