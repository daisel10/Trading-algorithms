# Testing Guide - kairos-core

Guía completa para ejecutar y escribir tests en el motor de trading KAIRÓS.

---

## 📋 Tipos de Tests

### 1. Unit Tests

Tests de funciones individuales y lógica de negocio aislada.

**Ubicación:** Mismo archivo que el código (`#[cfg(test)] mod tests`)

**Ejecutar:**
```bash
cargo test --lib
```

**Ejemplo:**
```rust
// En src/domain/strategies/bellman_ford.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_negative_cycle() {
        let mut graph = CurrencyGraph::new();
        graph.add_edge("BTC", "ETH", 0.05);
        graph.add_edge("ETH", "USDT", 2000.0);
        graph.add_edge("USDT", "BTC", 0.00003);
        
        let cycle = graph.find_negative_cycle();
        assert!(cycle.is_some(), "Should detect arbitrage opportunity");
        
        let profit = cycle.unwrap().calculate_profit();
        assert!(profit > 0.0, "Profit should be positive");
    }
}
```

### 2. Integration Tests

Tests que verifican la interacción entre múltiples componentes.

**Ubicación:** `tests/` directory (raíz del crate)

**Ejecutar:**
```bash
cargo test --test integration_tests
```

**Ejemplo:**
```rust
// En tests/integration_tests.rs

use kairos_core::adapters::inbound::feed_handler::MockFeedHandler;
use kairos_core::domain::strategies::TriangularArbitrageStrategy;

#[tokio::test]
async fn test_end_to_end_arbitrage_detection() {
    // Setup mock feed handler
    let mut feed_handler = MockFeedHandler::new();
    feed_handler.add_tick(create_btc_eth_tick(0.05));
    feed_handler.add_tick(create_eth_usdt_tick(2000.0));
    feed_handler.add_tick(create_usdt_btc_tick(0.00003));
    
    // Setup strategy
    let mut strategy = TriangularArbitrageStrategy::new(0.01);
    
    // Process ticks
    for tick in feed_handler.get_ticks() {
        if let Some(order) = strategy.on_market_tick(tick).await {
            assert_eq!(order.strategy, StrategyType::TriangularArbitrage);
            assert!(order.quantity > 0.0);
        }
    }
}
```

### 3. Benchmark Tests

Tests de rendimiento para validar latencia.

**Ubicación:** `benches/` directory

**Ejecutar:**
```bash
cargo bench
```

**Ejemplo:**
```rust
// En benches/market_tick_processing.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kairos_core::domain::strategies::TriangularArbitrageStrategy;

fn benchmark_arbitrage_detection(c: &mut Criterion) {
    let mut strategy = TriangularArbitrageStrategy::new(0.01);
    let tick = create_sample_tick();
    
    c.bench_function("arbitrage_detection", |b| {
        b.iter(|| {
            strategy.on_market_tick(black_box(tick.clone()))
        })
    });
}

criterion_group!(benches, benchmark_arbitrage_detection);
criterion_main!(benches);
```

---

## 🚀 Ejecutar Tests

### Todos los Tests

```bash
# Ejecutar todos los tests (unit + integration)
cargo test

# Con output detallado
cargo test -- --nocapture

# Tests en paralelo (por defecto)
cargo test -- --test-threads=4

# Tests secuenciales
cargo test -- --test-threads=1
```

### Tests Específicos

```bash
# Por nombre
cargo test test_detect_negative_cycle

# Por módulo
cargo test domain::strategies

# Por patrón
cargo test bellman_ford

# Ignorar tests lentos
cargo test -- --skip slow_test
```

### Tests con Dependencias Externas

Algunos tests requieren servicios externos (Redis, TimescaleDB). Para ejecutarlos:

```bash
# 1. Levantar servicios de test
docker compose -f ../../infrastructure/docker-compose.test.yml up -d

# 2. Ejecutar tests de integración
cargo test --test integration_tests

# 3. Limpiar servicios
docker compose -f ../../infrastructure/docker-compose.test.yml down -v
```

### Tests en CI/CD

```bash
# Ejecutar en modo CI (sin color, con tiempos)
cargo test --color=never -- --nocapture --test-threads=1
```

---

## 📊 Code Coverage

### Configuración

Instalar `tarpaulin`:
```bash
cargo install cargo-tarpaulin
```

### Generar Reporte

```bash
# HTML report
cargo tarpaulin --out Html --output-dir ./coverage

# Abrir reporte
# Windows
start coverage/index.html

# Linux/Mac
xdg-open coverage/index.html
```

### Cobertura en CI

```bash
# Lcov format para integración con Codecov/Coveralls
cargo tarpaulin --out Lcov
```

### Objetivo de Cobertura

- **Core domain logic:** >= 90%
- **Adapters:** >= 70%
- **Overall:** >= 80%

---

## 🧪 Mocking y Test Utilities

### Mock Feed Handler

```rust
// En tests/mocks/feed_handler.rs

pub struct MockFeedHandler {
    ticks: Vec<MarketTick>,
    current_index: usize,
}

impl MockFeedHandler {
    pub fn new() -> Self {
        Self {
            ticks: Vec::new(),
            current_index: 0,
        }
    }
    
    pub fn add_tick(&mut self, tick: MarketTick) {
        self.ticks.push(tick);
    }
}

#[async_trait]
impl FeedHandler for MockFeedHandler {
    async fn connect(&mut self) -> Result<()> {
        Ok(())
    }
    
    async fn next_tick(&mut self) -> Result<MarketTick> {
        if self.current_index < self.ticks.len() {
            let tick = self.ticks[self.current_index].clone();
            self.current_index += 1;
            Ok(tick)
        } else {
            Err(anyhow::anyhow!("No more ticks"))
        }
    }
    
    fn exchange_name(&self) -> &str {
        "Mock"
    }
}
```

### Test Helpers

```rust
// En tests/helpers.rs

pub fn create_sample_tick(symbol: &str, bid: f64, ask: f64) -> MarketTick {
    MarketTick {
        symbol: symbol.to_string(),
        exchange: Exchange::Binance,
        bid_price: bid,
        ask_price: ask,
        bid_volume: 10.0,
        ask_volume: 10.0,
        timestamp: Utc::now(),
    }
}

pub async fn setup_test_db() -> PgPool {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://test:test@localhost/test_kairos".to_string());
    
    PgPool::connect(&db_url).await.expect("Failed to connect to test DB")
}

pub async fn cleanup_test_db(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE market_ticks CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup test DB");
}
```

---

## 🐛 Debugging Tests

### Con Logs

```bash
# Mostrar logs durante tests
RUST_LOG=debug cargo test -- --nocapture

# Solo logs de un módulo
RUST_LOG=kairos_core::adapters=debug cargo test
```

### Con Debugger

```rust
#[tokio::test]
async fn test_with_breakpoint() {
    let strategy = setup_strategy();
    
    dbg!(&strategy);  // Print debug info
    
    let result = strategy.process().await;
    
    assert!(result.is_ok());
}
```

---

## 📝 Escribir Buenos Tests

### Nomenclatura

```rust
#[test]
fn test_<what>_<condition>_<expected_result>() {
    // Ejemplo:
    // test_order_validation_when_insufficient_balance_returns_error
}
```

### Estructura AAA (Arrange-Act-Assert)

```rust
#[tokio::test]
async fn test_risk_validator_rejects_oversized_orders() {
    // Arrange
    let validator = DefaultRiskValidator::new(100.0);  // max size = 100
    let order = create_order_with_quantity(150.0);
    
    // Act
    let result = validator.validate(&order);
    
    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), RiskError::OrderTooLarge);
}
```

### Property-Based Testing (con proptest)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_spread_is_always_positive(bid in 1.0..100000.0, ask in 1.0..100000.0) {
        let tick = MarketTick {
            bid_price: bid,
            ask_price: ask.max(bid),  // Ensure ask >= bid
            ..Default::default()
        };
        
        prop_assert!(tick.spread() >= 0.0);
    }
}
```

---

## ⚡ Performance Testing

### Latency Benchmarks

Target: p50 < 1ms, p99 < 5ms para procesamiento de market tick

```bash
# Ejecutar benchmarks
cargo bench

# Solo un benchmark específico
cargo bench -- arbitrage_detection
```

### Stress Testing

```rust
#[tokio::test]
async fn stress_test_high_volume_ticks() {
    let mut strategy = setup_strategy();
    let ticks = generate_random_ticks(100_000);
    
    let start = Instant::now();
    
    for tick in ticks {
        strategy.on_market_tick(tick).await;
    }
    
    let elapsed = start.elapsed();
    let tps = 100_000.0 / elapsed.as_secs_f64();
    
    assert!(tps > 10_000.0, "Should process >10k ticks/sec");
}
```

---

## 🔒 Security Testing

### Fuzzing (con cargo-fuzz)

```bash
# Instalar cargo-fuzz
cargo install cargo-fuzz

# Crear fuzz target
cargo fuzz init

# Ejecutar fuzzer
cargo fuzz run fuzz_target_1
```

Ejemplo de fuzz target:

```rust
#[macro_use] extern crate libfuzzer_sys;
extern crate kairos_core;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Try to parse as market tick JSON
        let _ = serde_json::from_str::<MarketTick>(s);
    }
});
```

---

## 📋 Checklist para PRs

Antes de crear un Pull Request, asegúrate de:

- [ ] Todos los tests pasan: `cargo test`
- [ ] Linter pasa: `cargo clippy -- -D warnings`
- [ ] Código formateado: `cargo fmt`
- [ ] Cobertura >= 80%: `cargo tarpaulin`
- [ ] Benchmarks no regresionaron: `cargo bench`
- [ ] Documentación actualizada

---

## 🤝 Contribuir Tests

Ver [../../CONTRIBUTING.md](../../CONTRIBUTING.md) para:
- Guías de estilo de tests
- Proceso de code review
- Integración CI/CD

---

**Última actualización:** 2026-01-06  
**Mantenido por:** KAIRÓS Team
