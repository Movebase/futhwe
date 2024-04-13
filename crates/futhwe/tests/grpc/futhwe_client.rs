pub mod futhwe {
    tonic::include_proto!("futhwe.v1");
}

use std::{fs::File, io::Read};

use anyhow::Result;
use base64::Engine;
use futhwe::futhwe_client::FuthweClient;
use tonic::transport::Channel;

use self::futhwe::{OffchainFuzzingRequest, SupportedVm};

async fn test_offchain_fuzzing(client: &mut FuthweClient<Channel>, vm: SupportedVm) -> Result<()> {
    let file_path = std::env::var("FILE_PATH").expect("FILE_PATH must be set");
    let mut file = File::open(file_path).unwrap();

    let name = "test".to_string();
    let mut content = vec![];
    file.read_to_end(&mut content)?;
    let base64_content = base64::engine::general_purpose::STANDARD.encode(content);

    let request = OffchainFuzzingRequest {
        name,
        vm: vm as i32,
        base64_content,
    };

    let response = client.offchain_fuzzing(request).await?;
    let response = response.into_inner(); // Discard response (assumed empty for simplicity)

    println!("Response: {:?}", response);

    Ok(())

    // Start streaming with an empty vec
}

#[tokio::test]
async fn test() -> Result<()> {
    let mut client = FuthweClient::connect("http://127.0.0.1:50051").await?;

    test_offchain_fuzzing(&mut client, futhwe::SupportedVm::Move).await?;

    Ok(())
}
