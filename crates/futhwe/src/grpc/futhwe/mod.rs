use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::grpc::futhwe::futhwe::{OffchainFuzzingRequest, OffchainFuzzingResponse};

pub mod futhwe {
    tonic::include_proto!("futhwe");
}

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

        let mut out_stream = ReceiverStream::new(rx);
        while let Some(Ok(result)) = out_stream.next().await {
            let user_id = result.id; // Extract user ID from the request
            let mut dir_path = PathBuf::from(std::env::current_dir().unwrap());
            dir_path.push(user_id.to_string()); // Create directory with user ID

            if !dir_path.exists() {
                create_dir_all(&dir_path).unwrap_or_else(|_| {
                    eprintln!("Error creating directory: {:?}", dir_path);
                });
            }

            let mut file_path = dir_path.clone();
            file_path.push("build.zip"); // Create file path with "build.zip"
            let mut file = File::create(file_path).unwrap();
            file.write_all(&result.content).unwrap();
        }

        println!("stream ended");

        Ok(Response::new(OffchainFuzzingResponse {}))
    }
}
