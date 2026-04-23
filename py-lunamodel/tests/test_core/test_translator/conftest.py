from __future__ import annotations

from pathlib import Path
from zipfile import ZipFile


MPS_DIR = Path(__file__).parent / "mps_models"
MPS_ARCHIVE = MPS_DIR / "mps_models.zip"


def pytest_configure() -> None:
    with ZipFile(MPS_ARCHIVE) as archive:
        for member in archive.infolist():
            if member.is_dir():
                continue
            target = MPS_DIR / member.filename
            if target.parent != MPS_DIR:
                raise RuntimeError(f"unexpected path in MPS fixture archive: {member.filename}")
            if target.exists():
                continue
            archive.extract(member, MPS_DIR)
