# Testing Guide - kairos-web

Guía para ejecutar y escribir tests en kairos-web (Angular).

---

## 🧪 Tipos de Tests

### Unit Tests (Jasmine/Karma)

```bash
npm test
```

**Ejemplo:**
```typescript
describe('MarketDataService', () => {
  it('should fetch market ticks', (done) => {
    service.getMarketTicks('BTCUSDT', 10).subscribe(ticks => {
      expect(ticks.length).toBe(10);
      done();
    });
  });
});
```

### E2E Tests

```bash
npm run e2e
```

### Coverage

```bash
npm run test:coverage
# Ver reporte en coverage/index.html
```

**Objetivo:** >= 70% coverage

---

**Última actualización:** 2026-01-06
