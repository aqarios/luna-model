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

"""Plugin registration for luna-model extensions.

External extensions (built with ``rs-lunamodel``, or a plain Rust/C library) can provide custom
transformation passes that the transpiler calls back into over the C ABI. Registering a plugin
makes its passes' reversal handlers known to the host and allowlists its import namespace so any
dynamically resolved artifacts/backwards can be imported.
"""

from types import ModuleType

from luna_model.transformation.decorators import register_allowed_import_prefix

__all__ = ["register"]


def register(module: ModuleType) -> None:
    """Register a luna-model plugin extension.

    Call once, after importing the plugin, before building a :class:`PassManager` with any of its
    passes::

        import luna_model
        import myplugin

        luna_model.register(myplugin)

    This allowlists the plugin's import namespace and, if the module exposes a
    ``__luna_register__()`` hook, invokes it so the plugin can install its passes' reversal
    handlers through the C ABI.
    """
    register_allowed_import_prefix(module.__name__)

    hook = getattr(module, "__luna_register__", None)
    if callable(hook):
        hook()
