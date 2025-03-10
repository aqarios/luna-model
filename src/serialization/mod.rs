// This module is responsible for serialization and deserialization
// of all objects used in the aqmodels library using the `prost!` library.
// The cool thing with protobuf is that it has automatic verioning, nevertheless, we
// use our own wrapper that includes a version, just to be sure and make the versions
// declerative. However, using plain protobuf nothing would break.

mod ser_constr;
mod ser_env;
mod ser_expression;
mod ser_model;
mod versioned;

mod decoder;
mod encoder;

pub use decoder::decode_model;
pub use encoder::encode_model;
