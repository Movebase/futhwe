pub use futhwe::futhwe_server::{Futhwe, FuthweServer};

pub mod futhwe {
    tonic::include_proto!("futhwe");
}

pub struct FuthweService {}

#[tonic::async_trait]
impl Futhwe for FuthweService {}
