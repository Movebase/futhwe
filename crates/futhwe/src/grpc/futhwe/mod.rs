pub mod futhwe {
    tonic::include_proto!("futhwe");
}

use std::{fs::OpenOptions, io::Write, path::PathBuf};

use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::{
    grpc::futhwe::futhwe::{OffchainFuzzingRequest, OffchainFuzzingResponse},
    utils::datastore,
};

pub use futhwe::futhwe_server::{Futhwe, FuthweServer};

pub struct FuthweService {}

#[tonic::async_trait]
impl Futhwe for FuthweService {
    async fn offchain_fuzzing(
        &self,
        request: Request<Streaming<OffchainFuzzingRequest>>,
    ) -> Result<Response<OffchainFuzzingResponse>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => tx.send(Ok(v)).await.expect("working rx"),
                    Err(err) => match tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break, // response was droped
                    },
                }
            }
        });

        println!("stream started");
        let mut total_bytes = 0;
        let mut dir_path = PathBuf::new();
        let mut out_stream = ReceiverStream::new(rx);

        while let Some(Ok(result)) = out_stream.next().await {
            total_bytes += result.content.len();
            dir_path = datastore::create_or_open(result.id).unwrap();
            let file_path = dir_path.clone().join("build.zip");

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .unwrap();

            file.write_all(&result.content).unwrap();
        }

        println!("stream ended with {} bytes", total_bytes);

        // unzip
        datastore::unzip_file(dir_path, "build.zip").unwrap();
        // remove directory
        // let _ = std::fs::remove_dir_all(dir_path);

        Ok(Response::new(OffchainFuzzingResponse {}))
    }
}
