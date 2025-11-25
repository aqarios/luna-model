use pyo3::{create_exception, exceptions::PyException};

create_exception!(
    aqmodels._core.errors,
    MultipleActiveEnvironmentsError,
    PyException,
    "Raised when multiple environments are active simultaneously.

This is a logic error, since LunaModel only supports one active environment
at a time. This is enforced to maintain clarity and safety."
);

create_exception!(
    aqmodels._core.errors,
    NoActiveEnvironmentFoundError,
    PyException,
    "Raised when a variable or expression is created without an active environment context.

This typically happens when not using `with Environment(): ...` and no environment
was explicitly provided."
);
