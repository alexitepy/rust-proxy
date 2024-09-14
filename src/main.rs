use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use rand::seq::SliceRandom;
use std::fs;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8083").await.unwrap();
    println!("Proxy server listening on 127.0.0.1:8080...");

    let target_servers = Arc::new(read_target_servers("target_servers.txt").unwrap());

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let target_servers = Arc::clone(&target_servers);
                tokio::spawn(handle_connection(stream, target_servers));
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
}

async fn handle_connection(mut client_stream: TcpStream, target_servers: Arc<Vec<String>>) {
    let mut buffer = vec![0; 1024];
    client_stream.read(&mut buffer).await.unwrap();

    let target_server = target_servers.choose(&mut rand::thread_rng()).unwrap();

    println!("Selected target server: {}", target_server);

    let mut target_stream = TcpStream::connect(target_server).await.unwrap();
    target_stream.write_all(&buffer).await.unwrap();

    let mut target_buffer = vec![0; 1024];
    target_stream.read(&mut target_buffer).await.unwrap();
    client_stream.write_all(&target_buffer).await.unwrap();
}

fn read_target_servers(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let target_servers: Vec<String> = contents.lines().map(String::from).collect();
    Ok(target_servers)
}
