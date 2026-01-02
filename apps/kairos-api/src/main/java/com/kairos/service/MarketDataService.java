package com.kairos.service;

import com.kairos.model.MarketTick;
import com.kairos.model.OhlcvCandle;
import com.kairos.repository.MarketTickRepository;
import com.kairos.repository.OhlcvRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;

import java.time.Instant;

@Slf4j
@Service
@RequiredArgsConstructor
public class MarketDataService {
    
    private final MarketTickRepository marketTickRepository;
    private final OhlcvRepository ohlcvRepository;
    private final RealtimeDataService realtimeDataService;
    
    /**
     * Get recent market ticks from TimescaleDB
     */
    public Flux<MarketTick> getRecentTicks(String symbol, int limit) {
        log.debug("Fetching recent {} ticks for symbol: {}", limit, symbol);
        return marketTickRepository.findRecentBySymbol(symbol, limit)
            .doOnComplete(() -> log.debug("Fetched recent ticks for {}", symbol));
    }
    
    /**
     * Get historical market ticks within time range
     */
    public Flux<MarketTick> getHistoricalTicks(String symbol, Instant start, Instant end) {
        log.debug("Fetching historical ticks for {} from {} to {}", symbol, start, end);
        return marketTickRepository.findBySymbolAndTimeRange(symbol, start, end)
            .doOnComplete(() -> log.debug("Fetched historical ticks for {}", symbol));
    }
    
    /**
     * Get OHLCV candles from materialized view
     */
    public Flux<OhlcvCandle> getOhlcvCandles(String symbol, Instant start, Instant end) {
        log.debug("Fetching OHLCV candles for {} from {} to {}", symbol, start, end);
        return ohlcvRepository.findOhlcvBySymbol(symbol, start, end)
            .doOnComplete(() -> log.debug("Fetched OHLCV candles for {}", symbol));
    }
    
    /**
     * Get latest OHLCV candles
     */
    public Flux<OhlcvCandle> getLatestOhlcv(String symbol, int limit) {
        log.debug("Fetching latest {} OHLCV candles for {}", limit, symbol);
        return ohlcvRepository.findLatestOhlcv(symbol, limit)
            .doOnComplete(() -> log.debug("Fetched latest OHLCV for {}", symbol));
    }
    
    /**
     * Get latest price from DragonflyDB (real-time)
     */
    public reactor.core.publisher.Mono<Double> getLatestPrice(String symbol) {
        log.debug("Fetching latest price for {} from DragonflyDB", symbol);
        return realtimeDataService.getLatestPrice(symbol);
    }
}
