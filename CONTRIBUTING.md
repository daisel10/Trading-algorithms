# Contributing to KAIRÓS

¡Gracias por tu interés en contribuir al proyecto KAIRÓS! Este documento establece las normas de código, estilo, flujo de Git y pautas para colaborar efectivamente.

---

## 📋 Tabla de Contenidos

1. [Código de Conducta](#código-de-conducta)
2. [Cómo Empezar](#cómo-empezar)
3. [Flujo de Trabajo Git](#flujo-de-trabajo-git)
4. [Estándares de Código](#estándares-de-código)
5. [Estructura de Commits](#estructura-de-commits)
6. [Pull Requests](#pull-requests)
7. [Testing](#testing)
8. [Documentación](#documentación)
9. [Trabajando con IA](#trabajando-con-ia)

---

## 📜 Código de Conducta

- Mantén un tono profesional y respetuoso
- Enfócate en el código, no en las personas
- Acepta críticas constructivas de forma positiva
- Prioriza la calidad sobre la velocidad

---

## 🚀 Cómo Empezar

### 1. Configurar el Entorno

```bash
# Clonar el repositorio
git clone https://github.com/your-org/kairos-monorepo.git
cd kairos-monorepo

# Instalar Rust Nightly
rustup install nightly
rustup default nightly

# Instalar herramientas
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-clippy

# Instalar Java 21 y Maven 3.9+
# Instalar Node.js 18+ y npm

# Configurar variables de entorno
cp .env.example .env
# Editar .env con tus configuraciones locales
```

### 2. Verificar la Instalación

```bash
# Compilar todo el workspace
cargo build --workspace

# Ejecutar tests
cargo test --workspace

# Verificar la API Java
cd apps/kairos-api
mvn clean test

# Verificar el frontend
cd apps/kairos-web
npm install
npm test
```

### 3. Leer la Documentación

- [README.md](./README.md) - Visión general del proyecto
- [DEPLOYMENT.md](./DEPLOYMENT.md) - Guía de despliegue
- [GLOSSARY.md](./GLOSSARY.md) - Términos del dominio
- [docs/ADR/](./docs/ADR/) - Decisiones arquitectónicas

---

## 🌿 Flujo de Trabajo Git

### Branches

Usamos **Git Flow** con las siguientes ramas principales:

- `main` - Código en producción (protegida)
- `develop` - Rama de integración para desarrollo
- `feature/*` - Nuevas características
- `bugfix/*` - Corrección de bugs
- `hotfix/*` - Arreglos críticos en producción
- `release/*` - Preparación de releases

### Convención de Nombres de Branches

```bash
feature/arbitrage-bellman-ford-optimization
bugfix/grpc-connection-leak
hotfix/critical-redis-timeout
release/v0.2.0
```

### Workflow Estándar

```bash
# 1. Crear branch desde develop
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name

# 2. Hacer cambios y commits
# (Ver sección "Estructura de Commits")

# 3. Push a remoto
git push origin feature/your-feature-name

# 4. Crear Pull Request en GitHub/GitLab
# Target: develop (NO main)

# 5. Después de code review y merge
git checkout develop
git pull origin develop
git branch -d feature/your-feature-name
```

### Reglas Importantes

- ❌ **NUNCA** hacer commit directo a `main` o `develop`
- ✅ Siempre crear una rama feature/bugfix
- ✅ Mantener branches pequeños (< 500 líneas cuando sea posible)
- ✅ Rebase antes de crear PR para mantener historial limpio

```bash
# Antes de crear PR
git checkout develop
git pull
git checkout feature/your-feature
git rebase develop
# Resolver conflictos si existen
git push -f origin feature/your-feature
```

---

## 💻 Estándares de Código

### Rust (kairos-core, libs)

#### Formateo

Usamos `rustfmt` con configuración estándar:

```bash
# Formatear todo el workspace
cargo fmt --all

# Verificar sin modificar
cargo fmt --all -- --check
```

#### Linting

```bash
# Ejecutar Clippy (linter)
cargo clippy --all-targets --all-features -- -D warnings

# Clippy debe pasar sin warnings antes de PR
```

#### Estilo

- **Nombres de variables:** `snake_case`
- **Nombres de funciones:** `snake_case`
- **Nombres de structs/enums:** `PascalCase`
- **Constantes:** `SCREAMING_SNAKE_CASE`
- **Lifetimes:** `'a`, `'b`, etc. (lowercase)

```rust
// ✅ Bueno
struct MarketTick {
    symbol: String,
    bid_price: f64,
    ask_price: f64,
}

impl MarketTick {
    pub fn new(symbol: String, bid: f64, ask: f64) -> Self {
        Self {
            symbol,
            bid_price: bid,
            ask_price: ask,
        }
    }
    
    pub fn calculate_spread(&self) -> f64 {
        self.ask_price - self.bid_price
    }
}

const MAX_ORDER_SIZE: f64 = 1000.0;
```

#### Documentación

```rust
/// Represents a market tick with bid/ask prices.
///
/// # Examples
///
/// ```
/// let tick = MarketTick::new("BTCUSDT".to_string(), 50000.0, 50001.0);
/// assert_eq!(tick.calculate_spread(), 1.0);
/// ```
pub struct MarketTick {
    // ...
}
```

#### Error Handling

- Usar `Result<T, E>` para operaciones que pueden fallar
- Usar `thiserror` para definir errores custom
- NO usar `.unwrap()` en código de producción (solo en tests)

```rust
// ✅ Bueno
fn parse_order(data: &str) -> Result<Order, OrderError> {
    let parsed: OrderData = serde_json::from_str(data)
        .map_err(|e| OrderError::ParseError(e))?;
    Ok(Order::from(parsed))
}

// ❌ Malo
fn parse_order(data: &str) -> Order {
    let parsed: OrderData = serde_json::from_str(data).unwrap(); // ¡NO!
    Order::from(parsed)
}
```

### Java (kairos-api)

#### Formateo

Usamos **Google Java Style Guide** con plugins de IDE:

```bash
# Verificar estilo (si tienes Checkstyle configurado)
mvn checkstyle:check
```

#### Estilo

- **Nombres de variables:** `camelCase`
- **Nombres de clases:** `PascalCase`
- **Constantes:** `UPPER_SNAKE_CASE`
- **Paquetes:** `lowercase`

```java
// ✅ Bueno
@Service
public class MarketDataService {
    private static final int MAX_RETRIES = 3;
    
    private final ReactiveRedisTemplate<String, String> redisTemplate;
    
    public Flux<MarketTick> streamMarketData(String symbol) {
        return redisTemplate
            .listenToChannel("market_data:" + symbol)
            .map(this::parseMarketTick);
    }
}
```

#### Lombok

Usar Lombok para reducir boilerplate:

```java
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class OrderRequest {
    private String symbol;
    private BigDecimal quantity;
    private OrderType type;
}
```

### TypeScript/Angular (kairos-web)

#### Formateo

Usamos Prettier:

```bash
npm run lint
npm run format
```

#### Estilo

- **Variables/funciones:** `camelCase`
- **Clases/interfaces:** `PascalCase`
- **Constantes:** `UPPER_SNAKE_CASE`
- **Archivos:** `kebab-case.ts`

```typescript
// ✅ Bueno
export interface MarketTick {
  symbol: string;
  bidPrice: number;
  askPrice: number;
  timestamp: Date;
}

export class MarketDataService {
  private readonly WS_URL = 'ws://localhost:4000/ws/market-data';
  
  streamMarketData(symbol: string): Observable<MarketTick> {
    return this.webSocketService.connect(this.WS_URL)
      .pipe(
        filter(msg => msg.symbol === symbol),
        map(msg => this.parseMarketTick(msg))
      );
  }
}
```

---

## 📝 Estructura de Commits

Usamos **Conventional Commits**:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: Nueva característica
- `fix`: Corrección de bug
- `docs`: Cambios en documentación
- `style`: Formateo, sin cambios de lógica
- `refactor`: Refactorización sin cambiar funcionalidad
- `perf`: Mejoras de performance
- `test`: Añadir o modificar tests
- `chore`: Mantenimiento (deps, config)

### Scopes

- `core` - kairos-core (Rust)
- `api` - kairos-api (Java)
- `web` - kairos-web (Angular)
- `domain` - libs/kairos-domain
- `proto` - libs/kairos-proto
- `infra` - Docker, CI/CD

### Ejemplos

```bash
feat(core): implement Bellman-Ford arbitrage detection

- Added weighted graph construction from market pairs
- Implemented negative cycle detection algorithm
- Added integration tests with mock market data

Closes #42

---

fix(api): resolve WebSocket connection leak

Redis connections were not being properly closed on client disconnect.
Added explicit cleanup in WebSocketHandler.onClose()

Fixes #67

---

docs(adr): add ADR-004 for DragonflyDB selection

Documented decision to use DragonflyDB over Redis for better performance
in high-throughput scenarios.

---

perf(core): optimize order validation by 40%

- Replaced HashMap with FxHashMap for AtomicF64 lookups
- Inlined hot path functions
- Reduced allocations in order parsing

Benchmark results: 1.2ms → 0.7ms (p50)
```

---

## 🔍 Pull Requests

### Checklist Antes de Crear PR

- [ ] Código formateado (`cargo fmt`, `mvn checkstyle`, `npm run lint`)
- [ ] Linter pasa sin warnings (`cargo clippy`)
- [ ] Tests añadidos/actualizados
- [ ] Tests pasan (`cargo test`, `mvn test`, `npm test`)
- [ ] Documentación actualizada (README, ADRs, CHANGELOG)
- [ ] Branch rebased con `develop`
- [ ] Commits siguen Conventional Commits

### Plantilla de PR

```markdown
## Descripción
Breve descripción de los cambios

## Tipo de cambio
- [ ] Bugfix (non-breaking change que arregla un issue)
- [ ] Feature (non-breaking change que añade funcionalidad)
- [ ] Breaking change (fix o feature que causaría mal funcionamiento en código existente)

## ¿Cómo se ha probado?
Describe las pruebas que ejecutaste

## Checklist
- [ ] Mi código sigue el estilo del proyecto
- [ ] He realizado self-review de mi código
- [ ] He comentado mi código en áreas difíciles de entender
- [ ] He actualizado la documentación
- [ ] Mis cambios no generan nuevos warnings
- [ ] He añadido tests que prueban que mi fix es efectivo o que mi feature funciona
- [ ] Tests unitarios e integración pasan localmente

## Screenshots (si aplica)
```

### Code Review

- Se requieren **2 aprobaciones** para merge a `develop`
- Se requiere **aprobación de tech lead** para merge a `main`
- Responder a comentarios de review en menos de 24 horas
- Hacer "Squash and Merge" para mantener historial limpio

---

## 🧪 Testing

### Rust

```bash
# Ejecutar todos los tests
cargo test --workspace

# Tests con output detallado
cargo test --workspace -- --nocapture

# Tests de un módulo específico
cargo test --package kairos-core strategies::bellman_ford

# Tests con coverage (requiere tarpaulin)
cargo tarpaulin --workspace --out Html
```

### Java

```bash
cd apps/kairos-api

# Tests unitarios
mvn test

# Tests de integración
mvn verify

# Coverage report
mvn jacoco:report
# Ver en target/site/jacoco/index.html
```

### Angular

```bash
cd apps/kairos-web

# Tests unitarios
npm test

# Tests e2e
npm run e2e

# Coverage
npm run test:coverage
```

### Cobertura Mínima

- **Rust core:** >= 80%
- **Java API:** >= 75%
- **Angular:** >= 70%

---

## 📚 Documentación

### Cuándo Actualizar Documentación

- **README.md:** Al añadir nuevos componentes o cambiar arquitectura
- **ADR:** Para decisiones técnicas significativas
- **CHANGELOG.md:** En cada release
- **API_DOCS.md:** Al modificar endpoints o contratos
- **Código:** Comentarios inline para lógica compleja

### Crear un Nuevo ADR

```bash
# Copiar template
cp docs/ADR/ADR-TEMPLATE.md docs/ADR/ADR-XXX-your-decision.md

# Editar con tu decisión

# Añadir al índice en docs/ADR/README.md

# Commit
git add docs/ADR/
git commit -m "docs(adr): add ADR-XXX for [decision]"
```

---

## 🤖 Trabajando con IA

KAIRÓS está diseñado para ser colaborativo con asistentes de IA como Gemini, Copilot, etc.

### Pautas para Solicitar Cambios a la IA

#### ✅ Buenas Prácticas

```
# Específico y contextual
"Implementa la función parse_binance_ticker en 
apps/kairos-core/src/adapters/inbound/feed_handler/binance.rs 
que convierta BinanceTickerMessage a MarketTick. 
Usa serde_json y maneja errores con thiserror::Error."

# Referencias a documentación
"Lee el GLOSSARY.md y luego implementa la estrategia de 
arbitraje triangular descrita en ADR-005."
```

#### ❌ Malas Prácticas

```
# Demasiado vago
"Mejora el rendimiento"

# Sin contexto
"Añade tests" (¿tests de qué?)

# Asume conocimiento implícito
"Haz lo que discutimos antes" (la IA no tiene memoria de sesiones previas)
```

### Prompts Recomendados

```
# Para refactoring
"Refactoriza la función X para seguir el patrón hexagonal 
descrito en ADR-002. Separa la lógica de negocio de los 
adapters Redis."

# Para debugging
"El test test_bellman_ford_detects_arbitrage está fallando. 
Revisa el algoritmo en strategies/bellman_ford.rs y compáralo 
con la especificación en GLOSSARY.md."

# Para documentación
"Genera documentación inline (/// en Rust) para el módulo 
risk_engine siguiendo los mismos estándares que 
feed_handler/binance.rs."
```

### Verificar Código Generado por IA

SIEMPRE:
- [ ] Ejecutar `cargo fmt` / `mvn checkstyle` / `npm run lint`
- [ ] Ejecutar `cargo clippy`
- [ ] Ejecutar tests completos
- [ ] Revisar lógica manualmente (especialmente en trade execution y risk management)
- [ ] Verificar que sigue patrones establecidos en ADRs

---

## 🐛 Reportar Bugs

Usa el template de GitHub Issues:

```markdown
## Descripción del Bug
Descripción clara y concisa

## Pasos para Reproducir
1. Ejecutar '...'
2. Llamar al endpoint '...'
3. Ver error

## Comportamiento Esperado
Qué esperabas que sucediera

## Comportamiento Actual
Qué sucede realmente

## Logs/Screenshots

## Entorno
- OS: [ej. Ubuntu 22.04]
- Rust version: [ej. 1.75 nightly]
- Java version: [ej. OpenJDK 21]
```

---

## 📞 Contacto

- **Issues:** GitHub Issues
- **Discusiones:** GitHub Discussions
- **Email:** kairos-dev@example.com (si existiera)

---

## 📄 Licencia

Este proyecto está bajo licencia MIT. Ver [LICENSE](./LICENSE) para más detalles.

---

**¡Gracias por contribuir a KAIRÓS!** 🚀
