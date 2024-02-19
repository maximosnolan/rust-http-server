use hyper::body::Body;

pub async fn format_response(payload : &str) -> String {
    let payload_length = payload.len();
    
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: text/plain\r\n\
        \r\n\
        {}",
        payload_length, payload 
    ); 

    response
}