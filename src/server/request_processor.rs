use crate::common::*;

use super::handle_get::handle_get;

pub async fn handle_http_request (payload : &str, method_type : HttpMethod) -> Result<(String), HttpError> {
    match method_type {
        HttpMethod::GET => {
            handle_get(payload).await
        }
        _ => {
            println!("Unknown or not yet implimented");
            Err(HttpError::VerbNotSupported)
        }
    }
}