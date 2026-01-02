package com.kairos.repository;

import com.kairos.model.Order;
import org.springframework.data.r2dbc.repository.Query;
import org.springframework.data.r2dbc.repository.R2dbcRepository;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.Instant;
import java.util.UUID;

@Repository
public interface OrderRepository extends R2dbcRepository<Order, UUID> {
    
    /**
     * Find order by ID
     */
    @Query("SELECT * FROM orders WHERE id = :orderId")
    Mono<Order> findByOrderId(UUID orderId);
    
    /**
     * Get recent orders
     */
    @Query("SELECT * FROM orders ORDER BY created_at DESC LIMIT :limit")
    Flux<Order> findRecentOrders(int limit);
    
    /**
     * Get orders by status
     */
    @Query("SELECT * FROM orders WHERE status = :status ORDER BY created_at DESC LIMIT :limit")
    Flux<Order> findByStatus(String status, int limit);
    
    /**
     * Get orders within a time range
     */
    @Query("SELECT * FROM orders WHERE created_at BETWEEN :start AND :end ORDER BY created_at DESC")
    Flux<Order> findByTimeRange(Instant start, Instant end);
    
    /**
     * Get orders by symbol
     */
    @Query("SELECT * FROM orders WHERE symbol = :symbol ORDER BY created_at DESC LIMIT :limit")
    Flux<Order> findBySymbol(String symbol, int limit);
}
