use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{Result, reader::Reader, writer::Writer};

pub struct Connection<R, W> {
    reader: Reader<R>,
    writer: Writer<W>,
}

impl<R, W> Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    pub(crate) fn new(reader: R, writer: W) -> Self {
        Self { reader: Reader::new(reader), writer: Writer::new(writer) }
    }

    pub async fn send(&mut self, bytes: &Bytes) -> Result<()> {
        self.writer.write_all(bytes).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn receive(&mut self, length: usize) -> Result<Bytes> {
        self.reader.read_exact(length).await
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.writer.shutdown().await
    }
}
