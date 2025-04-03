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

/// BackendStatusResponse : Backends Status Response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendStatusResponse {
    /// State of the backend
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<bool>,
    /// Status of the backend
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Description of the backend state
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Length of the queue for the backend
    #[serde(rename = "length_queue")]
    pub length_queue: i32,
    /// Backend version
    #[serde(rename = "backend_version", skip_serializing_if = "Option::is_none")]
    pub backend_version: Option<String>,
}

impl BackendStatusResponse {
    /// Backends Status Response
    pub fn new(length_queue: i32) -> BackendStatusResponse {
        BackendStatusResponse {
            state: None,
            status: None,
            message: None,
            length_queue,
            backend_version: None,
        }
    }
}

