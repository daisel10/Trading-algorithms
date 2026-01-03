package com.kairos.controller;

import com.kairos.model.Order;
import com.kairos.model.dto.OrderResponse;
import com.kairos.model.dto.PlaceOrderRequest;
import com.kairos.service.TradingService;
import jakarta.validation.Valid;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.format.annotation.DateTimeFormat;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.Instant;

@Slf4j
@RestController
@RequestMapping("/api/orders")
@RequiredArgsConstructor
public class OrderController {

    private final TradingService tradingService;

    /**
     * POST /api/orders
     * Place a new order (stores to database)
     */
    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public Mono<OrderResponse> placeOrder(@Valid @RequestBody PlaceOrderRequest request) {
        log.info("POST /api/orders - request: {}", request);
        return tradingService.placeOrder(request);
    }

    /**
     * DELETE /api/orders/{orderId}
     * Cancel an existing order
     */
    @DeleteMapping("/{orderId}")
    public Mono<OrderResponse> cancelOrder(@PathVariable String orderId) {
        log.info("DELETE /api/orders/{}", orderId);
        return tradingService.cancelOrder(orderId);
    }

    /**
     * GET /api/orders/{orderId}/status
     * Get order status from database
     */
    @GetMapping("/{orderId}/status")
    public Mono<OrderResponse> getOrderStatus(@PathVariable String orderId) {
        log.info("GET /api/orders/{}/status", orderId);
        return tradingService.getOrderStatus(orderId);
    }

    /**
     * GET /api/orders/history
     * Get order history from TimescaleDB
     */
    @GetMapping("/history")
    public Flux<Order> getOrderHistory(
            @RequestParam(defaultValue = "50") int limit) {
        log.info("GET /api/orders/history - limit: {}", limit);
        return tradingService.getOrderHistory(limit);
    }

    /**
     * GET /api/orders/history/range
     * Get orders within a time range
     */
    @GetMapping("/history/range")
    public Flux<Order> getOrdersByTimeRange(
            @RequestParam @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME) Instant start,
            @RequestParam @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME) Instant end) {
        log.info("GET /api/orders/history/range - start: {}, end: {}", start, end);
        return tradingService.getOrdersByTimeRange(start, end);
    }

    /**
     * GET /api/orders/status/{status}
     * Get orders by status
     */
    @GetMapping("/status/{status}")
    public Flux<Order> getOrdersByStatus(
            @PathVariable String status,
            @RequestParam(defaultValue = "50") int limit) {
        log.info("GET /api/orders/status/{} - limit: {}", status, limit);
        return tradingService.getOrdersByStatus(status.toUpperCase(), limit);
    }
}
