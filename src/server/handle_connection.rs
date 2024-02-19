use std::net::TcpStream;
use std::io::{BufWriter, Read, Write};
use std::time::Duration;

use tokio::io;

use crate::parsing::match_http_method;

use crate::server::{format_response, handle_http_request};


pub async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    //let num_bytes = stream.read(&mut buffer).unwrap();
    
    loop {
        match stream.read(&mut buffer) {
            Ok(num_bytes) => {
                // Handle the case where data was successfully read
                println!("Read {} bytes from the stream", num_bytes);
                match String::from_utf8(buffer[0..num_bytes].to_vec()) {
                    Ok(str_buff) => {
                        let request_type = match_http_method(&str_buff);
                        
                        let response = handle_http_request(&str_buff, request_type).await;
                        match response {
                            Ok(resp) => {
                                let http_resp = format_response(&resp);
                               
                                let mut writer = BufWriter::new(&stream);
                                writer.write_all(http_resp.await.as_bytes());
                                writer.flush();
                                break; 
                            }
                            Err(err) => {
                                eprint!("Error encounred handling request {:?} .", err);
                                break;
                            }
                        }
                        
                    }
                    Err(err) => {
                        eprint!("unable to convert buffer into str {}", err);
                        break; 
                    }
                }
                // Further processing of the data...
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Handle the case where the read operation would block
                println!("Read operation would block, no data available yet");
                std::thread::sleep(Duration::from_millis(100));
                // You might want to retry the read operation or do something else...
            }
            Err(e) => {
                // Handle other error cases
                eprintln!("Error reading from stream: {}", e);
                break;
                // Decide how to handle the error based on the specific requirements of your application
            }
        }
    }
    //stream.flush().unwrap();
}


