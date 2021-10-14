use std::sync::Arc;

use rusoto_credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials};

pub fn empty() -> ChainProvider {
    ChainProvider::empty()
}

pub fn default() -> ChainProvider {
    ChainProvider::default()
}

#[derive(Clone)]
pub struct ChainProvider {
    provider_list: Vec<Arc<dyn ProvideAwsCredentials + Send + Sync>>,
}

impl ChainProvider {
    pub fn empty() -> Self {
        Self {
            provider_list: Vec::new(),
        }
    }

    pub fn with_default_chain_provider(self) -> Self {
        self.with(rusoto_credential::ChainProvider::default())
    }

    pub fn with(mut self, provider: impl ProvideAwsCredentials + Send + Sync + 'static) -> Self {
        self.provider_list.push(Arc::new(provider) as _);
        self
    }
}

impl Default for ChainProvider {
    fn default() -> Self {
        Self::empty().with_default_chain_provider()
    }
}

#[async_trait::async_trait]
impl ProvideAwsCredentials for ChainProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        for provider in self.provider_list.iter() {
            if let Ok(credentials) = provider.credentials().await {
                return Ok(credentials);
            }
        }
        Err(CredentialsError::new(
            "Couldn't find AWS credentials in chained providers list.",
        ))
    }
}
