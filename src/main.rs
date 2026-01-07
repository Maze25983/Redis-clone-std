mod commnad;
mod database;
mod server;

use crate::server::create_server;

#[tokio::main]
async fn main() {
    let add = "127.0.0.1:3002";
    eprint!("Server will Start at {:?}", add);

    create_server(add).await

    
}
