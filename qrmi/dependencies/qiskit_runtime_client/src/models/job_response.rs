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

/// JobResponse : A job
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponse {
    /// Identifier assigned to the job
    #[serde(rename = "id")]
    pub id: String,
    /// &ast;&ast;Warning:&ast;&ast; While this parameter is not currently required for requests, specifying it is strongly encouraged. Running an ISA circuit on a backend that has a different instruction set will result in an error. The backend parameter will be required in a future release.  The backend on which to run the program.  If no backend is specified, the job is sent to the backend with the shortest queue that you have access to. 
    #[serde(rename = "backend", skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,
    #[serde(rename = "state")]
    pub state: Box<models::JobState>,
    /// Current status of the job
    #[serde(rename = "status")]
    pub status: Status,
    /// Parameters used to execute the job
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "program")]
    pub program: Box<models::JobResponseProgram>,
    /// UTC timestamp for when the job was created
    #[serde(rename = "created")]
    pub created: String,
    /// Name and tag of the image to use when running a program (IBM Quantum channel users only)
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Cost of the job
    #[serde(rename = "cost")]
    pub cost: i32,
    /// List of job or program tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "remote_storage", skip_serializing_if = "Option::is_none")]
    pub remote_storage: Option<Box<models::JobResponseRemoteStorage>>,
    /// Identifier of the session that the job is a part of
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// The id of the user submitted the job
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<models::Usage>>,
}

impl JobResponse {
    /// A job
    pub fn new(id: String, state: models::JobState, status: Status, program: models::JobResponseProgram, created: String, cost: i32) -> JobResponse {
        JobResponse {
            id,
            backend: None,
            state: Box::new(state),
            status,
            params: None,
            program: Box::new(program),
            created,
            runtime: None,
            cost,
            tags: None,
            remote_storage: None,
            session_id: None,
            user_id: None,
            usage: None,
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
    #[serde(rename = "Cancelled - Ran too long")]
    CancelledRanTooLong,
    #[serde(rename = "Failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}

