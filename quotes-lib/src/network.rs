use log::{debug, error};

use tokio;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};

pub async fn read_u64(reader: &mut ReadHalf<'_>) -> Result<u64, io::Error> {
    while let Err(error) = reader.readable().await {
        error!("Connection is not readable yet: {}", error);
    }
    match reader.read_u64().await {
        Ok(value) => Ok(value),
        Err(error) => {
            error!("Failed to read value: {}", error);
            Err(error)
        }
    }
}

pub async fn write_u64(value: u64, writer: &mut WriteHalf<'_>) -> Result<(), io::Error> {
    match writer.write_u64(value).await {
        Ok(()) => {
            debug!("value sent: {}", value);
            Ok(())
        }
        Err(error) => {
            error!("Failed to send value: {}", error);
            Err(error)
        }
    }
}
