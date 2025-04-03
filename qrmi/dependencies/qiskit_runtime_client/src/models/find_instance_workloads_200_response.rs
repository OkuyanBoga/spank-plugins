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
pub struct FindInstanceWorkloads200Response {
    #[serde(rename = "workloads")]
    pub workloads: Vec<models::FindInstanceWorkloads200ResponseWorkloadsInner>,
    #[serde(rename = "total_count")]
    pub total_count: f64,
    #[serde(rename = "limit")]
    pub limit: f64,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<Box<models::FindInstanceWorkloads200ResponsePrevious>>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<models::FindInstanceWorkloads200ResponseNext>>,
}

impl FindInstanceWorkloads200Response {
    pub fn new(workloads: Vec<models::FindInstanceWorkloads200ResponseWorkloadsInner>, total_count: f64, limit: f64) -> FindInstanceWorkloads200Response {
        FindInstanceWorkloads200Response {
            workloads,
            total_count,
            limit,
            previous: None,
            next: None,
        }
    }
}

