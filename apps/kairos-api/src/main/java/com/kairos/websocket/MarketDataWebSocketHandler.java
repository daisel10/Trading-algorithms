package com.kairos.websocket;

import com.kairos.service.RealtimeDataService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.springframework.web.reactive.socket.WebSocketHandler;
import org.springframework.web.reactive.socket.WebSocketMessage;
import org.springframework.web.reactive.socket.WebSocketSession;
import reactor.core.publisher.Mono;

@Slf4j
@Component
@RequiredArgsConstructor
public class MarketDataWebSocketHandler implements WebSocketHandler {
    
    private final RealtimeDataService realtimeDataService;
    
    @Override
    public Mono<Void> handle(WebSocketSession session) {
        log.info("WebSocket connection established: sessionId={}", session.getId());
        
        // Subscribe to DragonflyDB Pub/Sub and stream to WebSocket
        return session.send(
            realtimeDataService.subscribeToMarketData()
                .doOnNext(data -> log.debug("Sending market data to WebSocket: {}", data))
                .map(session::textMessage)
                .doOnError(error -> log.error("Error in WebSocket market data stream", error))
        )
        .doOnTerminate(() -> log.info("WebSocket connection closed: sessionId={}", session.getId()));
    }
}
