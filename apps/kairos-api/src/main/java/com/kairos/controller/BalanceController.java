package com.kairos.controller;

import com.kairos.service.TradingService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;
import trading_engine.BalanceResponse;

@Slf4j
@RestController
@RequestMapping("/api/balance")
@RequiredArgsConstructor
public class BalanceController {
    
    private final TradingService tradingService;
    
    /**
     * GET /api/balance/{currency}
     * Get balance for a specific currency
     */
    @GetMapping("/{currency}")
    public Mono<BalanceDto> getBalance(@PathVariable String currency) {
        log.info("GET /api/balance/{}", currency);
        return tradingService.getBalance(currency.toUpperCase())
            .map(grpcResponse -> new BalanceDto(
                currency.toUpperCase(),
                grpcResponse.getAvailable(),
                grpcResponse.getLocked(),
                grpcResponse.getTotal()
            ));
    }
    
    // Inner class for balance response
    record BalanceDto(String currency, double available, double locked, double total) {}
}
