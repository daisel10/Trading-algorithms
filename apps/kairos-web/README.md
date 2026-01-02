# KAIRÓS Dashboard - Angular Application

Frontend dashboard for the KAIRÓS trading system.

## Technology Stack

- **Angular 21**
- **TypeScript 5.9**
- **RxJS 7.8**
- **WebSocket** for real-time data streaming

## Architecture

The dashboard communicates with the `kairos-api` (Java Spring Boot) via:

1. **REST API** - HTTP calls for queries and commands
2. **WebSocket** - Real-time market data streaming

### Services

Located in `src/app/services/`:

#### MarketDataService
- `getRecentTicks(symbol, limit)` - Get recent market ticks
- `getHistoricalTicks(symbol, start, end)` - Historical ticks
- `getOhlcvCandles(symbol, start?, end?, limit)` - OHLCV candles
- `getLatestPrice(symbol)` - Latest price from DragonflyDB

#### TradingService
- `placeOrder(request)` - Place a new order
- `cancelOrder(orderId)` - Cancel an order
- `getOrderStatus(orderId)` - Get order status
- `getOrderHistory(limit)` - Order history
- `getOrdersByTimeRange(start, end)` - Orders by date range
- `getOrdersByStatus(status, limit)` - Orders by status

#### BalanceService
- `getBalance(currency)` - Get balance for a currency

#### WebSocketService
- `connect()` - Connect to market data stream
- `messages$` - Observable stream of market data
- `disconnect()` - Disconnect from stream

## Environment Configuration

### Development (`src/environments/environment.ts`)
```typescript
{
  apiUrl: 'http://localhost:4000',
  wsUrl: 'ws://localhost:4000'
}
```

### Production (`src/environments/environment.prod.ts`)
```typescript
{
  apiUrl: 'http://kairos-api:4000',
  wsUrl: 'ws://kairos-api:4000'
}
```

## Development

```bash
# Install dependencies
npm install

# Run development server
npm start

# Navigate to http://localhost:4200
```

## Build

```bash
# Production build
npm run build

# Output: dist/kairos-web
```

## Docker

See `Dockerfile` in this directory and `infrastructure/docker-compose.yml` for container deployment.

## Migration from GraphQL

Previous implementation (planned but not built) would have used GraphQL. This implementation uses:

- **REST API** for queries and mutations
- **WebSocket** for real-time subscriptions

This provides better compatibility with standard HTTP tooling and clearer API contracts.

## Example Usage

### Component Example

```typescript
import { Component, OnInit, OnDestroy } from '@angular/core';
import { MarketDataService } from './services/market-data.service';
import { WebSocketService } from './services/websocket.service';
import { TradingService } from './services/trading.service';

export class DashboardComponent implements OnInit, OnDestroy {
  constructor(
    private marketData: MarketDataService,
    private ws: WebSocketService,
    private trading: TradingService
  ) {}

  ngOnInit() {
    // Get latest price
    this.marketData.getLatestPrice('BTC-USDT').subscribe(price => {
      console.log('Latest BTC price:', price.price);
    });

    // Connect to WebSocket for real-time updates
    this.ws.connect();
    this.ws.messages$.subscribe(message => {
      console.log('Real-time update:', message);
    });

    // Place an order
    this.trading.placeOrder({
      symbol: 'BTC-USDT',
      side: 'BUY',
      orderType: 'MARKET',
      quantity: 0.01
    }).subscribe(response => {
      console.log('Order placed:', response);
    });
  }

  ngOnDestroy() {
    this.ws.disconnect();
  }
}
```
