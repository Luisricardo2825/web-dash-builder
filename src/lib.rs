pub mod builder;
pub mod config_schema;
pub mod custom_tags;
pub mod header;

use napi::Either;
use napi_derive::napi;

use crate::config_schema::ConfigSchema;

#[napi]
pub fn build(arg: Option<Either<ConfigSchema, String>>) -> bool {
  return builder::build(arg);
}
