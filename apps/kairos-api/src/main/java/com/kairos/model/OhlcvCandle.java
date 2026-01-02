package com.kairos.model;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.time.Instant;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class OhlcvCandle {
    
    private Instant bucket;
    private String symbol;
    private String exchange;
    private Double open;
    private Double high;
    private Double low;
    private Double close;
    private Double volume;
}
