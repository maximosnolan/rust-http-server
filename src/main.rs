mod server; 
mod common;
mod parsing;

use crate::server::start_server;
#[tokio::main]
async fn main() {
    start_server().await;
}