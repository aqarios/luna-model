# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from pathlib import Path
from typing import overload

from luna_model._lm import PyMpsTranslator
from luna_model.model.model import Model


class MpsTranslator:
    r"""Translator for MPS file format.

    MpsTranslator provides static methods to convert between LunaModel's internal
    Model representation and the MPS file format. The MPS format is a widely-used
    text-based format for representing linear and quadratic optimization problems.

    The MPS format supports:
    - Linear and quadratic objective functions
    - Linear and quadratic constraints
    - Integer, binary, and continuous variables
    - Variable bounds

    Examples
    --------
    Read a model from an MPS file:

    >>> from luna_model.translator import MpsTranslator
    >>> model = MpsTranslator.to_lm("problem.mps")  # doctest: +SKIP

    Write a model to an MPS file:

    >>> from luna_model import Model, Variable, Vtype
    >>> from luna_model.translator import MpsTranslator
    >>> model = Model(name="example")
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = 3 * x + 2 * y
    >>> model.constraints += x + y <= 1
    >>> MpsTranslator.from_lm(model, "problem.mps")

    Convert to MPS string without writing to file:

    >>> mps_string = MpsTranslator.from_lm(model)
    >>> print(mps_string)
    NAME          example
    OBJSENSE
     MIN
    ROWS
     N  OBJ
     L  c0
    COLUMNS
        x  OBJ  3.0
        x  c0  1.0
        y  OBJ  2.0
        y  c0  1.0
    RHS
        RHS1      c0  1.0
    BOUNDS
     BV BND1      x
     BV BND1      y
    ENDATA
    """

    @staticmethod
    def to_lm(file: str | Path) -> Model:
        """Convert MPS file or string to LunaModel.

        Reads an optimization model from an MPS file or parses an MPS-formatted
        string and converts it to a LunaModel Model object.

        Parameters
        ----------
        file : str or Path
            Either a path to an MPS file or an MPS-formatted string. If the string
            contains newlines or looks like MPS content, it will be parsed as a
            string. Otherwise, it will be treated as a file path.

        Returns
        -------
        Model
            The parsed model in LunaModel format.

        Raises
        ------
        TranslationError
            If the MPS file is malformed or contains unsupported constructs.
        FileNotFoundError
            If the specified file path does not exist.

        Examples
        --------
        From a file:

        >>> model = MpsTranslator.to_lm("model.mps")  # doctest: +SKIP

        From a string:

        >>> mps_str = '''
        ... NAME          example
        ... OBJSENSE
        ...  MAX
        ... ROWS
        ...  N  OBJ
        ...  L  c0
        ... COLUMNS
        ...     x         OBJ  3.0  c0  1.0
        ...     y         OBJ  2.0  c0  1.0
        ... RHS
        ...     RHS1      c0  1.0
        ... QUADOBJ
        ...     x  y  4.4
        ... QCMATRIX  c0
        ...     x  y  2.2
        ... BOUNDS
        ...  BV BND1      x
        ...  BV BND1      y
        ... ENDATA
        ... '''
        >>> model = MpsTranslator.to_lm(mps_str)
        >>> print(model)
        Model: example
        Maximize
          4.4 * x * y + 3 * x + 2 * y
        Subject To
          c0: 2.2 * x * y + x + y <= 1
        Binary
          x y
        """
        return Model._from_pym(PyMpsTranslator.to_lm(file))

    @staticmethod
    @overload
    def from_lm(model: Model) -> str: ...
    @staticmethod
    @overload
    def from_lm(model: Model, filepath: Path) -> None: ...
    @staticmethod
    def from_lm(model: Model, filepath: Path | None = None) -> str | None:
        r"""Convert LunaModel to MPS file or string.

        Converts a LunaModel Model to MPS format, either writing to a file or
        returning the MPS-formatted string.

        Parameters
        ----------
        model : Model
            The LunaModel to convert.
        filepath : Path, optional
            If provided, writes the MPS representation to this file path.
            If ``None``, returns the MPS string without writing to a file.

        Returns
        -------
        str or None
            If ``filepath`` is ``None``, returns the MPS-formatted string.
            If ``filepath`` is provided, writes to file and returns ``None``.

        Raises
        ------
        TranslationError
            If the model contains constructs that cannot be represented in
            MPS format (e.g., higher-order terms).

        Examples
        --------
        Write to file:

        >>> from luna_model import Model
        >>> model = Model(name="example")
        >>> # ... build model ...
        >>> MpsTranslator.from_lm(model, "output.mps")

        Get MPS string:

        >>> mps_string = MpsTranslator.from_lm(model)
        >>> print(mps_string)
        NAME          example
        OBJSENSE
         MIN
        ROWS
         N  OBJ
        COLUMNS
        RHS
        BOUNDS
        ENDATA
        """
        return PyMpsTranslator.from_lm(model._m, filepath)
