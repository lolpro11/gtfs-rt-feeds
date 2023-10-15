use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "$id")]
    pub id: Option<String>,
    pub feeds: Option<Vec<Feed>>,
    pub operators: Option<Vec<Operator>>,
    #[serde(rename = "license_spdx_identifier")]
    pub license_spdx_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub id: String,
    pub supersedes_ids: Option<Vec<String>>,
    pub spec: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub urls: Urls,
    pub languages: Option<Vec<String>>,
    pub license_description: Option<LicenseDescription>,
    pub authorization: Option<Authorization>,
    pub operators: Option<Vec<Operator>>,
    //pub tags: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    pub static_current: Option<String>,
    pub static_historic: Option<Vec<String>>,
    pub static_planned: Option<Vec<String>>,
    pub static_hypothetical: Option<Vec<String>>,
    pub realtime_vehicle_positions: Option<String>,
    pub realtime_trip_updates: Option<String>,
    pub realtime_alerts: Option<String>,
    pub gbfs_auto_discovery: Option<String>,
    pub mds_provider: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseDescription {
    pub spdx_identifier: Option<String>,
    pub url: Option<String>,
    pub use_without_attribution: Option<String>,
    pub create_derived_product: Option<String>,
    pub redistribution_allowed: Option<String>,
    pub commercial_use_allowed: Option<String>,
    pub share_alike_optional: Option<String>,
    pub attribution_text: Option<String>,
    pub attribution_instructions: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    #[serde(rename = "type")]
    pub type_auth: String,
    pub param_name: Option<String>,
    pub info_url: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    #[serde(rename = "onestop_id")]
    pub onestop_id: String,
    pub supersedes_ids: Option<Vec<String>>,
    pub name: String,
    pub short_name: Option<String>,
    pub website: Option<String>,
    pub associated_feeds: Option<Vec<Associated>>,
    //pub tags: Option<Vec<String>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Associated {
    pub gtfs_agency_id: Option<String>,
    pub feed_onestop_id: Option<String>,
}