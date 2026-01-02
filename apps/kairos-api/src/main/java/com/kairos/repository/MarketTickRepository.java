package com.kairos.repository;

import com.kairos.model.MarketTick;
import org.springframework.data.r2dbc.repository.Query;
import org.springframework.data.r2dbc.repository.R2dbcRepository;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Flux;

import java.time.Instant;

@Repository
public interface MarketTickRepository extends R2dbcRepository<MarketTick, Long> {
    
    /**
     * Get recent ticks for a symbol
     */
    @Query("SELECT * FROM market_ticks WHERE symbol = :symbol ORDER BY time DESC LIMIT :limit")
    Flux<MarketTick> findRecentBySymbol(String symbol, int limit);
    
    /**
     * Get ticks within a time range
     */
    @Query("SELECT * FROM market_ticks WHERE symbol = :symbol AND time BETWEEN :start AND :end ORDER BY time DESC")
    Flux<MarketTick> findBySymbolAndTimeRange(String symbol, Instant start, Instant end);
    
    /**
     * Get ticks for a symbol and exchange
     */
    @Query("SELECT * FROM market_ticks WHERE symbol = :symbol AND exchange = :exchange ORDER BY time DESC LIMIT :limit")
    Flux<MarketTick> findBySymbolAndExchange(String symbol, String exchange, int limit);
}
