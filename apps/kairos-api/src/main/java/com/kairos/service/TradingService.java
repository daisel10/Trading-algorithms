package com.kairos.service;

import com.kairos.grpc.TradingEngineGrpcClient;
import com.kairos.model.Order;
import com.kairos.model.dto.OrderResponse;
import com.kairos.model.dto.PlaceOrderRequest;
import com.kairos.repository.OrderRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import trading_engine.BalanceResponse;
import trading_engine.OrderSide;
import trading_engine.OrderType;

import java.time.Instant;
import java.util.UUID;

@Slf4j
@Service
@RequiredArgsConstructor
public class TradingService {
    
    private final TradingEngineGrpcClient grpcClient;
    private final OrderRepository orderRepository;
    private final RealtimeDataService realtimeDataService;
    
    /**
     * Place an order by calling gRPC to kairos-core
     */
    public Mono<OrderResponse> placeOrder(PlaceOrderRequest request) {
        log.info("Placing order: {}", request);
        
        // Map string to enum
        OrderSide side = "BUY".equalsIgnoreCase(request.getSide()) ? OrderSide.BUY : OrderSide.SELL;
        OrderType type = "MARKET".equalsIgnoreCase(request.getOrderType()) ? OrderType.MARKET : OrderType.LIMIT;
        
        return grpcClient.placeOrder(
            request.getSymbol(),
            side,
            type,
            request.getQuantity(),
            request.getPrice()
        )
        .map(grpcResponse -> OrderResponse.builder()
            .success(grpcResponse.getSuccess())
            .orderId(grpcResponse.getOrderId())
            .message(grpcResponse.getMessage())
            .status(grpcResponse.getStatus().name())
            .build())
        .doOnSuccess(response -> log.info("Order placed: {}", response));
    }
    
    /**
     * Cancel an order via gRPC
     */
    public Mono<OrderResponse> cancelOrder(String orderId) {
        log.info("Cancelling order: {}", orderId);
        
        return grpcClient.cancelOrder(orderId)
            .map(grpcResponse -> OrderResponse.builder()
                .success(grpcResponse.getSuccess())
                .orderId(grpcResponse.getOrderId())
                .message(grpcResponse.getMessage())
                .status(grpcResponse.getStatus().name())
                .build())
            .doOnSuccess(response -> log.info("Order cancelled: {}", response));
    }
    
    /**
     * Get order status via gRPC
     */
    public Mono<OrderResponse> getOrderStatus(String orderId) {
        log.debug("Getting order status for: {}", orderId);
        
        return grpcClient.getOrderStatus(orderId)
            .map(grpcResponse -> OrderResponse.builder()
                .success(true)
                .orderId(grpcResponse.getOrderId())
                .message("Filled: " + grpcResponse.getFilledQuantity() + " @ " + grpcResponse.getAveragePrice())
                .status(grpcResponse.getStatus().name())
                .build());
    }
    
    /**
     * Get balance from gRPC or DragonflyDB
     */
    public Mono<BalanceResponse> getBalance(String currency) {
        log.debug("Getting balance for currency: {}", currency);
        
        // Try gRPC first, fallback to DragonflyDB
        return grpcClient.getBalance(currency)
            .onErrorResume(error -> {
                log.warn("gRPC balance call failed, falling back to DragonflyDB", error);
                return realtimeDataService.getBalance(currency)
                    .map(balance -> BalanceResponse.newBuilder()
                        .setAvailable(balance)
                        .setLocked(0.0)
                        .setTotal(balance)
                        .build());
            });
    }
    
    /**
     * Get order history from TimescaleDB
     */
    public Flux<Order> getOrderHistory(int limit) {
        log.debug("Fetching order history, limit: {}", limit);
        return orderRepository.findRecentOrders(limit);
    }
    
    /**
     * Get orders by status
     */
    public Flux<Order> getOrdersByStatus(String status, int limit) {
        log.debug("Fetching orders with status: {}, limit: {}", status, limit);
        return orderRepository.findByStatus(status, limit);
    }
    
    /**
     * Get orders within time range
     */
    public Flux<Order> getOrdersByTimeRange(Instant start, Instant end) {
        log.debug("Fetching orders from {} to {}", start, end);
        return orderRepository.findByTimeRange(start, end);
    }
}
