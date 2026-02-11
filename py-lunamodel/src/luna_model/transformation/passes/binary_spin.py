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
from typing import Literal

from luna_model._lm import PyBinarySpinPass
from luna_model.model.model import Vtype
from luna_model.transformation.transform import ConcreteTransformationPass


class BinarySpinPass(ConcreteTransformationPass):
    """A transformation pass changing the binary/spin variables to spin/binary."""

    def __init__(self, vtype: Literal[Vtype.BINARY, Vtype.SPIN], prefix: str | None) -> None:
        super().__init__(base=PyBinarySpinPass(vtype._val, prefix))

    @property
    def vtype(self) -> Vtype:
        """Get the target vtype."""
        return Vtype._from_pyvtype(self._base.vtype)

    @property
    def prefix(self) -> str | None:
        """Get the naming prefix."""
        return self._base.prefix
