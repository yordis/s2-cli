use s2::{
    client::Client,
    types::{
        BasinConfig, BasinInfo, BasinName, CreateBasinRequest, DeleteBasinRequest,
        ListBasinsRequest, ListBasinsResponse, ReconfigureBasinRequest, StreamConfig,
    },
};

use crate::error::{ServiceError, ServiceErrorContext};

pub struct AccountService {
    client: Client,
}

impl AccountService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list_basins(
        &self,
        prefix: String,
        start_after: String,
        limit: Option<usize>,
    ) -> Result<ListBasinsResponse, ServiceError> {
        let list_basins_req = ListBasinsRequest::new()
            .with_prefix(prefix)
            .with_start_after(start_after)
            .with_limit(limit);

        self.client
            .list_basins(list_basins_req)
            .await
            .map_err(|e| ServiceError::new(ServiceErrorContext::ListBasins, e))
    }

    pub async fn create_basin(
        &self,
        basin: BasinName,
        storage_class: Option<crate::types::StorageClass>,
        retention_policy: Option<crate::types::RetentionPolicy>,
    ) -> Result<BasinInfo, ServiceError> {
        let mut stream_config = StreamConfig::new();

        if let Some(storage_class) = storage_class {
            stream_config = stream_config.with_storage_class(storage_class);
        }

        if let Some(retention_policy) = retention_policy {
            stream_config = stream_config.with_retention_policy(retention_policy.into());
        }

        let basin_config = BasinConfig {
            default_stream_config: Some(stream_config),
        };

        let create_basin_req = CreateBasinRequest::new(basin).with_config(basin_config);

        self.client
            .create_basin(create_basin_req)
            .await
            .map_err(|e| ServiceError::new(ServiceErrorContext::CreateBasin, e))
    }

    pub async fn delete_basin(&self, basin: BasinName) -> Result<(), ServiceError> {
        let delete_basin_req = DeleteBasinRequest::new(basin);
        self.client
            .delete_basin(delete_basin_req)
            .await
            .map_err(|e| ServiceError::new(ServiceErrorContext::DeleteBasin, e))
    }

    pub async fn get_basin_config(&self, basin: BasinName) -> Result<BasinConfig, ServiceError> {
        self.client
            .get_basin_config(basin)
            .await
            .map_err(|e| ServiceError::new(ServiceErrorContext::GetBasinConfig, e))
    }

    pub async fn reconfigure_basin(
        &self,
        basin: BasinName,
        basin_config: BasinConfig,
        mask: Vec<String>,
    ) -> Result<BasinConfig, ServiceError> {
        let reconfigure_basin_req = ReconfigureBasinRequest::new(basin)
            .with_config(basin_config)
            .with_mask(mask);
        self.client
            .reconfigure_basin(reconfigure_basin_req)
            .await
            .map_err(|e| ServiceError::new(ServiceErrorContext::ReconfigureBasin, e))
    }
}
