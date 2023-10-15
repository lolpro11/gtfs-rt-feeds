use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub feeds: Vec<Feed>,
    #[serde(rename = "license_spdx_identifier")]
    pub license_spdx_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub id: String,
    pub spec: String,
    pub urls: Option<Urls>,
    pub license: Option<License>,
    pub authorization: Option<Authorization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    #[serde(rename = "realtime_alerts")]
    pub realtime_alerts: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "param_name")]
    pub param_name: Option<String>,
    #[serde(rename = "info_url")]
    pub info_url: Option<String>,
}
