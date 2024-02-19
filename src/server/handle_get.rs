use crate::common::{GetPath, HttpError};
use crate::parsing::match_get_path;
use chrono::{DateTime, Utc};

// Since this is a project for learning rust, GET will support the following:
// Get current time (in UTC)

pub async fn handle_get(payload : &str) -> Result<String, HttpError> {
    match match_get_path(payload) {
        Ok(get_path) => {
            match get_path {
                GetPath::CurrentTime => {
                    handle_current_time().await
                    }
               }
        }
        Err(err) => {
            match err {
                HttpError::PathNotSupported => {
                    println!("payload: {}", &payload);
                    Ok("😔 HTTP path not currently supported".to_string())
                }
                HttpError::UnableToParsePayload => {
                    Err(err)
                }
                HttpError::VerbNotSupported => {
                    Err(err)
                }
            }
        }
    }

   
}

pub async fn handle_current_time() -> Result<String, HttpError> {
    let current_time: DateTime<Utc> = Utc::now();

    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S UTC");

    Ok(formatted_time.to_string())
}