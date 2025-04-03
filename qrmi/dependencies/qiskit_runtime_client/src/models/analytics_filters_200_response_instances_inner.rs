/*
 * Qiskit Runtime API
 *
 * The Qiskit Runtime API description
 *
 * The version of the OpenAPI document: 0.21.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsFilters200ResponseInstancesInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl AnalyticsFilters200ResponseInstancesInner {
    pub fn new(id: String, deleted: bool) -> AnalyticsFilters200ResponseInstancesInner {
        AnalyticsFilters200ResponseInstancesInner {
            id,
            deleted,
        }
    }
}

