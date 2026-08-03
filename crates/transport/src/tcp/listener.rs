use std::net::SocketAddr;

use tokio::net::{
    TcpListener as TokioTcpListener, ToSocketAddrs,
    tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::{Connection, Result};

pub struct TcpListener {
    inner: TokioTcpListener,
}

impl TcpListener {
    pub async fn bind<A>(address: A) -> Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self { inner: TokioTcpListener::bind(address).await? })
    }

    pub async fn accept(&self) -> Result<(Connection<OwnedReadHalf, OwnedWriteHalf>, SocketAddr)> {
        let (stream, address) = self.inner.accept().await?;

        let (reader, writer) = stream.into_split();

        Ok((Connection::new(reader, writer), address))
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.inner.local_addr()?)
    }
}
