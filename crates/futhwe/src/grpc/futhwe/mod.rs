pub mod futhwe {
    tonic::include_proto!("futhwe.v1");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("app_descriptor");
}

use std::{
    fs::{read_dir, OpenOptions},
    io::Write,
    path::PathBuf,
};

use base64::Engine;
use ityfuzz::fuzzers::move_fuzzer::{move_fuzzer, MoveFuzzConfig};
use serde_json::json;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    grpc::futhwe::futhwe::{OffchainFuzzingRequest, OffchainFuzzingResponse, SupportedVm},
    utils::datastore,
};

pub use futhwe::futhwe_server::{Futhwe, FuthweServer};

pub struct FuthweService {}

#[tonic::async_trait]
impl Futhwe for FuthweService {
    async fn offchain_fuzzing(
        &self,
        request: Request<OffchainFuzzingRequest>,
    ) -> Result<Response<OffchainFuzzingResponse>, Status> {
        let request = request.into_inner();
        let vm = SupportedVm::try_from(request.vm);
        let id = Uuid::new_v4().to_string();
        let dir_path = datastore::create_or_open(id).unwrap();

        let file_path = dir_path.clone().join("build.zip");

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let bytes_content = base64::engine::general_purpose::STANDARD
            .decode(request.base64_content)
            .map_err(|_| Status::data_loss("Invalid request"))?;

        file.write_all(&bytes_content).unwrap();

        if vm.is_err() {
            return Err(Status::data_loss("Invalid request"));
        }

        // unzip
        datastore::unzip_file(dir_path.clone(), "build.zip")
            .map_err(|_| Status::data_loss("Failed to unzip file"))?;

        let vm = vm.unwrap();

        if vm == SupportedVm::Move {
            let work_dir = dir_path.to_str().unwrap().to_string();
            let _ = tokio::spawn(async move {
                move_fuzzer(&MoveFuzzConfig {
                    target: format!("{}/{}", work_dir, "build"),
                    work_dir,
                    seed: 0,
                })
            })
            .await;
        }

        let vul_dir = dir_path.clone().join("vulnerabilities");
        let vul_dir = read_dir(vul_dir).unwrap();
        let mut output_file = PathBuf::new();

        // find a file with the name containing "_replayable"
        for entry in vul_dir {
            let entry = entry.unwrap();
            let path = entry.path();

            if path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("_replayable")
            {
                output_file = path;
            }
        }

        let mut replay: Vec<serde_json::Value> = Vec::new();

        if output_file.is_file() {
            // read the content as String
            for line in std::fs::read_to_string(output_file).unwrap().lines() {
                replay.push(json!(line));
            }
        }

        let vuln_info = dir_path.clone().join("vuln_info.jsonl");
        let mut result: Vec<serde_json::Value> = Vec::new();

        if vuln_info.is_file() {
            for line in std::fs::read_to_string(vuln_info).unwrap().lines() {
                result.push(json!(line));
            }
        }

        // remove directory
        let _ = std::fs::remove_dir_all(dir_path);

        let replay = serde_json::to_string(&replay).unwrap();
        let result = serde_json::to_string(&result).unwrap();

        return Ok(Response::new(OffchainFuzzingResponse { replay, result }));
    }
}
