use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::database::AsyncDatabase;

struct TcpDatabaseServer {
    database: AsyncDatabase,
}

impl TcpDatabaseServer {
    async fn new(database: AsyncDatabase) -> Result<Self> {
        Ok(Self { database })
    }

    async fn handle_connection(self, mut stream: TcpStream) -> Result<()> {
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;

        // Deserialize the block of data from the buffer.

        // Process the block of data using the database.

        // Serialize the response block of data.

        stream.write(&buf[0..n]).await?;

        Ok(())
    }
}

impl Clone for TcpDatabaseServer {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let database = AsyncDatabase::new();
    let server = TcpDatabaseServer::new(database).await?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            server.clone().handle_connection(stream).await.unwrap();
        });
    }
}



mod tests {
    use super::*;
    #[allow(unused_imports)]
    use tokio::test;

    #[tokio::test]
    async fn test_tcp_database_server() -> Result<()> {
        let mut buf = vec![0; 1024];
        let database = AsyncDatabase::new();
        let server = TcpDatabaseServer::new(database).await?;

        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

        // Serialize a block of data to the buffer.

        stream.write(&buf[0..]).await?;

        // Read the response block of data from the stream.

        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;

        // Deserialize the response block of data.

        // Assert that the response block of data is correct.

        Ok(())
    }

}