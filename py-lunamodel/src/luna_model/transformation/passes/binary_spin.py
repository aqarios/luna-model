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
    """Convert between Binary and Spin variable types.

    Transform the variables of type binary to spin typed variables or vice versa.
    Target vtype is set by the ``vtype`` parameter.

    Parameters
    ----------
    vtype : Literal[Vtype.BINARY, Vtype.SPIN]
        The target vtype to convert the variables to. If set to ``Vtype.SPIN`` all
        binary variables in the model are converted to spin variables. If set to
        ``Vtype.BINARY`` all spin variables are converted to binary variables.
    prefix : str, optional
        An optional prefix to prepend to the name of the new variables created.
        If none specified a default prefix is used.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import BinarySpinPass
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> pm = PassManager([BinarySpinPass(vtype=Vtype.SPIN)])
    >>> ir = pm.run(model)
    >>> spin_model = ir.model
    """

    def __init__(self, vtype: Literal[Vtype.BINARY, Vtype.SPIN], prefix: str | None = None) -> None:
        super().__init__(base=PyBinarySpinPass(vtype._val, prefix))

    @property
    def vtype(self) -> Vtype:
        """Get the target variable type to convert to."""
        return Vtype._from_pyvtype(self._base.vtype)

    @property
    def prefix(self) -> str | None:
        """Get the optional naming prefix."""
        return self._base.prefix
