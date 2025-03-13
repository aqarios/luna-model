// use super::versioned::{versionize, Version};
// use std::io;
//
// fn do_encode_versionized<T>(
//     encoder: &dyn Fn(&T, bool, Option<i32>) -> Result<Vec<u8>, io::Error>,
//     value: &T,
//     use_compression: bool,
//     level: Option<i32>,
//     version: Option<Version>,
// ) -> Result<Vec<u8>, io::Error> {
//     Ok(versionize(
//         encoder(value, use_compression, level)?,
//         version.unwrap_or(Version::latest()),
//     ))
// }
//
// // pub fn encode<T, S: Serializable<T>>(
// //     value: &T,
// //     use_compression: bool,
// //     level: Option<i32>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     compress(S::encode(value), use_compression, level)
// // }
//
// // pub fn encode_versionized<T, S: Serializable<T>>(
// //     value: &T,
// //     use_compression: bool,
// //     level: Option<i32>,
// //     version: Option<Version>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     do_encode_versionized(&encode::<T, S>, value, use_compression, level, version)
// // }
//
// pub fn encode(value: &T) -> Result<Vec<u8>, io::Error> {
//     todo!()
// }
//
// // // Expression
// // pub fn encode_expression(
// //     expression: &Expression<VarId, f64>,
// //     use_compression: bool,
// //     level: Option<i32>,
// // ) -> Result<Vec<u8>, std::io::Error> {
// //     compress(
// //         SerExpression::new(expression).encode_to_vec(),
// //         use_compression,
// //         level,
// //     )
// // }
// //
// // pub fn encode_versionized_expression(
// //     expression: &Expression<VarId, f64>,
// //     use_compression: bool,
// //     level: Option<i32>,
// //     version: Option<Version>,
// // ) -> Result<Vec<u8>, std::io::Error> {
// //     encode_versionized(
// //         &encode_expression,
// //         expression,
// //         use_compression,
// //         level,
// //         version,
// //     )
// // }
// //
// // // Constraints
// // pub fn encode_constraints(
// //     constraints: &Constraints<VarId, f64>,
// //     use_compression: bool,
// //     level: Option<i32>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     compress(
// //         SerConstraints::new(constraints, use_compression, level)?.encode_to_vec(),
// //         use_compression,
// //         level,
// //     )
// // }
// //
// // pub fn encode_versionized_constraints(
// //     constraints: &Constraints<VarId, f64>,
// //     use_compression: bool,
// //     level: Option<i32>,
// //     version: Option<Version>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     encode_versionized(
// //         &encode_constraints,
// //         constraints,
// //         use_compression,
// //         level,
// //         version,
// //     )
// // }
// //
// // // Environment
// // pub fn encode_environment(
// //     environment: &Environment<VarId>,
// //     use_compression: bool,
// //     level: Option<i32>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     compress(
// //         SerEnvironment::new(environment).encode_to_vec(),
// //         use_compression,
// //         level,
// //     )
// // }
// //
// // pub fn encode_versionized_environment(
// //     environment: &Environment<VarId>,
// //     use_compression: bool,
// //     level: Option<i32>,
// //     version: Option<Version>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     encode_versionized(
// //         &encode_environment,
// //         environment,
// //         use_compression,
// //         level,
// //         version,
// //     )
// // }
// //
// // // Model
// // pub fn encode_model(
// //     model: &Model<VarId, f64>,
// //     use_compression: bool,
// //     level: Option<i32>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     compress(
// //         SerModel::new(model, use_compression, level)?.encode_to_vec(),
// //         use_compression,
// //         level,
// //     )
// // }
// //
// // /// Alias for the latest version.
// // pub fn encode_versionized_model(
// //     model: &Model<VarId, f64>,
// //     use_compression: bool,
// //     level: Option<i32>,
// //     version: Option<Version>,
// // ) -> Result<Vec<u8>, io::Error> {
// //     encode_versionized(&encode_model, model, use_compression, level, version)
// // }
