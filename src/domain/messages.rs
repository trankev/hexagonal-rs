#[derive(Debug, serde::Serialize)]
#[serde(tag="type")]
pub enum Severity {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug, serde::Serialize)]
pub struct Message {
    severity: Severity,
    message: String,
    code: String,
}

impl Message {
    pub fn internal_error() -> Message {
        Message {
            severity: Severity::Error,
            message: "Internal server error".to_string(),
            code:"internal_server_error".to_string(),
        }
    }
}
