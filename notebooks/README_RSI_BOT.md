# 🤖 RSI Trading Bot - Guía de Uso

## 📋 Descripción

Bot de trading automatizado basado en el indicador RSI (Relative Strength Index) con protecciones de seguridad integradas para operar en Binance Testnet.

## ✨ Características Principales

### 🛡️ Protecciones de Seguridad

1. **Validación de Saldo Multi-Capa**
   - Verifica saldo mínimo antes de operar
   - Calcula margen para comisiones (0.1% estimado)
   - Previene errores por fondos insuficientes

2. **Sistema de Cooldown**
   - Tiempo de espera configurable entre operaciones (default: 30 min)
   - Previene sobre-trading en mercados laterales
   - Logging de tiempo restante

3. **Gestión de Riesgo**
   - Porcentaje configurable del capital (default: 10%)
   - Nunca usa 100% del saldo
   - Protección contra liquidación de cuenta

4. **Manejo de Errores Robusto**
   - Excepciones tipadas por categoría
   - Logging estructurado en cada capa
   - Recuperación graceful de errores de red

### 📊 Arquitectura Técnica

```
RSITradingBot
├── TradingValidator (Validaciones)
│   ├── validate_balance()
│   ├── validate_cooldown()
│   └── validate_market_data()
├── Excepciones Tipadas
│   ├── InsufficientBalanceError
│   ├── CooldownActiveError
│   ├── MarketDataError
│   └── OrderExecutionError
└── Ciclo de Trading
    ├── [1/6] Validar cooldown
    ├── [2/6] Obtener datos de mercado
    ├── [3/6] Analizar señal RSI
    ├── [4/6] Validar saldo
    ├── [5/6] Ejecutar orden
    └── [6/6] Activar cooldown
```

## 🚀 Instalación

### 1. Instalar Dependencias

```bash
pip install ccxt pandas ta python-dotenv
```

### 2. Configurar Credenciales

Crea un archivo `.env` en el directorio `notebooks/`:

```bash
cp .env.example.rsi_bot .env
```

Edita `.env` y agrega tus credenciales de Binance Testnet:

```env
BINANCE_TESTNET_API_KEY=tu_api_key_aqui
BINANCE_TESTNET_API_SECRET=tu_api_secret_aqui
```

**Obtener credenciales de testnet:**

1. Visita <https://testnet.binance.vision/>
2. Inicia sesión con GitHub
3. Genera API Key y Secret
4. Copia las credenciales a tu `.env`

### 3. Abrir el Notebook

```bash
jupyter notebook rsi_trading_bot.ipynb
```

## 📖 Uso

### Ejecución Básica

1. **Ejecutar todas las celdas de configuración** (1-6)
2. **Configurar credenciales** (celda 6)
3. **Inicializar el bot** (celda 7)
4. **Ejecutar un ciclo único** (celda 8)

```python
# Ejecutar un ciclo de trading
result = bot.run_trading_cycle()
```

### Ejecución Continua (Opcional)

Para ejecutar el bot en loop continuo, descomenta y ejecuta la celda 9:

```python
CHECK_INTERVAL_SECONDS = 60  # Revisar cada 60 segundos
MAX_ITERATIONS = 100  # Límite de seguridad
```

⚠️ **Advertencia**: El loop continuo ejecutará el bot indefinidamente. Usa `Ctrl+C` para detener.

### Testing de Componentes

Las celdas 11.1-11.4 permiten probar componentes individuales:

- **Test 1**: Obtención de datos de mercado
- **Test 2**: Cálculo de RSI
- **Test 3**: Verificación de saldo
- **Test 4**: Generación de señal completa

## ⚙️ Configuración

### Parámetros del Bot

```python
config = TradingConfig(
    symbol="BTC/USDT",           # Par de trading
    timeframe="15m",             # Timeframe para RSI
    rsi_period=14,               # Período del RSI
    rsi_oversold=30.0,           # Umbral de sobreventa
    capital_percentage=0.10,     # 10% del saldo por operación
    cooldown_minutes=30,         # Tiempo entre operaciones
    min_balance_usdt=11.0,       # Saldo mínimo requerido
    testnet=True                 # Modo testnet
)
```

### Personalización Recomendada

| Parámetro | Conservador | Moderado | Agresivo |
|-----------|-------------|----------|----------|
| `capital_percentage` | 0.05 (5%) | 0.10 (10%) | 0.20 (20%) |
| `cooldown_minutes` | 60 | 30 | 15 |
| `rsi_oversold` | 25 | 30 | 35 |

## 📊 Interpretación de Resultados

### Estados de Ciclo

```json
{
  "status": "executed",  // Posibles: pending, executed, failed, cooldown
  "signal": {
    "rsi": 28.5,
    "price": 42500.0,
    "signal": "BUY",
    "reason": "RSI en sobreventa (28.5 < 30)"
  },
  "order": {
    "id": "12345",
    "status": "filled",
    "filled": 0.002356,
    "cost": 100.15
  }
}
```

### Logging Estructurado

```
2026-01-20 00:18:00 | INFO     | RSI_TradingBot | ================================================================================
2026-01-20 00:18:00 | INFO     | RSI_TradingBot | 🔄 INICIANDO CICLO DE TRADING
2026-01-20 00:18:00 | INFO     | RSI_TradingBot | ================================================================================
2026-01-20 00:18:01 | INFO     | RSI_TradingBot | [1/6] Validando cooldown...
2026-01-20 00:18:01 | INFO     | RSI_TradingBot | ✅ Cooldown completado, listo para operar
2026-01-20 00:18:01 | INFO     | RSI_TradingBot | [2/6] Obteniendo datos de mercado...
2026-01-20 00:18:02 | INFO     | RSI_TradingBot | ✅ Datos obtenidos: 42 velas desde 2026-01-19 13:45:00
```

## 🔍 Manejo de Errores

### Errores Comunes y Soluciones

#### 1. `InsufficientBalanceError`

```
❌ Saldo insuficiente: 8.50 USDT disponible, 11.00 USDT necesario
```

**Solución**: Deposita más USDT en tu cuenta de testnet o reduce `capital_percentage`.

#### 2. `CooldownActiveError`

```
⏳ Cooldown activo. Tiempo restante: 15 minutos
```

**Solución**: Espera a que termine el cooldown o reduce `cooldown_minutes`.

#### 3. `MarketDataError`

```
❌ Error de red al obtener datos: Connection timeout
```

**Solución**: Verifica tu conexión a internet y reintenta.

#### 4. `OrderExecutionError`

```
❌ Orden inválida: LOT_SIZE filter error
```

**Solución**: El monto calculado es menor al mínimo permitido. Aumenta `capital_percentage`.

## 🧪 Testing

### Verificación Pre-Operación

Antes de ejecutar el bot, verifica:

```python
# 1. Conectividad
bot.exchange.fetch_ticker('BTC/USDT')

# 2. Saldo
balance = bot.get_available_balance()
print(f"Saldo: {balance:.2f} USDT")

# 3. Datos de mercado
df = bot.fetch_market_data()
print(f"Velas obtenidas: {len(df)}")

# 4. RSI actual
rsi = bot.calculate_rsi(df)
print(f"RSI: {rsi:.2f}")
```

## 📈 Análisis de Rendimiento

### Ver Historial

```python
history = bot.get_trade_history()
print(f"Total de operaciones: {len(history)}")

# Guardar en archivo
bot.save_trade_history("trade_history.json")
```

### Métricas Básicas

```python
import pandas as pd

df_trades = pd.DataFrame(history)
print(f"Operaciones exitosas: {(df_trades['status'] == 'executed').sum()}")
print(f"Operaciones fallidas: {(df_trades['status'] == 'failed').sum()}")
```

## ⚠️ Advertencias de Seguridad

### ❌ NO HACER

- **NO** uses credenciales de producción en el notebook
- **NO** hardcodees API keys en el código
- **NO** uses 100% del capital en una operación
- **NO** desactives las validaciones de seguridad
- **NO** ejecutes en producción sin backtesting exhaustivo

### ✅ HACER

- **SÍ** usa testnet para todas las pruebas
- **SÍ** mantén las API keys en `.env`
- **SÍ** revisa los logs antes de cada sesión
- **SÍ** empieza con porcentajes bajos (5-10%)
- **SÍ** implementa stop loss antes de producción

## 🚀 Mejoras Futuras

### Próximos Pasos Recomendados

1. **Stop Loss Automático**
   - Implementar órdenes OCO (One-Cancels-Other)
   - Stop loss dinámico basado en ATR

2. **Take Profit**
   - Niveles de salida basados en Fibonacci
   - Trailing take profit

3. **Confirmación Multi-Indicador**
   - Agregar MACD para confirmar tendencia
   - Bollinger Bands para volatilidad
   - Volumen para validar señales

4. **Backtesting**
   - Sistema de backtesting con datos históricos
   - Métricas: Sharpe Ratio, Max Drawdown, Win Rate

5. **Notificaciones**
   - Integración con Telegram Bot
   - Alertas por email en eventos críticos

6. **Dashboard**
   - Visualización en tiempo real con Streamlit
   - Gráficos de rendimiento con Plotly

## 📚 Recursos Adicionales

### Documentación

- [CCXT Documentation](https://docs.ccxt.com/)
- [Binance API Docs](https://binance-docs.github.io/apidocs/spot/en/)
- [TA Library](https://technical-analysis-library-in-python.readthedocs.io/)

### Binance Testnet

- [Testnet Portal](https://testnet.binance.vision/)
- [Testnet Faucet](https://testnet.binance.vision/) (para obtener fondos de prueba)

### Trading Algorítmico

- [Investopedia - RSI](https://www.investopedia.com/terms/r/rsi.asp)
- [Risk Management](https://www.investopedia.com/articles/trading/09/risk-management.asp)

## 🆘 Soporte

Si encuentras problemas:

1. Revisa los logs en el notebook
2. Verifica la sección "Errores Comunes"
3. Ejecuta las celdas de testing (11.1-11.4)
4. Consulta la documentación de CCXT

## 📝 Changelog

### v1.0.0 (2026-01-20)

- ✅ Implementación inicial con arquitectura defensiva
- ✅ Sistema de validación multi-capa
- ✅ Manejo de errores tipado
- ✅ Logging estructurado
- ✅ Cooldown entre operaciones
- ✅ Configuración centralizada
- ✅ Testing de componentes individuales

---

**Autor**: Kairos  
**Licencia**: MIT  
**Versión**: 1.0.0
