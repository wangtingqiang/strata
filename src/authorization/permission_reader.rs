use async_trait::async_trait;
use thiserror::Error;

use crate::error::PortError;

#[derive(Debug, Error)]
pub enum PermissionReaderError {
    #[error(transparent)]
    Port(#[from] PortError),
}

#[async_trait]
pub trait PermissionReader: Send + Sync + 'static {
    async fn list_permission_codes(
        &self,
        user_id: &str,
    ) -> Result<Vec<String>, PermissionReaderError>;
}
