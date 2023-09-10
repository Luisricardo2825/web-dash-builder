use napi_derive::napi;
use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct ConfigSchema {
    pub src: Option<String>,
    pub out_dir: Option<String>,
    pub jsp: Option<Vec<Jsp>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct Jsp {
    #[serde(rename = "type")]
    pub type_field: String,
    pub code: Option<String>,
    pub path: Option<String>,
    pub var_name: Option<String>,
}
