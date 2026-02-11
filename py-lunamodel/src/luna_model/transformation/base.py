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
from typing import Protocol


class BasePass(Protocol):
    """Protocol for passes."""

    @property
    def name(self) -> str:
        """
        Unique identifier for this pass.

        Returns
        -------
        str
            The unique pass name.
        """
        ...

    @property
    def requires(self) -> list[str]:
        """
        List of passes that must run before this pass.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        ...
