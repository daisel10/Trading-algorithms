---
name: testing
description: >
  Complete testing guide for kairos-core including unit, integration, and benchmark tests.
  Trigger: When writing tests, running tests, setting up test infrastructure, creating mocks, benchmarking performance, measuring code coverage.
license: Apache-2.0
metadata:
  author: kairos-team
  version: "1.0"
---

## When to Use

- Writing unit, integration, or benchmark tests
- Setting up test infrastructure and mocks
- Running tests locally or in CI/CD
- Measuring code coverage
- Debugging failing tests
- Performance testing and benchmarking

## Critical Patterns

### Test Types

| Type | Location | Command | Purpose |
|------|----------|---------|---------|
| **Unit Tests** | Same file (`#[cfg(test)] mod tests`) | `cargo test --lib` | Test individual functions |
| **Integration Tests** | `tests/` directory | `cargo test --test integration_tests` | Test component interaction |
| **Benchmark Tests** | `benches/` directory | `cargo bench` | Performance validation |

### AAA Pattern (Arrange-Act-Assert)

```rust
#[tokio::test]
async fn test_risk_validator_rejects_oversized_orders() {
    // Arrange - Set up test data
    let validator = DefaultRiskValidator::new(100.0);
    let order = create_order_with_quantity(150.0);
    
    // Act - Execute the code under test
    let result = validator.validate(&order);
    
    // Assert - Verify the result
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), RiskError::OrderTooLarge);
}
```

### Test Naming Convention

```rust
#[test]
fn test_<what>_<condition>_<expected_result>() {
    // Example:
    // test_order_validation_when_insufficient_balance_returns_error
}
```

### Code Coverage Targets

- **Core domain logic**: ≥ 90%
- **Adapters**: ≥ 70%
- **Overall**: ≥ 80%

## Code Examples

### Unit Test

```rust
// In src/domain/strategies/bellman_ford.rs

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

### Integration Test

```rust
// In tests/integration_tests.rs

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

### Mock Feed Handler

```rust
// In tests/mocks/feed_handler.rs

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
// In tests/helpers.rs

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

### Benchmark Test

```rust
// In benches/market_tick_processing.rs

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

## Commands

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test
cargo test test_detect_negative_cycle

# Run tests by module
cargo test domain::strategies

# Run tests by pattern
cargo test bellman_ford

# Run tests sequentially
cargo test -- --test-threads=1

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench -- arbitrage_detection

# With logs enabled
RUST_LOG=debug cargo test -- --nocapture

# Run with external dependencies (Docker)
docker compose -f ../../infrastructure/docker-compose.test.yml up -d
cargo test --test integration_tests
docker compose -f ../../infrastructure/docker-compose.test.yml down -v
```

## Code Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate HTML report
cargo tarpaulin --out Html --output-dir ./coverage

# Open report (Windows)
start coverage/index.html

# Generate Lcov for CI/CD
cargo tarpaulin --out Lcov

# With specific target
cargo tarpaulin --lib --out Html
```

## Debugging Tests

```bash
# Show logs during tests
RUST_LOG=debug cargo test -- --nocapture

# Module-specific logging
RUST_LOG=kairos_core::adapters=debug cargo test

# Use dbg! macro
#[tokio::test]
async fn test_with_debug() {
    let strategy = setup_strategy();
    dbg!(&strategy);  // Print debug info
    let result = strategy.process().await;
    assert!(result.is_ok());
}
```

## Best Practices

### ✅ DO

- Use descriptive test names: `test_<what>_<condition>_<result>`
- Follow AAA pattern (Arrange-Act-Assert)
- Create helper functions for common test setup
- Use mocks for external dependencies
- Test both success and failure cases
- Keep tests focused on one thing

### ❌ DON'T

- Write tests that depend on external services without mocks
- Create tests that depend on execution order
- Test implementation details instead of behavior
- Write tests with random/flaky results
- Duplicate test setup code

## Performance Testing

### Latency Targets

- p50 < 1ms for market tick processing
- p99 < 5ms for market tick processing

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

## PR Checklist

Before creating a Pull Request:

- [ ] All tests pass: `cargo test`
- [ ] Linter passes: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt`
- [ ] Coverage ≥ 80%: `cargo tarpaulin`
- [ ] Benchmarks not regressed: `cargo bench`
- [ ] Documentation updated

## Resources

- **Source Documentation**: [TESTING.md](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/docs/TESTING.md)
- **Test Directory**: [tests/](file:///c:/Users/david/Documents/Trading-algorithms/apps/kairos-core/tests/)
- **Criterion Documentation**: <https://bheisler.github.io/criterion.rs/book/>
- **Tokio Testing**: <https://tokio.rs/tokio/topics/testing>
