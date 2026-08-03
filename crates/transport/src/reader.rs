use std::vec;

use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{Result, TransportError};

pub(crate) struct Reader<R> {
    inner: R,
}

impl<R> Reader<R>
where
    R: AsyncRead + Unpin,
{
    pub(crate) fn new(inner: R) -> Self {
        Self { inner }
    }

    pub(crate) fn into_inner(self) -> R {
        self.inner
    }

    pub(crate) fn get_ref(&self) -> &R {
        &self.inner
    }

    pub(crate) fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    pub(crate) async fn read_exact(
        &mut self,
        length: usize
    ) -> Result<Bytes> {
        let mut buffer = vec![0; length];

        match self.inner.read_exact(&mut buffer).await {
            Ok(_) => Ok(Bytes::from(buffer)),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                Err(TransportError::ConnectionClosed)
            },
            Err(e) => Err(e.into())
        }
    }
}
