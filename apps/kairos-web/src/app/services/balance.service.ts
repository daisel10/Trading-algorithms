import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Balance {
    currency: string;
    available: number;
    locked: number;
    total: number;
}

@Injectable({
    providedIn: 'root'
})
export class BalanceService {
    private readonly apiUrl = `${environment.apiUrl}/api/balance`;

    constructor(private http: HttpClient) { }

    /**
     * Get balance for a specific currency
     */
    getBalance(currency: string): Observable<Balance> {
        return this.http.get<Balance>(`${this.apiUrl}/${currency}`);
    }
}
