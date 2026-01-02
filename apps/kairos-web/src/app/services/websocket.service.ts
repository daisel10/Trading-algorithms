import { Injectable, OnDestroy } from '@angular/core';
import { Observable, Subject } from 'rxjs';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { environment } from '../../environments/environment';

export interface MarketDataMessage {
    type: string;
    symbol?: string;
    price?: number;
    volume?: number;
    timestamp?: string;
    [key: string]: any;
}

@Injectable({
    providedIn: 'root'
})
export class WebSocketService implements OnDestroy {
    private socket$: WebSocketSubject<MarketDataMessage> | null = null;
    private messagesSubject$ = new Subject<MarketDataMessage>();
    public messages$ = this.messagesSubject$.asObservable();

    private readonly wsUrl = `${environment.wsUrl}/ws/market-data`;

    constructor() { }

    /**
     * Connect to the WebSocket server
     */
    connect(): void {
        if (!this.socket$ || this.socket$.closed) {
            this.socket$ = webSocket<MarketDataMessage>({
                url: this.wsUrl,
                openObserver: {
                    next: () => {
                        console.log('[WebSocket] Connected to market data stream');
                    }
                },
                closeObserver: {
                    next: () => {
                        console.log('[WebSocket] Disconnected from market data stream');
                    }
                }
            });

            this.socket$.subscribe({
                next: (message) => this.messagesSubject$.next(message),
                error: (error) => {
                    console.error('[WebSocket] Error:', error);
                    // Auto-reconnect after 5 seconds
                    setTimeout(() => this.connect(), 5000);
                }
            });
        }
    }

    /**
     * Send a message to the server (if needed)
     */
    sendMessage(message: MarketDataMessage): void {
        if (this.socket$) {
            this.socket$.next(message);
        } else {
            console.error('[WebSocket] Not connected. Call connect() first.');
        }
    }

    /**
     * Disconnect from WebSocket
     */
    disconnect(): void {
        if (this.socket$) {
            this.socket$.complete();
            this.socket$ = null;
        }
    }

    ngOnDestroy(): void {
        this.disconnect();
    }
}
