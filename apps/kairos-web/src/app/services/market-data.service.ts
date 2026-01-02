import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface MarketTick {
  time: string;
  symbol: string;
  exchange: string;
  price: number;
  volume: number;
}

export interface OhlcvCandle {
  bucket: string;
  symbol: string;
  exchange: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface PriceResponse {
  symbol: string;
  price: number | null;
  timestamp: string;
}

@Injectable({
  providedIn: 'root'
})
export class MarketDataService {
  private readonly apiUrl = `${environment.apiUrl}/api/market-data`;

  constructor(private http: HttpClient) {}

  /**
   * Get recent market ticks for a symbol
   */
  getRecentTicks(symbol: string, limit: number = 100): Observable<MarketTick[]> {
    const params = new HttpParams().set('limit', limit.toString());
    return this.http.get<MarketTick[]>(`${this.apiUrl}/ticks/${symbol}`, { params });
  }

  /**
   * Get historical ticks within a time range
   */
  getHistoricalTicks(symbol: string, start: Date, end: Date): Observable<MarketTick[]> {
    const params = new HttpParams()
      .set('start', start.toISOString())
      .set('end', end.toISOString());
    return this.http.get<MarketTick[]>(`${this.apiUrl}/ticks/${symbol}/range`, { params });
  }

  /**
   * Get OHLCV candles
   */
  getOhlcvCandles(symbol: string, start?: Date, end?: Date, limit: number = 100): Observable<OhlcvCandle[]> {
    let params = new HttpParams();
    
    if (start && end) {
      params = params.set('start', start.toISOString()).set('end', end.toISOString());
    } else {
      params = params.set('limit', limit.toString());
    }
    
    return this.http.get<OhlcvCandle[]>(`${this.apiUrl}/ohlcv/${symbol}`, { params });
  }

  /**
   * Get latest price from DragonflyDB (real-time)
   */
  getLatestPrice(symbol: string): Observable<PriceResponse> {
    return this.http.get<PriceResponse>(`${this.apiUrl}/latest/${symbol}`);
  }
}
