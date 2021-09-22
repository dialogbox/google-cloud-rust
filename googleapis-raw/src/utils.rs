use gcp_auth::Token;
use tonic::{
    codegen::InterceptedService,
    metadata::{AsciiMetadataValue, MetadataValue},
    service::Interceptor,
    transport::Channel,
    Status,
};

#[derive(Clone, Debug)]
pub struct AuthInterceptor {
    header_value: AsciiMetadataValue,
}

impl AuthInterceptor {
    pub async fn auth() -> Result<AuthInterceptor, gcp_auth::Error> {
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
