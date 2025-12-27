// gRPC client for communicating with kairos-core

use kairos_proto::trading_engine_client::TradingEngineClient;
use kairos_proto::{OrderRequest, OrderResponse};
use tonic::transport::Channel;

pub struct CoreGrpcClient {
    client: TradingEngineClient<Channel>,
}

impl CoreGrpcClient {
    pub async fn new(addr: &str) -> anyhow::Result<Self> {
        let client = TradingEngineClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }

    pub async fn place_order(
        &mut self,
        symbol: String,
        side: i32,
        quantity: f64,
        price: Option<f64>,
    ) -> anyhow::Result<OrderResponse> {
        let request = tonic::Request::new(OrderRequest {
            symbol,
            side,
            order_type: 0, // Market
            quantity,
            price,
        });

        let response = self.client.place_order(request).await?;
        Ok(response.into_inner())
    }
}
