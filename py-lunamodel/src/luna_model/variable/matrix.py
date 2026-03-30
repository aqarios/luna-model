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

from typing import Any, Literal, Self

import numpy as np


class NDLmArray(np.ndarray):
    """A n-dimensional array of variables."""

    def __new__(cls, input_array: np.ndarray) -> Self:
        """Create a new NDLmArray based on a ndarray."""
        return np.asarray(input_array).view(cls)

    def __array_ufunc__(
        self,
        ufunc: np.ufunc,
        method: Literal["__call__", "reduce", "reduceat", "accumulate", "outer", "at"],
        *inputs: Any,
        **kwargs: Any,
    ) -> Any:  # noqa: ANN401
        """Call ufunc."""
        if method == "__call__":
            match ufunc:
                case np.less_equal:
                    return NDLmArray.__vectorized_le(*inputs, **kwargs)
                case np.greater_equal:
                    return NDLmArray.__vectorized_ge(*inputs, **kwargs)
                case np.equal:
                    return NDLmArray.__vectorized_eq(*inputs, **kwargs)
                case np.less | np.greater:
                    return NotImplemented
        raw_inputs = tuple(x.view(np.ndarray) if isinstance(x, NDLmArray) else x for x in inputs)
        return NDLmArray(super().__array_ufunc__(ufunc, method, *raw_inputs, **kwargs))

    def __vectorized_le(self, other: float | np.ndarray | Self) -> Any:  # noqa: ANN401
        if isinstance(other, (type(self), np.ndarray)):
            if self.shape != other.shape:
                msg = "shape mismatch for elementwise comparison"
                raise ValueError(msg)
            return NDLmArray(
                np.fromiter(
                    (x <= y for x, y in zip(self.flat, other.flat, strict=True)),
                    dtype=object,
                    count=self.size,
                ).reshape(self.shape)
            )

        return NDLmArray(np.vectorize(lambda o: o <= other)(self))

    def __vectorized_ge(self, other: float | np.ndarray | Self) -> Any:  # noqa: ANN401
        if isinstance(other, (type(self), np.ndarray)):
            if self.shape != other.shape:
                msg = "shape mismatch for elementwise comparison"
                raise ValueError(msg)
            return NDLmArray(
                np.fromiter(
                    (x >= y for x, y in zip(self.flat, other.flat, strict=True)),
                    dtype=object,
                    count=self.size,
                ).reshape(self.shape)
            )

        return NDLmArray(np.vectorize(lambda o: o >= other)(self))

    def __vectorized_eq(self, other: float | np.ndarray | Self) -> Any:  # noqa: ANN401
        if isinstance(other, (type(self), np.ndarray)):
            if self.shape != other.shape:
                msg = "shape mismatch for elementwise comparison"
                raise ValueError(msg)
            return NDLmArray(
                np.fromiter(
                    (x == y for x, y in zip(self.flat, other.flat, strict=True)),
                    dtype=object,
                    count=self.size,
                ).reshape(self.shape)
            )

        return NDLmArray(np.vectorize(lambda o: o == other)(self))
