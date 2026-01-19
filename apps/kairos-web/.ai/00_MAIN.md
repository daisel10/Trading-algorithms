# kairos-web: Configuración de Agentes

## 📘 Filosofía de Uso

**Este documento define el contexto del Dashboard KAIRÓS (Angular) para agentes de IA.**
Consulta la documentación de referencia según la tarea que vayas a realizar.

---

## 🛠 Habilidades Disponibles (Skillset)

Habilidades específicas de kairos-web (Dashboard en Angular).

### 🌐 Habilidades Tecnológicas (Angular/TypeScript Stack)

*Patrones técnicos específicos del frontend.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `angular-components` | Componentes reutilizables, standalone components | `docs/COMPONENTS.md` (crear si necesario) |
| `rxjs-patterns` | Observables, operators, state management | `docs/RXJS.md` (crear si necesario) |
| `angular-forms` | Formularios reactivos, validación | `docs/FORMS.md` (crear si necesario) |
| `websocket-client` | Cliente WebSocket para streaming en tiempo real | `docs/WEBSOCKET_CLIENT.md` (crear si necesario) |
| `chart-integration` | Integración de gráficos (Chart.js, ApexCharts) | `docs/CHARTS.md` (crear si necesario) |

> **Nota**: Los archivos de habilidades en `docs/` pueden crearse según necesidad siguiendo el patrón de kairos-core.

---

## 🤖 Disparadores Automáticos (Auto-invoke)

**REGLA DE ORO:** Antes de realizar una acción, carga la habilidad correspondiente.

### 🏗 Desarrollo & Arquitectura

| Acción (Lo que vas a hacer) | Habilidad Requerida (Lo que debes leer antes) |
| :--- | :--- |
| Crear nuevos componentes Angular | `angular-components` |
| Implementar estado reactivo | `rxjs-patterns` |
| Crear formularios | `angular-forms` |
| Conectar a WebSocket | `websocket-client` |
| Añadir gráficos de precios | `chart-integration` |
| Evitar memory leaks | `rxjs-patterns` (⚠️ CRÍTICO - usar AsyncPipe) |

### 🧪 Calidad & Testing

| Acción | Habilidad Requerida |
| :--- | :--- |
| Escribir tests unitarios | Testing patterns (Jasmine + Karma) |
| Verificar subscriptions | Code review con `grep .subscribe(` |
| Optimizar performance | OnPush change detection |

---

## 🗺 Visión General del Proyecto

**kairos-web** es el dashboard interactivo del sistema KAIRÓS, construido con Angular 21 y TypeScript.

### Propósito

Aplicación web que:

- Visualiza **precios en tiempo real** (WebSocket)
- Muestra **gráficos OHLCV** y métricas
- Permite **crear y gestionar órdenes**
- Configura **estrategias** de trading
- Monitorea **balances** y posiciones

### Stack Tecnológico

| Categoría | Tecnología | Versión |
| :--- | :--- | :--- |
| **Framework** | Angular | 21.0.0 |
| **Lenguaje** | TypeScript | ~5.0.0 |
| **Runtime** | Node.js | 18+ |
| **State** | RxJS | ~7.8.0 |
| **Testing** | Jasmine + Karma | ~5.1 + ~6.4 |
| **Build Tool** | Angular CLI | 21.0.0 |

### Arquitectura Angular

```
core/         → Servicios singleton (API, WebSocket, Auth)
features/     → Módulos de dominio (dashboard, settings, orders)
shared/       → Componentes UI reutilizables (charts, tables)
environments/ → Configuración por entorno (dev, prod)
```

### Estructura de Directorios

```
apps/kairos-web/
├── angular.json
├── package.json
├── tsconfig.json
└── src/
    ├── main.ts                    # Entry point
    ├── index.html
    ├── styles.css
    ├── environments/              # Environment configs
    ├── app/
    │   ├── app.component.ts       # Root component
    │   ├── app.routes.ts          # Routing config
    │   ├── core/                  # Singleton services
    │   │   └── services/
    │   │       ├── api.service.ts         # HTTP client
    │   │       ├── websocket.service.ts   # WebSocket
    │   │       └── auth.service.ts        # Auth
    │   ├── features/              # Feature modules
    │   │   ├── dashboard/         # Main view
    │   │   ├── settings/          # Configuration
    │   │   └── orders/            # Order management
    │   └── shared/                # Shared components
    │       ├── components/        # Reusable UI
    │       └── models/            # TypeScript interfaces
    └── assets/                    # Static files
```

---

## ⚡ Flujo de Trabajo (Workflow)

### Desarrollo Local

```bash
# Instalar dependencias
npm install

# Dev server
ng serve
# Abre http://localhost:4200

# Dev server con proxy API
ng serve --proxy-config proxy.conf.json
```

### Testing

```bash
# Tests unitarios
ng test

# Tests con coverage
ng test --code-coverage

# E2E tests (si configurado)
ng e2e
```

### Linting & Build

```bash
# Linter
ng lint

# Build de producción
ng build --configuration production

# Analizar bundle size
ng build --stats-json
npm run webpack-bundle-analyzer
```

---

## 📋 Flujo de Datos

```
┌────────────────────┐
│   main.ts          │ → Bootstraps AppComponent
└──────────┬─────────┘
           ↓
┌────────────────────┐
│  Router            │ → Lazy loads feature modules
└──────────┬─────────┘
           ↓
┌────────────────────┐
│  Component         │ → Usa Service
└──────────┬─────────┘
           ├─→ HTTP (ApiService) → kairos-api
           └─→ WebSocket (WebSocketService) → kairos-api
```

---

## 📝 Convenciones de Código

### TypeScript

- **Interfaces**: PascalCase con `I` prefix `IMarketData`
- **Components**: PascalCase `PriceChartComponent`
- **Services**: PascalCase `MarketDataService`
- **Variables**: camelCase `latestPrice`
- **Constants**: SCREAMING_SNAKE_CASE `API_URL`

### Angular Patterns

```typescript
// ✅ CORRECTO: Servicio con Observable
@Injectable({ providedIn: 'root' })
export class MarketDataService {
  getLatestPrice(symbol: string): Observable<MarketData> {
    return this.http.get<MarketData>(`${API_URL}/latest/${symbol}`)
      .pipe(catchError(this.handleError));
  }
}

// ✅ CORRECTO: Componente con OnPush + AsyncPipe
@Component({
  selector: 'app-price-chart',
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `<div>{{ prices$ | async }}</div>`
})
export class PriceChartComponent {
  prices$ = this.marketData.getPriceStream('btcusdt');
  constructor(private marketData: MarketDataService) {}
}

// ❌ INCORRECTO: Subscribe sin unsubscribe
ngOnInit() {
  this.service.getData().subscribe(data => {
    this.data = data; // Memory leak!
  });
}
```

---

## 🔗 Comunicación

- **kairos-web → kairos-api**: REST (HttpClient)
- **kairos-web → kairos-api**: WebSocket (rxjs/webSocket)

### Environment Config

```typescript
// environment.ts
export const environment = {
  production: false,
  apiUrl: 'http://localhost:8080/api',
  wsUrl: 'ws://localhost:8080/ws'
};
```

---

## 📋 Guía deCommits

**Formato**: `<type>(<scope>): <description>`

**Ejemplos**:

- `feat(dashboard): add real-time price chart`
- `fix(websocket): resolve reconnection issue`
- `perf(chart): optimize rendering with OnPush`

---

**Última actualización:** 2026-01-19  
**Mantenido por:** kairos-web Development Team
