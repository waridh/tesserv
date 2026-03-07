/*!
Module that contains error for the HTTP runtime
 */

use std::fmt::Display;
use warp::{
    Reply,
    http::StatusCode,
    reject::{Reject, Rejection},
};

#[derive(Clone, Debug)]
pub enum RuntimeError {
    DownloadFailure,
    InvalidFiletype,
    ExecutionFailure(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            RuntimeError::DownloadFailure => "Failed to download file".to_string(),
            RuntimeError::InvalidFiletype => "Incorrect file type".to_string(),
            RuntimeError::ExecutionFailure(x) => format!("Failed to execute job: {}", x),
        };
        write!(f, "{}", value)
    }
}

impl Reject for RuntimeError {}

/**
Error handler function. Translates internal error representation to HTTP valid
return code.
 */
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    let (message, code) = if let Some(error) = r.find::<RuntimeError>() {
        let err = match error {
            RuntimeError::DownloadFailure => (error.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            RuntimeError::InvalidFiletype => {
                (error.to_string(), StatusCode::UNSUPPORTED_MEDIA_TYPE)
            }
            RuntimeError::ExecutionFailure(..) => (
                "failed to execute submission".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        err
    } else if r.is_not_found() {
        ("Not Found".to_string(), StatusCode::NOT_FOUND)
    } else if r.find::<warp::reject::PayloadTooLarge>().is_some() {
        ("Payload too large".to_string(), StatusCode::BAD_REQUEST)
    } else {
        eprintln!("unhandled error: {:?}", r);
        (
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    };
    Ok(warp::reply::with_status(message, code))
}
