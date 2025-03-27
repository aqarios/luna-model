from __future__ import annotations
from aqmodels._api_utils import export, dispatched


@export
class Expression:
    @dispatched
    def __init__(self, env):
        """ """
        return env

    @dispatched
    def get_offset(self):
        """ """
        return

    @dispatched
    def get_linear(self, variable):
        """ """
        return variable

    @dispatched
    def get_higher_order(self, variables):
        """ """
        return variables

    @dispatched
    def get_quadratic(self, u, v):
        """ """
        return u, v

    @dispatched
    def num_variables(self):
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
    def deserialize(data):
        """ """
        return data

    @dispatched
    @staticmethod
    def decode(data):
        """ """
        return data
