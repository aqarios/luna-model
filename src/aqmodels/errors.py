"""
Define error types for validation, failures, and runtime evaluation issues.

This module defines the set of custom exception classes used throughout the system
to handle errors related to model translation, environment management, solution
evaluation, and sampling.

The exceptions are categorized as follows:

1. **Model Errors**:
   - Raised when a model does not meet certain structural or semantic requirements,
     such as being quadratic, unconstrained, or having the correct variable types.

2. **Translation Errors**:
   - Raised when issues occur during translation between formats, including both
     model and solution translation failures.

3. **Variable and Constraint Errors**:
   - Raised when there are issues related to variable creation, lookup, duplication,
     or constraint naming.

4. **Environment Errors**:
   - Raised when conflicts or inconsistencies occur in managing active environments,
     such as having multiple active environments or none at all.

5. **Sampling and Evaluation Errors**:
   - Raised during post-processing or evaluation of samples, particularly when the
     sample does not conform to expected formats or contains incompatible data.

These error classes enable precise and meaningful error handling, helping developers
diagnose and respond to failures consistently and effectively.
"""

from ._core.errors import *  # type: ignore[reportMissingImports] # noqa: F403
