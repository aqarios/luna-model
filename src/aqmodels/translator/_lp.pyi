from pathlib import Path
from typing import overload

from aqmodels import Model

class LpTranslator:
    """
    Utility class for converting between LP files and symbolic models.

    `LpTranslator` provides methods to:
    - Convert an LP file into a symbolic `Model`
    - Convert a `Model` into an Lp file.

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on LP-based problem definitions.

    Examples
    --------
    >>> from pathlib import Path
    >>> from luna_quantum import LpTranslator
    >>> lp_filepath = Path("path/to/the/lp_file")

    >>> model = LpTranslator.to_aq(lp_filepath)

    Convert it back to an LP file:

    >>> recovered = LpTranslator.to_file(model)
    """

    @overload
    @staticmethod
    def to_aq(file: Path) -> Model: ...
    @overload
    @staticmethod
    def to_aq(file: str) -> Model:
        """
        Convert an LP file into a symbolic `Model`.

        Parameters
        ----------
        file: Path | String
            An LP file representing a symbolic model, either given as a
            Path object to the LP file or its contents as a string.
            If you pass the path as a string, it will be interpreted as a
            model and thus fail to be parsed to a Model.

        Returns
        -------
        Model
            A symbolic model representing the given lp file structure.

        Raises
        ------
        TypeError
            If `file` is not of type `str` or `Path`.
        TranslationError
            If the translation fails for a different reason.
        """
        ...

    @overload
    @staticmethod
    def from_aq(model: Model) -> str: ...
    @overload
    @staticmethod
    def from_aq(model: Model, *, filepath: Path) -> None:
        """
        Convert a symbolic model to an LP file representation.

        Parameters
        ----------
        model : Model
            The symbolic model to convert.
        file : Path, optional
            The filepath to write the model contents to.

        Returns
        -------
        str
            If no file to write to is given, i.e., the file is None.

        Raises
        ------
        TranslationError
            If the translation fails for some reason.
        """
        ...
