// gRPC Server - receives orders from satellites

use kairos_proto::trading_engine_server::{TradingEngine as TradingEngineService, TradingEngineServer};
use kairos_proto::{OrderRequest, OrderResponse, CancelOrderRequest, BalanceRequest, BalanceResponse, OrderStatusRequest, OrderStatusResponse};
use tonic::{transport::Server, Request, Response, Status};

pub struct GrpcServer;

#[tonic::async_trait]
impl TradingEngineService for GrpcServer {
    async fn place_order(
        &self,
        request: Request<OrderRequest>,
    ) -> Result<Response<OrderResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Received order via gRPC: {:?}", req);

        // TODO: Convert to InternalOrder and send to MPSC channel
        
        let response = OrderResponse {
            success: true,
            order_id: uuid::Uuid::new_v4().to_string(),
            message: "Order received".to_string(),
            status: 0, // Pending
        };

        Ok(Response::new(response))
    }

    async fn cancel_order(
        &self,
        _request: Request<CancelOrderRequest>,
    ) -> Result<Response<OrderResponse>, Status> {
        // TODO: Implement order cancellation
        Err(Status::unimplemented("Not implemented yet"))
    }

    async fn get_balance(
        &self,
        _request: Request<BalanceRequest>,
    ) -> Result<Response<BalanceResponse>, Status> {
        // TODO: Return actual balance from RiskEngine
        let response = BalanceResponse {
            available: 10000.0,
            locked: 0.0,
            total: 10000.0,
        };
        Ok(Response::new(response))
    }

    async fn get_order_status(
        &self,
        _request: Request<OrderStatusRequest>,
    ) -> Result<Response<OrderStatusResponse>, Status> {
        // TODO: Implement order status lookup
        Err(Status::unimplemented("Not implemented yet"))
    }
}

pub async fn start_grpc_server(addr: String) -> anyhow::Result<()> {
    let service = GrpcServer;
    let addr = addr.parse()?;

    tracing::info!("🌐 Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(TradingEngineServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
