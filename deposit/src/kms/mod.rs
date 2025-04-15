pub mod provider;
mod sample_provider;

use anyhow::{
    Result,
    anyhow,
};
use provider::KMSProvider;

pub fn create_provider(provider_type: &str) -> Result<Box<dyn KMSProvider>> {
    match provider_type.to_lowercase().as_str() {
        "" => Ok(Box::new(sample_provider::SampleProvider::new())),
        // TODO: Add more providers as needed
        _ => Err(anyhow!("Unsupported KMS provider: {}", provider_type)),
    }
}
