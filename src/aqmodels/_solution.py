from aqmodels._api_utils import export, dispatched


@export
class Sample:
    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __next__(self):
        return


@export
class ResultView:
    @property
    @dispatched
    def sample(self):
        return

    @property
    @dispatched
    def num_occurrences(self):
        return

    @property
    @dispatched
    def obj_value(self):
        return

    @property
    @dispatched
    def constraints(self):
        return

    @property
    @dispatched
    def feasible(self):
        return


@export
class Results:
    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __next__(self):
        return


@export
class Solution:
    @dispatched
    def __str__(self):
        return

    @dispatched
    def __repr__(self):
        return

    @dispatched
    def __iter__(self):
        return

    @property
    @dispatched
    def results(self):
        return

    @property
    @dispatched
    def samples(self):
        return

    @property
    @dispatched
    def obj_values(self):
        return

    @property
    @dispatched
    def num_occurrences(self):
        return

    @property
    @dispatched
    def runtime(self):
        return

    @property
    @dispatched
    def best_sample_idx(self): return


@export
class Timing:
    @property
    @dispatched
    def start(self):
        return

    @property
    @dispatched
    def end(self):
        return

    @property
    @dispatched
    def total(self):
        return

    @property
    @dispatched
    def total_seconds(self):
        return

    @property
    @dispatched
    def qpu(self):
        return

    @qpu.setter
    @dispatched
    def qpu(self, value):
        return value

    @dispatched
    def add_qpu(self, value: float):
        return


@export
class Timer:
    @dispatched
    @staticmethod
    def start(cls):
        return

    @dispatched
    def stop(self):
        return
