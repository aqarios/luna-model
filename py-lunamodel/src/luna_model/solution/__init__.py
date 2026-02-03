"""Solution objects for optimization results.

This module provides classes and protocols for representing solution results
from optimization solvers, including samples, results, and result views.

Key components:
    - Solution: Main solution class containing results and samples
    - Result/ResultView: Individual result with sample and metadata
    - Sample/Samples: Variable assignments in solutions
    - ValueSource: Source of solution values (objective or raw)
"""

from luna_model.solution.res import Result, ResultIter, ResultView
from luna_model.solution.sample import Sample
from luna_model.solution.sol import Solution
from luna_model.solution.src import ValueSource

__all__ = [
    "Result",
    "ResultIter",
    "ResultView",
    "Sample",
    "Solution",
    "ValueSource",
]
