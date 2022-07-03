use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::io::{Read, Write};
use crate::utils::io::*;

#[derive(Debug)]
pub enum SerdeError<E> {
    IoError(E),
    SerdeError,
}

#[cfg(feature = "json_io")]
pub fn read_buf<'a, R, T>(read: R, buf: &'a mut [u8]) -> Result<T, SerdeError<R::Error>>
where
    R: Read,
    T: Deserialize<'a>,
{
    let read_len = try_read_full(read, buf).map_err(|(e, _)| SerdeError::IoError(e))?;

    let result = serde_json::from_slice(&buf[..read_len]).map_err(|_| SerdeError::SerdeError)?;

    Ok(result)
}

#[cfg(feature = "json_io")]
pub fn read<const N: usize, R, T>(read: R) -> Result<T, SerdeError<R::Error>>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut buf = [0_u8; N];

    let read_len = try_read_full(read, &mut buf).map_err(|(e, _)| SerdeError::IoError(e))?;

    let result = serde_json::from_slice(&buf[..read_len]).map_err(|_| SerdeError::SerdeError)?;

    Ok(result)
}

#[cfg(feature = "json_io")]
pub fn write<const N: usize, W, T>(mut write: W, value: &T) -> Result<(), SerdeError<W::Error>>
where
    W: Write,
    T: Serialize,
{
    let vec = serde_json::to_vec(value).map_err(|_| SerdeError::SerdeError)?;

    write.write_all(&vec).map_err(SerdeError::IoError)
}

#[cfg(feature = "json_io")]
pub fn req_write<const N: usize, R, T>(
    mut req: R,
    value: &T,
) -> Result<R::Write, SerdeError<R::Error>>
where
    R: crate::http::client::Request,
    T: Serialize,
{
    req.set_header("Content-Type", "application/json");

    let mut writer = req.into_writer().map_err(SerdeError::IoError)?;

    write::<N, _, _>(&mut writer, value)?;

    Ok(writer)
}

#[cfg(feature = "json_io")]
pub fn resp_write<const N: usize, P, T>(
    mut response: P,
    value: &T,
) -> Result<P::Write, SerdeError<P::Error>>
where
    P: crate::http::server::Response,
    T: Serialize,
{
    response.set_header("Content-Type", "application/json");

    let mut writer = response.into_writer().map_err(SerdeError::IoError)?;

    write::<N, _, _>(&mut writer, value)?;

    Ok(writer)
}

pub mod asynch {
    use serde::{de::DeserializeOwned, Deserialize, Serialize};

    use crate::io::asynch::{Read, Write};
    use crate::utils::io::asynch::*;

    pub use super::SerdeError;

    #[cfg(feature = "json_io")]
    pub async fn read_buf<'a, R, T>(read: R, buf: &'a mut [u8]) -> Result<T, SerdeError<R::Error>>
    where
        R: Read,
        T: Deserialize<'a>,
    {
        let read_len = try_read_full(read, buf)
            .await
            .map_err(|(e, _)| SerdeError::IoError(e))?;

        let result =
            serde_json::from_slice(&buf[..read_len]).map_err(|_| SerdeError::SerdeError)?;

        Ok(result)
    }

    #[cfg(feature = "json_io")]
    pub async fn read<const N: usize, R, T>(read: R) -> Result<T, SerdeError<R::Error>>
    where
        R: Read,
        T: DeserializeOwned,
    {
        let mut buf = [0_u8; N];

        let read_len = try_read_full(read, &mut buf)
            .await
            .map_err(|(e, _)| SerdeError::IoError(e))?;

        let result =
            serde_json::from_slice(&buf[..read_len]).map_err(|_| SerdeError::SerdeError)?;

        Ok(result)
    }

    #[cfg(feature = "json_io")]
    pub async fn write<const N: usize, W, T>(
        mut write: W,
        value: &T,
    ) -> Result<(), SerdeError<W::Error>>
    where
        W: Write,
        T: Serialize,
    {
        let vec = serde_json::to_vec(value).map_err(|_| SerdeError::SerdeError)?;

        write.write_all(&vec).await.map_err(SerdeError::IoError)
    }
}