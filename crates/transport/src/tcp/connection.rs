use tokio::net::{TcpStream, ToSocketAddrs, tcp::{OwnedReadHalf, OwnedWriteHalf}};

use crate::{Connection, Result};

pub struct TcpConnection;

impl TcpConnection {
    pub(crate) fn from_stream(
        stream: TcpStream,
    ) -> Connection<OwnedReadHalf, OwnedWriteHalf> {
        let (reader, writer) = stream.into_split();

        Connection::new(reader, writer)
    }

    pub async fn connect<A>(address: A) -> Result<Connection<OwnedReadHalf, OwnedWriteHalf>>
    where 
        A: ToSocketAddrs
    {
        let stream = TcpStream::connect(address).await?;

        stream.set_nodelay(true)?;

        let (reader, writer) = stream.into_split();

        Ok(Connection::new(reader, writer))
    }
}
