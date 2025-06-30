"""Transformations collection.

The `transformations` module provides a collection of transformations for converting
between various representations of optimization problems and their solutions.

Transformations generally convert between different representations of an optimization
model. For example, changing the Sense of a model.

Each transformation encapsulates the logic needed for transforming a model to a desired
output model with changed properties and the logic to convert a solution of the output
model back to a solution representation matching the input model.

In addition to the predefined transformations contained in this module.
One can implement their own transformations by implementing the `TransformationPass`
and `AnalysisPass` abstract classes. See the examples for further details.
"""

from ._core.transformations import *  # type: ignore[reportMissingImports] # noqa: F403
