package com.kairos.service;

import com.kairos.model.Order;
import com.kairos.model.dto.OrderResponse;
import com.kairos.model.dto.PlaceOrderRequest;
import com.kairos.repository.OrderRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.Instant;
import java.util.UUID;

@Slf4j
@Service
@RequiredArgsConstructor
public class TradingService {

    private final OrderRepository orderRepository;
    private final RealtimeDataService realtimeDataService;

    /**
     * Place an order (simplified without gRPC - stores to database only)
     */
    public Mono<OrderResponse> placeOrder(PlaceOrderRequest request) {
        log.info("Placing order: {}", request);

        // Create order entity
        Order order = new Order();
        order.setId(UUID.randomUUID());
        order.setCreatedAt(Instant.now());
        order.setSymbol(request.getSymbol());
        order.setSide(request.getSide());
        order.setOrderType(request.getOrderType());
        order.setQuantity(request.getQuantity());
        order.setPrice(request.getPrice());
        order.setStatus("PENDING");
        order.setExchange("SIMULATED");

        return orderRepository.save(order)
                .map(savedOrder -> OrderResponse.builder()
                        .success(true)
                        .orderId(savedOrder.getId().toString())
                        .message("Order placed successfully (simulated)")
                        .status("PENDING")
                        .build())
                .doOnSuccess(response -> log.info("Order placed: {}", response));
    }

    /**
     * Cancel an order (simplified without gRPC)
     */
    public Mono<OrderResponse> cancelOrder(String orderId) {
        log.info("Cancelling order: {}", orderId);

        return orderRepository.findById(UUID.fromString(orderId))
                .flatMap(order -> {
                    order.setStatus("CANCELLED");
                    return orderRepository.save(order);
                })
                .map(order -> OrderResponse.builder()
                        .success(true)
                        .orderId(order.getId().toString())
                        .message("Order cancelled successfully")
                        .status("CANCELLED")
                        .build())
                .switchIfEmpty(Mono.just(OrderResponse.builder()
                        .success(false)
                        .orderId(orderId)
                        .message("Order not found")
                        .status("NOT_FOUND")
                        .build()));
    }

    /**
     * Get order status (from database)
     */
    public Mono<OrderResponse> getOrderStatus(String orderId) {
        log.debug("Getting order status for: {}", orderId);

        return orderRepository.findById(UUID.fromString(orderId))
                .map(order -> OrderResponse.builder()
                        .success(true)
                        .orderId(order.getId().toString())
                        .message("Order status: " + order.getStatus())
                        .status(order.getStatus())
                        .build())
                .switchIfEmpty(Mono.just(OrderResponse.builder()
                        .success(false)
                        .orderId(orderId)
                        .message("Order not found")
                        .status("NOT_FOUND")
                        .build()));
    }

    /**
     * Get balance from DragonflyDB (no gRPC)
     */
    public Mono<Double> getBalance(String currency) {
        log.debug("Getting balance for currency: {}", currency);
        return realtimeDataService.getBalance(currency);
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
