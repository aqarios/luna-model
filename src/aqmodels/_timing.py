from aqmodels._api_utils import export, dispatched


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
    def start():
        return

    @dispatched
    def stop(self):
        return
