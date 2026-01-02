package com.kairos.repository;

import com.kairos.model.OhlcvCandle;
import io.r2dbc.spi.Row;
import lombok.RequiredArgsConstructor;
import org.springframework.r2dbc.core.DatabaseClient;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Flux;

import java.time.Instant;

@Repository
@RequiredArgsConstructor
public class OhlcvRepository {
    
    private final DatabaseClient databaseClient;
    
    /**
     * Get OHLCV candles from the materialized view
     */
    public Flux<OhlcvCandle> findOhlcvBySymbol(String symbol, Instant start, Instant end) {
        return databaseClient.sql(
            "SELECT bucket, symbol, exchange, open, high, low, close, volume " +
            "FROM ohlcv_1m " +
            "WHERE symbol = :symbol AND bucket BETWEEN :start AND :end " +
            "ORDER BY bucket DESC"
        )
        .bind("symbol", symbol)
        .bind("start", start)
        .bind("end", end)
        .map(this::mapToOhlcvCandle)
        .all();
    }
    
    /**
     * Get latest OHLCV candles
     */
    public Flux<OhlcvCandle> findLatestOhlcv(String symbol, int limit) {
        return databaseClient.sql(
            "SELECT bucket, symbol, exchange, open, high, low, close, volume " +
            "FROM ohlcv_1m " +
            "WHERE symbol = :symbol " +
            "ORDER BY bucket DESC " +
            "LIMIT :limit"
        )
        .bind("symbol", symbol)
        .bind("limit", limit)
        .map(this::mapToOhlcvCandle)
        .all();
    }
    
    private OhlcvCandle mapToOhlcvCandle(Row row) {
        return OhlcvCandle.builder()
            .bucket(row.get("bucket", Instant.class))
            .symbol(row.get("symbol", String.class))
            .exchange(row.get("exchange", String.class))
            .open(row.get("open", Double.class))
            .high(row.get("high", Double.class))
            .low(row.get("low", Double.class))
            .close(row.get("close", Double.class))
            .volume(row.get("volume", Double.class))
            .build();
    }
}
