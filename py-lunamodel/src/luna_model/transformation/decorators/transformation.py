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

import inspect
from collections.abc import Callable
from dataclasses import dataclass
from importlib import import_module
from typing import Generic, TypeAlias, TypeVar, cast

from typing_extensions import override

from luna_model.model.model import Model
from luna_model.solution.sol import Solution
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.context import PassContext
from luna_model.transformation.transformation import TransformationPass

A = TypeVar("A", bound=TransformationPassArtifact)

TransformationSignature: TypeAlias = Callable[[Model, PassContext], tuple[Model, A]]
BackwardSignature: TypeAlias = Callable[[A, Solution], Solution]


_ALLOWED_IMPORT_PREFIXES: set[str] = {"luna_model."}


def register_allowed_import_prefix(prefix: str) -> None:
    """Allow dynamic import resolution for modules under a given prefix.

    Parameters
    ----------
    prefix : str
        Allowed module prefix, e.g. ``"my_package.transforms."``.
    """
    if not prefix:
        msg = "prefix must not be empty"
        raise ValueError(msg)
    normalized = prefix if prefix.endswith(".") else f"{prefix}."
    _ALLOWED_IMPORT_PREFIXES.add(normalized)


def allowed_import_prefixes() -> tuple[str, ...]:
    """Get currently allowed import prefixes used by artifact resolution."""
    return tuple(sorted(_ALLOWED_IMPORT_PREFIXES))


def _is_allowed_module_path(module_path: str) -> bool:
    return any(module_path.startswith(p) for p in _ALLOWED_IMPORT_PREFIXES)


def __identity_backward(_: TransformationPassArtifact, solution: Solution) -> Solution:
    return solution


_MAGIC: bytes = b"AEPY"
_VERSION: int = 1
_HEADER_LEN: int = 5


def _write_field(out: bytearray, data: bytes) -> None:
    out.extend(len(data).to_bytes(4, "big"))
    out.extend(data)


def _read_u32(buf: bytes, i: int) -> tuple[int, int]:
    if i + 4 > len(buf):
        msg = "Truncated envelope"
        raise ValueError(msg)
    return int.from_bytes(buf[i : i + 4], "big"), i + 4


def _read_field(buf: bytes, i: int) -> tuple[bytes, int]:
    n, i = _read_u32(buf, i)
    j = i + n
    if j > len(buf):
        msg = "Truncated envelope"
        raise ValueError(msg)
    return buf[i:j], j


def _resolve(module_path: str, qualname: str) -> object:
    if not _is_allowed_module_path(module_path):
        msg = f"Disallowed module path '{module_path}'. Allowed prefixes: {', '.join(sorted(_ALLOWED_IMPORT_PREFIXES))}"
        raise ValueError(msg)

    if module_path.startswith(".") or ".." in module_path:
        msg = f"Invalid module path '{module_path}'"
        raise ValueError(msg)

    if "<locals>" in qualname:
        msg = f"Cannot resolve local symbol '{qualname}'"
        raise ValueError(msg)

    # trusted scope constrained by allowlist above
    obj = import_module(module_path)  # nosem
    for part in qualname.split("."):
        obj = getattr(obj, part)
    return obj


@dataclass(frozen=True)
class _ArtifactEnvelope(TransformationPassArtifact, Generic[A]):
    artifact_module: str
    artifact_qualname: str
    backward_module: str
    backward_qualname: str
    artifact_payload: bytes

    @classmethod
    def from_parts(cls, artifact: A, backward: BackwardSignature[A]) -> _ArtifactEnvelope[A]:
        acls = artifact.__class__
        bmod = backward.__module__
        bqual = backward.__qualname__
        if backward.__name__ == "<lambda>" or "<locals>" in bqual:
            msg = "backward must be a module-level named function"
            raise TypeError(msg)
        return cls(
            artifact_module=acls.__module__,
            artifact_qualname=acls.__qualname__,
            backward_module=bmod,
            backward_qualname=bqual,
            artifact_payload=artifact.serialize(),
        )

    def serialize(self) -> bytes:
        out = bytearray(_MAGIC)
        out.append(_VERSION)
        _write_field(out, self.artifact_module.encode())
        _write_field(out, self.artifact_qualname.encode())
        _write_field(out, self.backward_module.encode())
        _write_field(out, self.backward_qualname.encode())
        _write_field(out, self.artifact_payload)
        return bytes(out)

    @classmethod
    def deserialize(cls, buf: bytes) -> _ArtifactEnvelope:
        if len(buf) < _HEADER_LEN or buf[:4] != _MAGIC or buf[4] != _VERSION:
            msg = "Invalid envelope header"
            raise ValueError(msg)
        i = 5
        am, i = _read_field(buf, i)
        aq, i = _read_field(buf, i)
        bm, i = _read_field(buf, i)
        bq, i = _read_field(buf, i)
        payload, i = _read_field(buf, i)
        if i != len(buf):
            msg = "Trailing bytes"
            raise ValueError(msg)
        return cls(am.decode(), aq.decode(), bm.decode(), bq.decode(), payload)

    @property
    def artifact(self) -> A:
        artifact_cls: type[A] = cast("type[A]", _resolve(self.artifact_module, self.artifact_qualname))
        return artifact_cls.deserialize(self.artifact_payload)

    @property
    def backward_fn(self) -> BackwardSignature[A]:
        return cast("BackwardSignature[A]", _resolve(self.backward_module, self.backward_qualname))


class _DynamicTransformationPass(TransformationPass, Generic[A]):
    _name: str
    _requires: list[str]
    _invalidates: list[str]
    _forward_f: TransformationSignature
    _backward_f: BackwardSignature

    def __init__(
        self,
        name: str,
        requires: list[str],
        invalidates: list[str],
        forward: TransformationSignature,
        backward: BackwardSignature,
    ) -> None:
        super().__init__()
        self._name = name
        self._requires = requires
        self._invalidates = invalidates
        self._forward_f = forward
        self._backward_f = backward

    @override
    def name(self) -> str:
        return self._name

    @override
    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, _ArtifactEnvelope[A]]:
        result: tuple[Model, A] = self._forward_f(model, ctx)
        model, artifact = result
        return model, _ArtifactEnvelope.from_parts(artifact, self._backward_f)

    @override
    @classmethod
    def backward(cls, artifact: _ArtifactEnvelope[A], solution: Solution) -> Solution:
        return artifact.backward_fn(artifact.artifact, solution)

    @override
    def requires(self) -> list[str]:
        return self._requires

    @override
    def invalidates(self) -> list[str]:
        return self._invalidates


def _validate_backward(fn: object) -> None:
    if not inspect.isfunction(fn):
        msg = "backward must be a function"
        raise TypeError(msg)
    if fn.__name__ == "<lambda>" or "<locals>" in fn.__qualname__:
        msg = "backward must be a module-level named function. lambdas or local functions are not allowed."
        raise TypeError(msg)


def transform(
    name: str | None = None,
    requires: list[str] | None = None,
    invalidates: list[str] | None = None,
    backward: BackwardSignature[A] | None = None,
) -> Callable[[TransformationSignature[A]], _DynamicTransformationPass[A]]:
    """Create a TransformationPass from a function decorator.

    This decorator converts a regular function into a ``TransformationPass`` that modifies
    models in transformation pipelines. Transformation passes can restructure models,
    add/remove constraints, change variable types, or perform other model modifications.

    !!! warning "Disclaimer"
        Dynamic artifact/backward resolution is restricted to an import allowlist.
        By default, only modules under ``luna_model.`` are allowed.
        Use ``register_allowed_import_prefix(...)`` to allow custom plugin namespaces.

    Parameters
    ----------
    name : str, optional
        The name of the transformation pass. If not provided, uses the function name
        with underscores replaced by hyphens.
    requires : list[str], optional
        List of pass names that must run before this transformation. Defaults to ``[]``.
    invalidates : list[str], optional
        List of analysis pass names whose results become invalid after this transformation. Defaults to ``[]``.
    backwards : Callable[[A, Solution], Solution], optional
        Optional function to map solutions from the transformed model back to the
        original model's variable space. If not provided, solutions pass through unchanged.

    Returns
    -------
    Callable[[TransformationSignature], _DynamicTransformationPass[A]]
        A decorator that transforms the decorated function into a ``TransformationPass``.
        The generic ``A`` is the artifact produced by this transformation pass.

    Examples
    --------
    Create a simple transformation:

    >>> from luna_model.transformation import transform
    >>> @transform(name="scale-objective")
    ... def scale_obj(model: Model, ctx: PassContext):
    ...     model.objective = model.objective * 2.0
    ...     return model, NothingArtifact()

    Notes
    -----
    Dynamic artifact/backward resolution is restricted to an import allowlist.
    By default, only modules under ``luna_model.`` are allowed.
    Use ``register_allowed_import_prefix(...)`` to allow custom plugin namespaces.

    The decorated function must return:

    - ``(Model, Artifact)`` tuple

    The backwards function is crucial when transformations change the variable space
    (e.g., adding/removing variables, changing variable types). It ensures solutions
    from downstream solvers can be correctly interpreted in the original model's context.
    """
    if requires is None:
        requires = []
    if invalidates is None:
        invalidates = []

    if backward is None:
        backward = __identity_backward
    else:
        _validate_backward(backward)

    def _decorator(forward: TransformationSignature) -> _DynamicTransformationPass:
        loc_name = name or forward.__name__.replace("_", "-")
        return _DynamicTransformationPass(
            name=loc_name,
            requires=requires,
            invalidates=invalidates,
            forward=forward,
            backward=backward,
        )

    return _decorator
