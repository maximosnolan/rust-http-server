use crate::common::*;
use crate::parsing::*;

pub fn match_http_method(method: &str) -> HttpMethod {

    let payload_parts : Vec<&str> = method.split_whitespace().collect();

    if let Some(method_type) = payload_parts.first() {
        match *method_type {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "PATCH" => HttpMethod::PATCH,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::Unknown,
        }
    } else {
        dbg!("Unable to parse method str");
        return HttpMethod::Unknown;
    }
}

pub fn match_get_path(method: &str) -> Result<GetPath, HttpError> {
    let payload_parts : Vec<&str> = method.split_whitespace().collect();

    let path = payload_parts.get(1).map(|&s| s);

    if let Some(get_path) = path {
        let parsed_get_path = remove_first_slash(get_path);
        match parsed_get_path {
            "current-time" => Ok(GetPath::CurrentTime),
            _ => Err(HttpError::PathNotSupported)
        }
    } else {
        dbg!("Unable to parse method str");
        println!("HERE");
        Err(HttpError::PathNotSupported)
    }
}