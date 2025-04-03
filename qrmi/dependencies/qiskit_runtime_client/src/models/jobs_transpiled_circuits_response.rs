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

/// JobsTranspiledCircuitsResponse : Jobs Transpiled Circuits presigned URL to download
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsTranspiledCircuitsResponse {
    /// URL to download the transpiled circuits
    #[serde(rename = "url")]
    pub url: String,
}

impl JobsTranspiledCircuitsResponse {
    /// Jobs Transpiled Circuits presigned URL to download
    pub fn new(url: String) -> JobsTranspiledCircuitsResponse {
        JobsTranspiledCircuitsResponse {
            url,
        }
    }
}

