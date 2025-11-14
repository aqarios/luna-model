//! # LunaModel: Symbolic modeling for optimization
//!
//! LunaModel is a high-performance symbolic modeling library for describing, translating and 
//! transforming optimization problems. It provides the following high-level features:
//!     - System for defining symbolic algebraic expressions of arbitrary degree, constraints and 
//!       optimization models (like dimod, gurobi or cplex)
//!     - Translations from and to an LunaModel for many common optimization model formats (like LP)
//!     - Transformations to map an LunaModel from a general model to a specific model, such as transforming a 
//!       Constrained (Binary) Quadratic Model (CQM) to a (Unconstrained) Binary Quadratic Model (BQM), 
//!       or from an Integer Model to a Binary Model.
//!     - Builtin serialization for maximum portability
//!     - Python-first development experience
//!
//! You can use LunaModel as a standalone package or by using [luna-quantum](https://pypi.org/project/luna-quantum/) 
//! which gives you additional builtin primitives to solve your optimization problems using the [Luna Platform](https://aqarios.com/platform).
