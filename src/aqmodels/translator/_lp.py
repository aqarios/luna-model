from aqmodels._api_utils import dispatched, export


@export("translator", "top")
class LpTranslator:
    """
    Utility class for converting between LP files and symbolic models.

    `LpTranslator` provides methods to:
    - Convert a LP file into a symbolic `Model`
    - Convert a `Model` into a Lp file.

    These conversions are especially useful when interacting with external solvers
    or libraries that operate on LP-based problem definitions.

    Examples
    --------
    >>> from pathlib import Path
    >>> from aqmodels import LpTranslator
    >>> lp_filepath = Path("path/to/the/lp_file")

    >>> model = LpTranslator.to_model(lp_filepath)

    Convert it back to a LP file:

    >>> recovered = LpTranslator.to_file(model)
    """

    @dispatched
    @staticmethod
    def to_model(file):
        """
        Convert a LP file into a symbolic `Model`.

        Parameters
        ----------
        file: Path | String
            A LP file representing a symbolic model, either given as a
            path to the LP file or it's contents as a string.

        Returns
        -------
        Model
            A symbolic model representing the given lp file structure.
        """
        return file

    @dispatched
    @staticmethod
    def from_model(model, file):
        """
        Convert a symbolic model to a lp file representation.

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
        """
        return model, file
