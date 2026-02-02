"""LP file format translator for LunaModel.

This module provides translation between LunaModel's internal representation
and the LP (Linear Programming) file format, which is a standard text format
for representing optimization models.
"""

from pathlib import Path

from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model


class LpTranslator:
    """Translator for LP file format.

    LpTranslator provides static methods to convert between LunaModel's internal
    Model representation and the LP file format. The LP format is a widely-used
    text-based format for representing linear and quadratic optimization problems,
    supported by many solvers including CPLEX, Gurobi, and SCIP.

    The LP format supports:
    - Linear and quadratic objective functions
    - Linear and quadratic constraints
    - Integer, binary, and continuous variables
    - Variable bounds

    Examples
    --------
    Read a model from an LP file:

    >>> from luna_model.translator import LpTranslator
    >>> model = LpTranslator.to_lm("problem.lp")

    Write a model to an LP file:

    >>> from luna_model import Model, Variable, Vtype
    >>> from luna_model.translator import LpTranslator
    >>> 
    >>> model = Model(name="example")
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = 3*x + 2*y
    >>> model.constraints += x + y <= 1
    >>> 
    >>> LpTranslator.from_lm(model, "problem.lp")

    Convert to LP string without writing to file:

    >>> lp_string = LpTranslator.from_lm(model)
    >>> print(lp_string)

    Notes
    -----
    The LP format has some limitations:
    - Higher-order (degree > 2) terms are not supported
    - Some special constraint types may need transformation
    - Variable names must follow LP naming conventions

    See Also
    --------
    QuboTranslator : Translator for QUBO format
    BqmTranslator : Translator for D-Wave BQM format
    CqmTranslator : Translator for D-Wave CQM format
    """

    @staticmethod
    def to_lm(file: str | Path) -> Model:
        """Convert LP file or string to LunaModel.

        Reads an optimization model from an LP file or parses an LP-formatted
        string and converts it to a LunaModel Model object.

        Parameters
        ----------
        file : str | Path
            Either a path to an LP file or an LP-formatted string. If the string
            contains newlines or looks like LP content, it will be parsed as a
            string. Otherwise, it will be treated as a file path.

        Returns
        -------
        Model
            The parsed model in LunaModel format.

        Raises
        ------
        TranslationError
            If the LP file is malformed or contains unsupported constructs.
        FileNotFoundError
            If the specified file path does not exist.

        Examples
        --------
        From a file:

        >>> model = LpTranslator.to_lm("model.lp")

        From a string:

        >>> lp_str = '''
        ... Minimize
        ...   obj: 3 x + 2 y
        ... Subject To
        ...   c1: x + y <= 1
        ... Bounds
        ...   0 <= x <= 1
        ...   0 <= y <= 1
        ... Binary
        ...   x y
        ... End
        ... '''
        >>> model = LpTranslator.to_lm(lp_str)
        """
        return Model._from_pym(PyLpTranslator.to_lm(file))

    @staticmethod
    def from_lm(model: Model, filepath: Path | None = None) -> str | None:
        """Convert LunaModel to LP file or string.

        Converts a LunaModel Model to LP format, either writing to a file or
        returning the LP-formatted string.

        Parameters
        ----------
        model : Model
            The LunaModel to convert.
        filepath : Path | None, optional
            If provided, writes the LP representation to this file path.
            If ``None``, returns the LP string without writing to a file.

        Returns
        -------
        str | None
            If ``filepath`` is ``None``, returns the LP-formatted string.
            If ``filepath`` is provided, writes to file and returns ``None``.

        Raises
        ------
        TranslationError
            If the model contains constructs that cannot be represented in
            LP format (e.g., higher-order terms).

        Examples
        --------
        Write to file:

        >>> from luna_model import Model
        >>> model = Model(name="example")
        >>> # ... build model ...
        >>> LpTranslator.from_lm(model, "output.lp")

        Get LP string:

        >>> lp_string = LpTranslator.from_lm(model)
        >>> print(lp_string)

        Notes
        -----
        Variable and constraint names are automatically sanitized to comply
        with LP format naming rules. The original names are preserved in the
        Model object and can be used when working with solutions.
        """
        return PyLpTranslator.from_lm(model._m, filepath)
