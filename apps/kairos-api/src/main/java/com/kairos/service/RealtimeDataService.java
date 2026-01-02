package com.kairos.service;

import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.redis.core.ReactiveRedisTemplate;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;

@Slf4j
@Service
@RequiredArgsConstructor
public class RealtimeDataService {
    
    private final ReactiveRedisTemplate<String, String> redisTemplate;
    
    @Value("${kairos.api.websocket.market-data-channel:market_data}")
    private String marketDataChannel;
    
    /**
     * Get balance from DragonflyDB (hot data)
     */
    public Mono<Double> getBalance(String currency) {
        String key = "balance:" + currency;
        return redisTemplate.opsForValue().get(key)
            .map(Double::parseDouble)
            .defaultIfEmpty(0.0)
            .doOnSuccess(balance -> log.debug("Retrieved balance for {}: {}", currency, balance))
            .onErrorResume(error -> {
                log.error("Error retrieving balance from DragonflyDB for currency: {}", currency, error);
                return Mono.just(0.0);
            });
    }
    
    /**
     * Get latest price from DragonflyDB
     */
    public Mono<Double> getLatestPrice(String symbol) {
        String key = "price:" + symbol;
        return redisTemplate.opsForValue().get(key)
            .map(Double::parseDouble)
            .doOnSuccess(price -> log.debug("Retrieved latest price for {}: {}", symbol, price))
            .onErrorResume(error -> {
                log.error("Error retrieving price from DragonflyDB for symbol: {}", symbol, error);
                return Mono.empty();
            });
    }
    
    /**
     * Subscribe to market data updates from DragonflyDB Pub/Sub
     */
    public reactor.core.publisher.Flux<String> subscribeToMarketData() {
        return redisTemplate.listenToChannel(marketDataChannel)
            .map(message -> message.getMessage())
            .doOnSubscribe(sub -> log.info("Subscribed to market data channel: {}", marketDataChannel))
            .doOnNext(msg -> log.debug("Received market data: {}", msg))
            .doOnError(error -> log.error("Error in market data subscription", error));
    }
    
    /**
     * Publish market data (if needed for testing or internal use)
     */
    public Mono<Long> publishMarketData(String data) {
        return redisTemplate.convertAndSend(marketDataChannel, data)
            .doOnSuccess(count -> log.debug("Published market data to {} subscribers", count));
    }
    
    /**
     * Set a value in DragonflyDB
     */
    public Mono<Boolean> setValue(String key, String value) {
        return redisTemplate.opsForValue().set(key, value)
            .doOnSuccess(success -> log.debug("Set key {} = {}: {}", key, value, success));
    }
    
    /**
     * Get a value from DragonflyDB
     */
    public Mono<String> getValue(String key) {
        return redisTemplate.opsForValue().get(key)
            .doOnSuccess(value -> log.debug("Got key {}: {}", key, value));
    }
}
