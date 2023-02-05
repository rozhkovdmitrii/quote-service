use log::{debug, error};
use std::time::Duration;

use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::{io, select};

pub async fn call_timed_out<F>(future: F, timeout: Duration) -> F::Output
where F: std::future::Future<Output = Result<u64, io::Error>> {
    let sleep = tokio::time::sleep(timeout);
    tokio::pin!(sleep);
    select! {
        read_result = future =>  read_result,
        _ = &mut sleep => {
            error!("Reading bump_seed timed_out, break connection");
            return Err(io::Error::new(io::ErrorKind::TimedOut, "Future timed out"));
        }
    }
}

pub async fn read_u64(reader: &mut ReadHalf<'_>) -> Result<u64, io::Error> {
    while let Err(error) = reader.readable().await {
        error!("Connection is not readable yet: {}", error);
    }
    reader.read_u64().await
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
