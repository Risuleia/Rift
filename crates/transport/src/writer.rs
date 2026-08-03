use bytes::Bytes;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::Result;

pub(crate) struct Writer<W> {
    inner: W,
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    pub(crate) fn new(inner: W) -> Self {
        Self { inner }
    }

    pub(crate) fn into_inner(self) -> W {
        self.inner
    }

    pub(crate) fn get_ref(&self) -> &W {
        &self.inner
    }

    pub(crate) fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    pub(crate) async fn write_all(&mut self, bytes: &Bytes) -> Result<()> {
        self.inner.write_all(bytes).await?;
        Ok(())
    }

    pub(crate) async fn flush(&mut self) -> Result<()> {
        self.inner.flush().await?;
        Ok(())
    }

    pub(crate) async fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown().await?;
        Ok(())
    }
}
