package com.kairos.controller;

import com.kairos.service.TradingService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

@Slf4j
@RestController
@RequestMapping("/api/balance")
@RequiredArgsConstructor
public class BalanceController {

    private final TradingService tradingService;

    /**
     * Get balance for a specific currency
     * GET /api/balance/{currency}
     */
    @GetMapping("/{currency}")
    public Mono<BalanceResponse> getBalance(@PathVariable String currency) {
        log.debug("Getting balance for currency: {}", currency);

        return tradingService.getBalance(currency)
                .map(balance -> new BalanceResponse(currency, balance, 0.0, balance))
                .defaultIfEmpty(new BalanceResponse(currency, 0.0, 0.0, 0.0));
    }

    // Inner DTO class
    public record BalanceResponse(
            String currency,
            double available,
            double locked,
            double total) {
    }
}
