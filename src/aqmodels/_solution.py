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
    @dispatched
    def __getitem__(self, item):
        return item

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

    @dispatched
    def __getitem__(self, item):
        return item

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

# def wrap_from_dimod_sample_set(f):
#     @functools.wraps(SampleSetTranslator.from_dimod_sample_set)
#     def inner(sample_set: SampleSet, timing: Timing | None = None) -> Solution:
#         sample_set = sample_set.aggregate()
#         record = sample_set.record
#         sample = record.sample.astype(np.int64, order="C")
#         num_occurrences = record.num_occurrences.astype(np.int64, order="C")
#
#         return f(sample, num_occurrences, timing)
#
#     return inner
#
#
# SampleSetTranslator.from_dimod_sample_set = wrap_from_dimod_sample_set(
#     SampleSetTranslator.from_dimod_sample_set
# )
