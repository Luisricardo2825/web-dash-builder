pub mod builder;
pub mod config_schema;
pub mod custom_tags;
pub mod header;

use napi_derive::napi;

use crate::config_schema::ConfigSchema;
// Pub Either struct
pub use napi::Either;
#[napi]
pub fn build(arg: Option<Either<ConfigSchema, String>>) -> bool {
  return builder::build(arg);
}
