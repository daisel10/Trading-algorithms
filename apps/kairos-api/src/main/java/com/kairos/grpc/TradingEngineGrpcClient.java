package com.kairos.grpc;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import lombok.extern.slf4j.Slf4j;
import net.devh.boot.grpc.client.inject.GrpcClient;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;
import trading_engine.*;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;

@Slf4j
@Service
public class TradingEngineGrpcClient {
    
    @GrpcClient("kairos-core")
    private TradingEngineGrpc.TradingEngineBlockingStub tradingEngineStub;
    
    /**
     * Place an order via gRPC to kairos-core
     */
    public Mono<OrderResponse> placeOrder(String symbol, OrderSide side, OrderType orderType, 
                                          double quantity, Double price) {
        return Mono.fromCallable(() -> {
            log.debug("Placing order via gRPC: symbol={}, side={}, quantity={}", symbol, side, quantity);
            
            OrderRequest.Builder requestBuilder = OrderRequest.newBuilder()
                .setSymbol(symbol)
                .setSide(side)
                .setOrderType(orderType)
                .setQuantity(quantity);
            
            if (price != null) {
                requestBuilder.setPrice(price);
            }
            
            OrderRequest request = requestBuilder.build();
            return tradingEngineStub.placeOrder(request);
        })
        .subscribeOn(Schedulers.boundedElastic())
        .doOnSuccess(response -> log.info("Order placed successfully: orderId={}, status={}", 
            response.getOrderId(), response.getStatus()))
        .doOnError(error -> log.error("Error placing order via gRPC", error));
    }
    
    /**
     * Cancel an order via gRPC
     */
    public Mono<OrderResponse> cancelOrder(String orderId) {
        return Mono.fromCallable(() -> {
            log.debug("Canceling order via gRPC: orderId={}", orderId);
            
            CancelOrderRequest request = CancelOrderRequest.newBuilder()
                .setOrderId(orderId)
                .build();
            
            return tradingEngineStub.cancelOrder(request);
        })
        .subscribeOn(Schedulers.boundedElastic())
        .doOnSuccess(response -> log.info("Order cancelled: orderId={}, status={}", 
            response.getOrderId(), response.getStatus()))
        .doOnError(error -> log.error("Error cancelling order via gRPC", error));
    }
    
    /**
     * Get balance via gRPC
     */
    public Mono<BalanceResponse> getBalance(String currency) {
        return Mono.fromCallable(() -> {
            log.debug("Getting balance via gRPC: currency={}", currency);
            
            BalanceRequest request = BalanceRequest.newBuilder()
                .setCurrency(currency)
                .build();
            
            return tradingEngineStub.getBalance(request);
        })
        .subscribeOn(Schedulers.boundedElastic())
        .doOnSuccess(response -> log.debug("Balance retrieved: available={}, locked={}", 
            response.getAvailable(), response.getLocked()))
        .doOnError(error -> log.error("Error getting balance via gRPC", error));
    }
    
    /**
     * Get order status via gRPC
     */
    public Mono<OrderStatusResponse> getOrderStatus(String orderId) {
        return Mono.fromCallable(() -> {
            log.debug("Getting order status via gRPC: orderId={}", orderId);
            
            OrderStatusRequest request = OrderStatusRequest.newBuilder()
                .setOrderId(orderId)
                .build();
            
            return tradingEngineStub.getOrderStatus(request);
        })
        .subscribeOn(Schedulers.boundedElastic())
        .doOnSuccess(response -> log.debug("Order status retrieved: orderId={}, status={}", 
            response.getOrderId(), response.getStatus()))
        .doOnError(error -> log.error("Error getting order status via gRPC", error));
    }
}
