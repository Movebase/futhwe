use std::sync::Arc;

use anyhow::Result;
use app::{
    grpc::{self, AppContext},
    message,
};
use common::config::Config;
use database::get_connection;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    grpc::serve().await?;

    Ok(())
}
