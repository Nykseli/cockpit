use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OsRelease {
    pub bug_report_url: String,
    pub home_url: String,
    pub id_like: String,
    pub id: String,
    pub name: String,
    pub pretty_name: String,
    pub privacy_policy_url: String,
    pub support_url: String,
    pub version_codename: String,
    pub version_id: String,
    pub version: String,
    pub extra: BTreeMap<String, String>,
}
