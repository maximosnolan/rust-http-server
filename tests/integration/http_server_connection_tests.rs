mod common;

#[cfg(test)]
mod tests {
    use self::common::read_sample_request;
    
    use super::*;
    use hyper::server;
    use rust_http_server::parsing::match_http_method;
    use rust_http_server::common::HttpMethod;
    use rust_http_server::server::start_server;
    use reqwest::Error;
    #[test]
    fn parse_get_http_payload() {
        match read_sample_request("get.txt") {
            Ok(payload) => {
                assert_eq!(HttpMethod::GET, match_http_method(&payload));
            }
            Err(err) => {
                eprint!("test file does not exist with error : {}", err);
            }
        }
    }
    #[test]
    fn parse_post_http_payload() {
        match read_sample_request("post.txt") {
            Ok(payload) => {
                assert_eq!(HttpMethod::POST, match_http_method(&payload));
            }
            Err(err) => {
                eprint!("test file does not exist with error : {}", err);
            }
        }
    }

    #[test]
    fn parse_invalid_http_payload() {
        match read_sample_request("invalid.txt") {
            Ok(payload) => {
                assert_eq!(HttpMethod::Unknown, match_http_method(&payload));
            }
            Err(err) => {
                eprint!("test file does not exist with error : {}", err);
            }
        }
    }


    #[tokio::test]
    async fn test_server_response()  -> Result<(), Box<dyn std::error::Error>> {
        // Start the server in a separate task
        let server_task = tokio::spawn(async move {
            start_server().await;
            println!("STARTED SERVER");
        });

        // // Give the server some time to start
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Make a request to the server
        let url = "http://127.0.0.1:8080/current-time";
        let response = reqwest::get(url).await;
        println!("resp gotten");
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Read the response body as a string
                    let body: String = resp.text().await?;
                    println!("body is {}", body);
                    Ok(())
                } else {
                    Err(format!("Request failed with status code: {}", resp.status()).into())
                }
            }
            Err(err) => {
                eprint!("unable to unwrap resp");
                Err(err.into())
            }
        }

    }
}
