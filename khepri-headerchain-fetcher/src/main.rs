use anyhow::Result;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    debug!("Starting khepri-headerchain-fetcher...");
    Ok(())
}
