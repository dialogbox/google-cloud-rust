use std::time::{SystemTime, UNIX_EPOCH};

use gcp_auth::Token;
use tonic::codegen::InterceptedService;
use tonic::metadata::{AsciiMetadataValue, MetadataValue};
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::Status;

#[derive(Clone, Debug)]
pub struct AuthInterceptor {
    header_value: AsciiMetadataValue,
}

impl AuthInterceptor {
    pub async fn with_adc() -> Result<AuthInterceptor, GcpAuthError> {
        let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
        let authentication_manager = gcp_auth::init().await?;
        let token = authentication_manager.get_token(scopes).await?;

        Ok(Self::with_token(&token))
    }

    pub fn with_token(token: &Token) -> AuthInterceptor {
        let bearer_token = format!("Bearer {}", token.as_str());
        AuthInterceptor {
            header_value: MetadataValue::from_str(&bearer_token).unwrap(),
        }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.header_value.clone());
        Ok(request)
    }
}

pub type GcpService = InterceptedService<Channel, AuthInterceptor>;
pub type GcpAuthError = gcp_auth::Error;

/// Create a new channel used for the different types of clients
pub async fn connect(endpoint: &'static str) -> Result<Channel, tonic::transport::Error> {
    let channel = Channel::from_static(endpoint).connect().await?;

    Ok(channel)
}

#[allow(dead_code)]
fn timestamp() -> u128 {
    let start = SystemTime::now();
    let time = start
        .duration_since(UNIX_EPOCH)
        .expect("Failed to fetch timestamp");
    time.as_micros()
}
