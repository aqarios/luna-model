from aqmodels._api_utils import export


@export("solution_translator", "top")
class MatrixTranslator:
    @staticmethod
    def from_dimod_sample_set(sample_set, timing=None, env=None):
        return sample_set, timing, env
