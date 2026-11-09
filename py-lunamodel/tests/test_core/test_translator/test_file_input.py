"""`to_lm` of the file translators: a single-line string or a Path names a file."""

from pathlib import Path

import pytest

from luna_model.translator import LpTranslator, MpsTranslator


@pytest.mark.parametrize("translator", [LpTranslator, MpsTranslator])
@pytest.mark.parametrize("as_input", [str, Path])
def test_missing_file_raises_file_not_found(translator, as_input, tmp_path: Path):
    with pytest.raises(FileNotFoundError, match="missing"):
        translator.to_lm(as_input(tmp_path / "missing"))
