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

/// JobMetricsBss : Contains information about job usage metrics
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobMetricsBss {
    /// Total billed time for the job - quantum seconds only.
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i32>,
}

impl JobMetricsBss {
    /// Contains information about job usage metrics
    pub fn new() -> JobMetricsBss {
        JobMetricsBss {
            seconds: None,
        }
    }
}

