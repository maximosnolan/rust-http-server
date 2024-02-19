use tokio::net::TcpListener;

use crate::server::handle_client;

pub async fn start_server() {
    println!("starting server...");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080...");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let std_stream = stream.into_std();
                match std_stream {
                    Ok(tcp_stream) => {
                        tokio::spawn(async move {
                            handle_client(tcp_stream).await;
                        });
                    }
                    Err(err) => {
                        println!("unable to fetch tcp stream {}", err);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}