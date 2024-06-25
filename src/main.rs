pub mod builder;
pub mod config_schema;
pub mod custom_tags;
pub mod header;
use builder::build;
use napi::Either;

use crate::config_schema::ConfigSchema;

fn main() {
  build(Some(Either::A(ConfigSchema {
    out_dir: Some("./dist".to_string()),
    src: Some("./build".to_string()),
    jsp: None,
  })));
}
