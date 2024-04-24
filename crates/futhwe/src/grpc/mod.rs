use common::config::Config;
use std::{sync::Arc, time::Duration};
use tonic::transport::Server;
use tonic_health::server::HealthReporter;
use tracing::info;

use self::futhwe::{FuthweServer, FuthweService};

mod futhwe;

pub struct AppContext {
    pub config: Arc<Config>,
}

pub async fn serve(ctx: AppContext) -> anyhow::Result<()> {
    let config = ctx.config;

    let addr = format!("{}:{}", config.app.host, config.app.port)
        .parse()
        .unwrap();

    let layer = tower::ServiceBuilder::new().into_inner();

    let futhwe_service = FuthweService {};
    let futhwe_server = FuthweServer::new(futhwe_service);

    info!("gRPC server started at {}", addr);

    // Health check
    let (mut health_reporter, health_server) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<FuthweServer<FuthweService>>()
        .await;

    tokio::spawn(twiddle_service_status(health_reporter.clone()));

    // Reflection
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(futhwe::futhwe::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .layer(layer)
        .add_service(health_server)
        .add_service(futhwe_server)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}

/// This function (somewhat improbably) flips the status of a service every second, in order
/// that the effect of `tonic_health::HealthReporter::watch` can be easily observed.
async fn twiddle_service_status(mut reporter: HealthReporter) {
    let mut iter = 0u64;
    loop {
        iter += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;

        if iter % 2 == 0 {
            reporter.set_serving::<FuthweServer<FuthweService>>().await;
        } else {
            reporter
                .set_not_serving::<FuthweServer<FuthweService>>()
                .await;
        };
    }
}
