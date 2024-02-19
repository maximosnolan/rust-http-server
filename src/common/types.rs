#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive()]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    Unknown,
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum HttpError {
    VerbNotSupported,
    PathNotSupported,
    UnableToParsePayload,

}

pub enum GetPath {
    CurrentTime,
}

struct PostData {
    
}