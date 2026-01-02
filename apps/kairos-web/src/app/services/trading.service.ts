import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface PlaceOrderRequest {
    symbol: string;
    side: 'BUY' | 'SELL';
    orderType: 'MARKET' | 'LIMIT';
    quantity: number;
    price?: number;
}

export interface OrderResponse {
    success: boolean;
    orderId: string;
    message: string;
    status: string;
}

export interface Order {
    id: string;
    createdAt: string;
    symbol: string;
    side: string;
    orderType: string;
    quantity: number;
    price: number | null;
    status: string;
    executedAt: string | null;
    exchange: string;
}

@Injectable({
    providedIn: 'root'
})
export class TradingService {
    private readonly apiUrl = `${environment.apiUrl}/api/orders`;

    constructor(private http: HttpClient) { }

    /**
     * Place a new order
     */
    placeOrder(request: PlaceOrderRequest): Observable<OrderResponse> {
        return this.http.post<OrderResponse>(this.apiUrl, request);
    }

    /**
     * Cancel an existing order
     */
    cancelOrder(orderId: string): Observable<OrderResponse> {
        return this.http.delete<OrderResponse>(`${this.apiUrl}/${orderId}`);
    }

    /**
     * Get order status
     */
    getOrderStatus(orderId: string): Observable<OrderResponse> {
        return this.http.get<OrderResponse>(`${this.apiUrl}/${orderId}/status`);
    }

    /**
     * Get order history
     */
    getOrderHistory(limit: number = 50): Observable<Order[]> {
        const params = new HttpParams().set('limit', limit.toString());
        return this.http.get<Order[]>(`${this.apiUrl}/history`, { params });
    }

    /**
     * Get orders by time range
     */
    getOrdersByTimeRange(start: Date, end: Date): Observable<Order[]> {
        const params = new HttpParams()
            .set('start', start.toISOString())
            .set('end', end.toISOString());
        return this.http.get<Order[]>(`${this.apiUrl}/history/range`, { params });
    }

    /**
     * Get orders by status
     */
    getOrdersByStatus(status: string, limit: number = 50): Observable<Order[]> {
        const params = new HttpParams().set('limit', limit.toString());
        return this.http.get<Order[]>(`${this.apiUrl}/status/${status}`, { params });
    }
}
