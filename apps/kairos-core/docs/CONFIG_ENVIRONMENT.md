# Guía Completa del Sistema de Configuración

## 📋 Índice

1. [Conceptos Fundamentales](#conceptos-fundamentales)
2. [Arquitectura del Sistema](#arquitectura-del-sistema)
3. [Relación entre .env y config/](#relación-entre-env-y-config)
4. [Orden de Prioridad](#orden-de-prioridad)
5. [Ejemplos Prácticos](#ejemplos-prácticos)
6. [Variables de Entorno](#variables-de-entorno)
7. [Casos de Uso](#casos-de-uso)

---

## Conceptos Fundamentales

### ¿Qué es la Configuración por Capas?

El sistema de configuración de kairos-core utiliza un patrón **jerárquico por capas** donde cada capa puede sobrescribir los valores de la capa anterior. Esto permite:

- ✅ Separación clara entre configuración base y específica por entorno
- ✅ Personalización local sin afectar el código compartido
- ✅ Sobrescritura granular con variables de entorno
- ✅ Configuración tipo-segura con TOML + Rust

---

## Arquitectura del Sistema

### Estructura de Archivos

```
apps/kairos-core/
├── .env                          # Variables de entorno actuales (gitignored)
├── .env.example                  # Plantilla de ejemplo
└── config/
    ├── default.toml              # Configuración base (SIEMPRE cargado)
    ├── development.toml          # Overrides para desarrollo
    ├── test.toml                 # Overrides para testing
    ├── production.toml           # Overrides para producción
    ├── local.toml                # Overrides locales (gitignored)
    └── local.toml.example        # Plantilla para local.toml
```

### Tipos de Archivos

| Archivo | Formato | Propósito | En Git |
|---------|---------|-----------|--------|
| `.env` | KEY=value | Variables de entorno del sistema | ❌ No |
| `.env.example` | KEY=value | Plantilla de referencia | ✅ Sí |
| `config/*.toml` | TOML estructurado | Configuración jerárquica | ✅ Sí (excepto local.toml) |

---

## Relación entre .env y config/

### ¿Cómo Se Complementan?

Los archivos `.env` y `config/*.toml` trabajan juntos pero tienen propósitos diferentes:

#### Archivos .env

- **Propósito**: Variables de entorno del sistema operativo
- **Formato**: Plano `KEY=value`
- **Cuándo usar**: Para secrets, API keys, configuración específica de máquina
- **Ejemplos**:

  ```bash
  DATABASE_PASSWORD=secret123
  API_KEY=xyz789
  APP_ENV=production
  ```

#### Archivos config/*.toml

- **Propósito**: Configuración estructurada de la aplicación
- **Formato**: TOML jerárquico
- **Cuándo usar**: Para configuración compartida, valores por defecto, settings por entorno
- **Ejemplos**:

  ```toml
  [grpc]
  port = 50051
  host = "0.0.0.0"
  
  [trading]
  max_position_size = 1000.0
  max_leverage = 3.0
  ```

### Flujo de Carga

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Aplicación inicia                                        │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. dotenvy::dotenv() carga .env → Variables de entorno      │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. Lee APP_ENV de variables de entorno                      │
│    (default: "development" si no existe)                    │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. CAPA 1: Carga config/default.toml                        │
│    → Configuración base                                     │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. CAPA 2: Carga config/{APP_ENV}.toml                      │
│    → Sobrescribe valores específicos del entorno            │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 6. CAPA 3: Carga config/local.toml (si existe)              │
│    → Sobrescribe valores locales del desarrollador          │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 7. CAPA 4: Lee variables de entorno                         │
│    → Máxima prioridad, sobrescribe todo lo anterior         │
│    Formatos: GRPC_PORT o KAIROS__GRPC__PORT                │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│ 8. Deserializa a Settings struct → Aplicación lista         │
└─────────────────────────────────────────────────────────────┘
```

---

## Orden de Prioridad

### Prioridad de Sobrescritura (de menor a mayor)

```
1. config/default.toml           [Prioridad MÁS BAJA]
   ↓ (sobrescrito por)
2. config/{environment}.toml
   ↓ (sobrescrito por)
3. config/local.toml
   ↓ (sobrescrito por)
4. Variables de entorno           [Prioridad MÁS ALTA]
```

### Ejemplo de Sobrescritura

Supongamos que queremos configurar el puerto gRPC:

**1. config/default.toml**

```toml
[grpc]
port = 50051  # Valor por defecto
```

**2. config/development.toml**

```toml
[grpc]
port = 50051  # Mismo valor (no sobrescribe)
```

**3. config/local.toml** (si existe)

```toml
[grpc]
port = 50053  # Sobrescribe a 50053
```

**4. Variable de entorno**

```bash
$env:GRPC_PORT="50054"  # Sobrescribe TODO, puerto final = 50054
```

**Resultado final**: `port = 50054` ✅

---

## Ejemplos Prácticos

### Ejemplo 1: Configuración Básica sin Overrides

**Archivo**: `config/default.toml`

```toml
rust_log = "info"

[grpc]
port = 50051
host = "0.0.0.0"
```

**Comando**:

```powershell
cargo run
```

**Resultado**: Usa valores de `default.toml` directamente

- Puerto: 50051
- Logging: info

---

### Ejemplo 2: Usando Entorno de Desarrollo

**Archivo**: `config/development.toml`

```toml
rust_log = "debug,kairos_core=trace"  # Más verbose

[grpc]
host = "127.0.0.1"  # Solo localhost en dev
```

**Comando**:

```powershell
# APP_ENV no está seteado, usa development por defecto
cargo run
```

**Resultado**: Combina default.toml + development.toml

- Puerto: 50051 (de default.toml)
- Host: 127.0.0.1 (sobrescrito por development.toml)
- Logging: debug (sobrescrito por development.toml)

---

### Ejemplo 3: Override Local del Desarrollador

**Archivo**: `config/local.toml`

```toml
[grpc]
port = 50099  # Puerto personalizado local
```

**Comando**:

```powershell
cargo run
```

**Resultado**: Combina default + development + local

- Puerto: 50099 (sobrescrito por local.toml)
- Host: 127.0.0.1 (de development.toml)
- Logging: debug (de development.toml)

---

### Ejemplo 4: Variables de Entorno Máxima Prioridad

**Archivo .env**:

```bash
GRPC_PORT=60000
MAX_POSITION_SIZE=50.0
```

**Comando**:

```powershell
cargo run
```

**Resultado**: Variables de .env sobrescriben todo

- Puerto: 60000 (de .env, máxima prioridad)
- Host: 127.0.0.1 (de development.toml)
- max_position_size: 50.0 (de .env)

---

### Ejemplo 5: Producción con Override Temporal

**Comando**:

```powershell
$env:APP_ENV="production"
$env:GRPC_PORT="50052"  # Override temporal
cargo run
```

**Resultado**: Usa production.toml + override de puerto

- Puerto: 50052 (variable de entorno, máxima prioridad)
- Logging: warn (de production.toml)
- Exchange URLs: Producción live (de production.toml)

---

## Variables de Entorno

### Formatos Soportados

El sistema acepta dos formatos para variables de entorno:

#### 1. Flat (sin prefijo)

```bash
GRPC_PORT=50052
MAX_POSITION_SIZE=500.0
RUST_LOG=trace
```

#### 2. Hierarchical (con prefijo KAIROS)

```bash
KAIROS__GRPC__PORT=50052
KAIROS__TRADING__MAX_POSITION_SIZE=500.0
```

### Archivo .env vs Variables del Sistema

#### Opción A: Archivo .env

```bash
# .env
APP_ENV=production
GRPC_PORT=50052
```

```powershell
cargo run  # Lee automáticamente .env
```

#### Opción B: Variables del sistema

```powershell
$env:APP_ENV="production"
$env:GRPC_PORT="50052"
cargo run
```

**Ambas opciones son equivalentes**, pero:

- `.env` es mejor para configuración persistente local
- Variables del sistema son mejores para overrides temporales

---

## Casos de Uso

### Caso 1: Desarrollo Local

**Situación**: Desarrollador trabajando en su máquina

**Configuración**:

- No tocar archivos TOML (están en git)
- Crear `config/local.toml` con overrides personales
- Variables sensibles en `.env` (gitignored)

```toml
# config/local.toml
[grpc]
port = 50099  # Puerto único para evitar conflictos

[trading]
max_position_size = 10.0  # Muy conservador para pruebas
```

---

### Caso 2: CI/CD Pipeline

**Situación**: Tests automatizados en GitHub Actions

**Configuración**:

```yaml
# .github/workflows/test.yml
env:
  APP_ENV: test
  DATABASE_URL: postgresql://test:test@localhost/test
```

El sistema usa `config/test.toml` + variables de entorno del pipeline

---

### Caso 3: Docker Deployment

**Situación**: Deploy a producción con Docker

**docker-compose.yml**:

```yaml
services:
  kairos-core:
    environment:
      - APP_ENV=production
      - DATABASE_PASSWORD=${DB_PASS}  # Desde secrets
      - API_KEY=${API_KEY}
```

El sistema usa `config/production.toml` + variables de Docker

---

### Caso 4: Testing en Múltiples Entornos

**Situación**: Probar configuración de producción localmente

```powershell
# Simular producción localmente
$env:APP_ENV="production"
$env:GRPC_HOST="127.0.0.1"  # Override: solo localhost
cargo run
```

Usa settings de producción PERO con host local por seguridad

---

## Resumen Visual

### ¿Qué Archivo Usar Cuándo?

```
┌─────────────────────────────────────────────────────────┐
│ ¿Necesitas...?                                          │
└─────────────────────────────────────────────────────────┘

📋 Configuración base compartida
   → config/default.toml

🔧 Settings específicos de desarrollo
   → config/development.toml

🧪 Settings específicos de testing
   → config/test.toml

🚀 Settings específicos de producción
   → config/production.toml

👤 Tu configuración personal local
   → config/local.toml (crea este archivo)

🔐 Secrets, API keys, contraseñas
   → .env (o variables de entorno del sistema)

⚡ Override temporal para una ejecución
   → Variables de entorno: $env:KEY="value"
```

---

## Reglas de Oro

1. **NUNCA** commitear `.env` o `config/local.toml` ❌
2. **SIEMPRE** usar `config/default.toml` para valores base ✅
3. **SOLO** sobrescribir lo que cambia en archivos de entorno ✅
4. **PREFERIR** variables de entorno para secrets 🔐
5. **VERIFICAR** logs al iniciar para confirmar entorno correcto ✅

---

## Debugging

### Ver Qué Configuración Se Cargó

Los logs de inicio muestran el entorno activo:

```
🚀 Starting KAIRÓS Trading Core...
⚡ Initializing Tokio Runtime
🌍 Environment: development    <-- ¡Aquí!
📋 Configuration loaded successfully
   gRPC Server: 127.0.0.1:50051
   Exchange: wss://wspap.okx.com:8443/ws/v5/public
```

### Problemas Comunes

**Q: Mi `config/local.toml` no funciona**

```bash
# Verificar que existe
ls config/local.toml

# Verificar sintaxis TOML
# Debe tener secciones [nombre]
```

**Q: Variables de .env no se aplican**

```bash
# Verificar que .env está en la raíz del proyecto
# Verificar que dotenvy::dotenv() se llama antes de Settings::new()
```

**Q: No sé qué valor se está usando**

```rust
// Agregar logging temporal en config.rs
println!("Final port: {}", settings.grpc.port);
```
