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

import sys

if sys.version_info < (3, 13):
    from typing_extensions import deprecated
else:
    from warnings import deprecated

from luna_model.transformation.output import TransformationOutput


@deprecated(
    "This method is deprecated and will be removed in the next non-patch release. "
    "Use the `backward` method of the `TransformationRecord` produced as part of the `TransformationOutcome` by the "
    "PassManager execution instead."
)
class IR(TransformationOutput):
    """The legacy name used for the TransformationOutcome in the previous versions."""
