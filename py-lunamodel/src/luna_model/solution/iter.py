from __future__ import annotations

from luna_model.solution.res import Result


class ResultView(Result): ...


class ResultIter:
    def __next__(self) -> ResultView: ...
    def __iter__(self) -> ResultIter: ...
