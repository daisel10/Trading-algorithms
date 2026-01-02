package com.kairos.model;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Column;
import org.springframework.data.relational.core.mapping.Table;

import java.time.Instant;
import java.util.UUID;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@Table("orders")
public class Order {
    
    @Id
    @Column("id")
    private UUID id;
    
    @Column("created_at")
    private Instant createdAt;
    
    @Column("symbol")
    private String symbol;
    
    @Column("side")
    private String side; // BUY or SELL
    
    @Column("order_type")
    private String orderType; // MARKET or LIMIT
    
    @Column("quantity")
    private Double quantity;
    
    @Column("price")
    private Double price;
    
    @Column("status")
    private String status; // PENDING, APPROVED, REJECTED, EXECUTED, CANCELLED
    
    @Column("executed_at")
    private Instant executedAt;
    
    @Column("exchange")
    private String exchange;
}
