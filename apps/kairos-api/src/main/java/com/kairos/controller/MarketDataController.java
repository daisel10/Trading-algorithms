package com.kairos.controller;

import com.kairos.model.MarketTick;
import com.kairos.model.OhlcvCandle;
import com.kairos.service.MarketDataService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.format.annotation.DateTimeFormat;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.time.Instant;

@Slf4j
@RestController
@RequestMapping("/api/market-data")
@RequiredArgsConstructor
public class MarketDataController {
    
    private final MarketDataService marketDataService;
    
    /**
     * GET /api/market-data/ticks/{symbol}
     * Get recent market ticks for a symbol
     */
    @GetMapping("/ticks/{symbol}")
    public Flux<MarketTick> getRecentTicks(
            @PathVariable String symbol,
            @RequestParam(defaultValue = "100") int limit) {
        log.info("GET /api/market-data/ticks/{} - limit: {}", symbol, limit);
        return marketDataService.getRecentTicks(symbol, limit);
    }
    
    /**
     * GET /api/market-data/ticks/{symbol}/range
     * Get historical ticks within a time range
     */
    @GetMapping("/ticks/{symbol}/range")
    public Flux<MarketTick> getHistoricalTicks(
            @PathVariable String symbol,
            @RequestParam @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME) Instant start,
            @RequestParam @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME) Instant end) {
        log.info("GET /api/market-data/ticks/{}/range - start: {}, end: {}", symbol, start, end);
        return marketDataService.getHistoricalTicks(symbol, start, end);
    }
    
    /**
     * GET /api/market-data/ohlcv/{symbol}
     * Get OHLCV candles from materialized view
     */
    @GetMapping("/ohlcv/{symbol}")
    public Flux<OhlcvCandle> getOhlcvCandles(
            @PathVariable String symbol,
            @RequestParam(required = false) @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME) Instant start,
            @RequestParam(required = false) @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME) Instant end,
            @RequestParam(defaultValue = "100") int limit) {
        log.info("GET /api/market-data/ohlcv/{} - start: {}, end: {}, limit: {}", symbol, start, end, limit);
        
        if (start != null && end != null) {
            return marketDataService.getOhlcvCandles(symbol, start, end);
        } else {
            return marketDataService.getLatestOhlcv(symbol, limit);
        }
    }
    
    /**
     * GET /api/market-data/latest/{symbol}
     * Get latest price from DragonflyDB (real-time)
     */
    @GetMapping("/latest/{symbol}")
    public Mono<PriceResponse> getLatestPrice(@PathVariable String symbol) {
        log.info("GET /api/market-data/latest/{}", symbol);
        return marketDataService.getLatestPrice(symbol)
            .map(price -> new PriceResponse(symbol, price, Instant.now()))
            .defaultIfEmpty(new PriceResponse(symbol, null, Instant.now()));
    }
    
    // Inner class for price response
    record PriceResponse(String symbol, Double price, Instant timestamp) {}
}
