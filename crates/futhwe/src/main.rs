use std::sync::Arc;

use anyhow::Result;
use common::config::Config;
use futhwe::grpc::{self, AppContext};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let config = Arc::new(Config::new()?);
    let ctx = AppContext { config };

    grpc::serve(ctx).await?;

    Ok(())
}
