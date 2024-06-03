use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultEntryResource {
    pub id: String,
    pub policy_version: String,
    pub kind: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultEntry {
    pub resource: ResultEntryResource,
    pub actions: HashMap<String, i32>,
    pub meta: ResultEntryMeta,
    pub outputs: Vec<OutputEntry>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultEntryMeta {
    pub actions: HashMap<String, EffectMeta>,
    pub effective_derived_roles: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectMeta {
    pub matched_scope: String,
    pub matched_policy: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputEntry {
    pub src: String,
    pub val: Value,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub results: Vec<ResultEntry>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceEntry {
    pub actions: Vec<String>,
    pub resource: Resource,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub id: Value,
    #[serde(default)]
    pub policy_version: String,
    pub kind: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub attr: Map<String, Value>,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Principal {
    pub id: Value,
    #[serde(default)]
    pub policy_version: String,
    pub roles: Value,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub attr: Map<String, Value>,
}

#[derive(Deserialize, Default, Serialize)]
pub struct AuxData {
    #[serde(default)]
    pub jwt: Map<String, Value>,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub request_id: String,
    pub principal: Principal,
    pub resources: Vec<ResourceEntry>,
    #[serde(default, alias = "aux_data", alias = "auxData")]
    pub aux_data: AuxData,
}
