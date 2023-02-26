use mozart::prelude::*;
use mozart::server;

#[tokio::main]
pub async fn main() -> Result<()> {
    let port = 50051;
    let addr = format!("[::1]:{}", port)
        .parse()
        .map_err(|_| Error::InvalidAddress(port))?;

    server::run(addr).await?;
    Ok(())
}
