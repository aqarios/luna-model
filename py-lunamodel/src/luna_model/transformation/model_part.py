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

from __future__ import annotations

from enum import Enum

from luna_model._lm import PyModelPart


class ModelPart(Enum):
    """Parts of a model that passes modify and analyses depend on.

    A transformation declares the parts it may modify and an analysis the parts its result
    is computed from. An analysis result is invalidated by a transformation exactly when the
    two overlap.

    Attributes
    ----------
    OBJECTIVE : str
        The objective expression.
    CONSTRAINTS : str
        The constraint collection.
    VARIABLES : str
        The environment: variables, their types and their bounds.
    SENSE : str
        The optimization sense.

    Examples
    --------
    >>> from luna_model import ModelPart
    >>> from luna_model.transformation import transform
    >>> @transform(modifies={ModelPart.OBJECTIVE})
    ... def scale_obj(model, ctx): ...
    """

    OBJECTIVE = "Objective"
    CONSTRAINTS = "Constraints"
    VARIABLES = "Variables"
    SENSE = "Sense"

    @property
    def _val(self) -> PyModelPart:
        match self:
            case ModelPart.OBJECTIVE:
                return PyModelPart.Objective
            case ModelPart.CONSTRAINTS:
                return PyModelPart.Constraints
            case ModelPart.VARIABLES:
                return PyModelPart.Variables
            case ModelPart.SENSE:
                return PyModelPart.Sense
