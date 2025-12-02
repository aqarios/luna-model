# PyO3 LunaModel

PyO3 based Python API for LunaModel. This is required to write extensions for the main LunaModel python library.
We cannot just use the lunamodel-python crate, as the types **are different** even if the same lunamodel-python code is used, since
Rust **does not** have a stable ABI and **does not** allow for dyamic linking to other libraries we have have to go via the Python route and it's
**C API** which is stable. So we need to call **into** the rust code from our extension via it's **Python API** we do this in each **IntoPyObject** and **FromPyObject** implementation. In the **IntoPyObject** impls we create a pyo3-lunamodel local representation of the lunamodel-python representation.

These two types are by default **not compatible**. We can act like it is in our custom API. Each pyo3-lunamodel type is automatically transformed to the python LunaModel type based on functions that are guaranteed to exist in the main LunaModel python library.

Since we do not want to clone our data and everything must be thread-safe we are required to write some `unsafe` code here and their to transform our pointers. However, this `unsafe` code actually is safe due to our checks before and after and the use of native types to move our pointers between the extension and the main lib and vice versa. Such a python/pyo3 native type that we are relying on heavily for the dynamic extendability is the `PyCapsule` (a python `Capsule`) which is designed explicitly for this kind of usecase.
