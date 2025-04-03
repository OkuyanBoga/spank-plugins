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

/// JobState : Current state of the job
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobState {
    /// Current status of the job
    #[serde(rename = "status")]
    pub status: Status,
    /// Reason for the current status
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Reason code for the current status
    #[serde(rename = "reason_code", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<i32>,
    /// Next steps user can take in case of failure
    #[serde(rename = "reason_solution", skip_serializing_if = "Option::is_none")]
    pub reason_solution: Option<String>,
}

impl JobState {
    /// Current state of the job
    pub fn new(status: Status) -> JobState {
        JobState {
            status,
            reason: None,
            reason_code: None,
            reason_solution: None,
        }
    }
}
/// Current status of the job
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}

