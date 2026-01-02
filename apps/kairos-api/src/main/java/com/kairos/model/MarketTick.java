package com.kairos.model;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Column;
import org.springframework.data.relational.core.mapping.Table;

import java.time.Instant;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
@Table("market_ticks")
public class MarketTick {
    
    @Column("time")
    private Instant time;
    
    @Column("symbol")
    private String symbol;
    
    @Column("exchange")
    private String exchange;
    
    @Column("price")
    private Double price;
    
    @Column("volume")
    private Double volume;
}
