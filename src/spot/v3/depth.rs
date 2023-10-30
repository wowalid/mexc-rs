use async_trait::async_trait;
use rust_decimal::Decimal;
use crate::spot::{MexcSpotApiClient, MexcSpotApiClientWithAuthentication, MexcSpotApiEndpoint};
use crate::spot::v3::{ApiResponse, ApiResult};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthParams<'a> {
    /// Symbol
    pub symbol: &'a str,
    /// Return number default 100; max 5000
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PriceAndQuantity {
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthOutput {
    pub last_update_id: u64,
    pub bids: Vec<PriceAndQuantity>,
    pub asks: Vec<PriceAndQuantity>,
}

#[async_trait]
pub trait DepthEndpoint {
    /// Order book
    async fn depth(&self, params: DepthParams<'_>) -> ApiResult<DepthOutput>;
}

async fn depth_impl(
    endpoint: &MexcSpotApiEndpoint,
    client: &reqwest::Client,
    params: DepthParams<'_>,
) -> ApiResult<DepthOutput> {
    let endpoint = format!("{}/api/v3/depth", endpoint.as_ref());
    let response = client.get(&endpoint).query(&params).send().await?;
    let api_response = response.json::<ApiResponse<DepthOutput>>().await?;
    let output = api_response.into_api_result()?;

    Ok(output)
}

#[async_trait]
impl DepthEndpoint for MexcSpotApiClient {
    async fn depth(&self, params: DepthParams<'_>) -> ApiResult<DepthOutput> {
        depth_impl(&self.endpoint, &self.reqwest_client, params).await
    }
}

#[async_trait]
impl DepthEndpoint for MexcSpotApiClientWithAuthentication {
    async fn depth(&self, params: DepthParams<'_>) -> ApiResult<DepthOutput> {
        depth_impl(&self.endpoint, &self.reqwest_client, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_depth() {
        let client = MexcSpotApiClient::default();
        let depth_params = DepthParams {
            symbol: "BTCUSDT",
            limit: None,
        };
        let result = client.depth(depth_params).await;
        assert!(result.is_ok());
    }
}
