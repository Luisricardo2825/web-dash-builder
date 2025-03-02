pub mod builder;
pub mod config_schema;
pub mod custom_tags;
pub mod header;

use napi::Either;
use napi_derive::napi;

use crate::config_schema::ConfigSchema;

#[napi]
pub fn build(arg: Option<Either<ConfigSchema, String>>, entry_file: Option<&str>) -> bool {
  return builder::build(arg, entry_file);
}
