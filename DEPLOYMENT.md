# 🚀 Guía de Despliegue - KAIRÓS Trading System

Esta guía te mostrará cómo ejecutar los componentes del sistema KAIRÓS, tanto individualmente como en conjunto usando Docker Compose.

---

## 📋 Prerequisitos

### Para desarrollo local:
- **Rust** (nightly): `rustup install nightly && rustup default nightly`
- **Node.js** 18+ y npm
- **Protobuf Compiler**: `choco install protoc` (Windows) o `apt install protobuf-compiler` (Linux)
- **PostgreSQL** con extensión TimescaleDB (opcional para desarrollo local)
- **Redis** o DragonflyDB (opcional para desarrollo local)

### Para ejecutar con Docker:
- **Docker Desktop** (con Docker Compose)

---

## 🐳 Opción 1: Ejecutar TODO con Docker Compose (Recomendado)

### Iniciar todos los servicios

```bash
# Desde la raíz del proyecto
docker compose -f infrastructure/docker-compose.yml up --build
```

Este comando levantará:
- 🗄️ **DragonflyDB** (puerto 6379) - Base de datos en memoria
- 🐘 **TimescaleDB** (puerto 5432) - Base de datos de series temporales
- ⚡ **kairos-core** (puerto 50051) - Motor de trading (gRPC)
- 🌐 **kairos-api** (puerto 4000) - API GraphQL
- 🎨 **kairos-web** (puerto 4200) - Dashboard Angular

### Detener todos los servicios

```bash
docker compose -f infrastructure/docker-compose.yml down
```

### Detener y eliminar volúmenes (datos)

```bash
docker compose -f infrastructure/docker-compose.yml down -v
```

### Ver logs de un servicio específico

```bash
# Logs de kairos-core
docker compose -f infrastructure/docker-compose.yml logs -f kairos-core

# Logs de kairos-api
docker compose -f infrastructure/docker-compose.yml logs -f kairos-api

# Logs de timescale
docker compose -f infrastructure/docker-compose.yml logs -f timescale
```

### Reconstruir una imagen específica

```bash
# Reconstruir solo kairos-core
docker compose -f infrastructure/docker-compose.yml build kairos-core

# Reconstruir solo kairos-api
docker compose -f infrastructure/docker-compose.yml build kairos-api
```

---

## 💻 Opción 2: Ejecutar Componentes Individualmente (Desarrollo)

### 1. Bases de Datos

#### DragonflyDB (alternativa a Redis)
```bash
# Opción A: Con Docker
docker run -d \
  --name dragonfly \
  -p 6379:6379 \
  docker.dragonflydb.io/dragonflydb/dragonfly:latest \
  --maxmemory=2gb

# Opción B: Redis nativo
redis-server
```

#### TimescaleDB
```bash
# Con Docker
docker run -d \
  --name timescale \
  -p 5432:5432 \
  -e POSTGRES_USER=kairos \
  -e POSTGRES_PASSWORD=kairos_password \
  -e POSTGRES_DB=kairos_trading \
  -v $(pwd)/infrastructure/db/init_timescale.sql:/docker-entrypoint-initdb.d/init.sql \
  timescale/timescaledb:latest-pg16
```

### 2. KAIRÓS Core (Motor de Trading)

```bash
# Configurar variables de entorno
export RUST_LOG=info
export DRAGONFLY_URL=redis://localhost:6379
export TIMESCALE_URL=postgresql://kairos:kairos_password@localhost:5432/kairos_trading
export GRPC_PORT=50051

# Compilar y ejecutar
cargo +nightly build --release --bin kairos-core
./target/release/kairos-core

# O directamente con cargo run
cargo +nightly run --release --bin kairos-core
```

**Puerto:** 50051 (gRPC)

### 3. KAIRÓS API (GraphQL Gateway)

```bash
# Configurar variables de entorno
export RUST_LOG=info
export DRAGONFLY_URL=redis://localhost:6379
export CORE_GRPC_URL=http://localhost:50051

# Compilar y ejecutar
cargo +nightly build --release --bin kairos-api
./target/release/kairos-api

# O directamente con cargo run
cargo +nightly run --release --bin kairos-api
```

**Puerto:** 4000 (GraphQL)  
**Endpoint:** http://localhost:4000/graphql

### 4. KAIRÓS Web (Dashboard Angular)

```bash
# Navegar al directorio del frontend
cd apps/kairos-web

# Instalar dependencias (primera vez)
npm install

# Ejecutar servidor de desarrollo
npm run start

# O con ng serve
ng serve
```

**Puerto:** 4200  
**URL:** http://localhost:4200

---

## 🔧 Comandos Útiles

### Rust (Desarrollo)

```bash
# Compilar todo el workspace
cargo build --release

# Compilar solo un binario específico
cargo build --release --bin kairos-core
cargo build --release --bin kairos-api

# Ejecutar tests
cargo test

# Formatear código
cargo fmt

# Linter
cargo clippy

# Limpiar artefactos de compilación
cargo clean
```

### Docker

```bash
# Ver contenedores en ejecución
docker ps

# Ver todas las imágenes
docker images

# Limpiar todo (contenedores, volúmenes, imágenes no utilizadas)
docker system prune -a --volumes

# Ejecutar un shell dentro de un contenedor
docker exec -it kairos-core sh
docker exec -it kairos-timescale psql -U kairos -d kairos_trading
```

### Angular (Frontend)

```bash
cd apps/kairos-web

# Servidor de desarrollo
npm run start

# Build de producción
npm run build

# Ejecutar tests
npm test

# Ejecutar tests e2e
npm run e2e

# Linter
npm run lint
```

---

## 🌐 URLs de Acceso

Una vez que todos los servicios estén corriendo:

| Servicio | URL | Descripción |
|----------|-----|-------------|
| **Web Dashboard** | http://localhost:4200 | Interfaz de usuario principal |
| **GraphQL API** | http://localhost:4000/graphql | API GraphQL (Playground) |
| **gRPC Core** | localhost:50051 | Motor de trading (gRPC) |
| **DragonflyDB** | localhost:6379 | Redis-compatible (CLI: `redis-cli`) |
| **TimescaleDB** | localhost:5432 | PostgreSQL (CLI: `psql -U kairos -d kairos_trading`) |

---

## 🐛 Solución de Problemas

### Error: "Could not find `protoc`"
```bash
# Windows
choco install protoc

# Linux/Ubuntu
sudo apt-get install protobuf-compiler

# macOS
brew install protobuf
```

### Error: "edition2024 is required"
```bash
# Asegúrate de usar Rust nightly
rustup install nightly
rustup default nightly
```

### Error: TimescaleDB no inicia
```bash
# Limpia los volúmenes y vuelve a crear
docker compose -f infrastructure/docker-compose.yml down -v
docker compose -f infrastructure/docker-compose.yml up --build
```

### Error: Puerto ya en uso
```bash
# Ver qué proceso está usando el puerto
netstat -ano | findstr :4000  # Windows
lsof -i :4000                 # Linux/macOS

# Detener todos los contenedores Docker
docker compose -f infrastructure/docker-compose.yml down
```

### Logs completos de compilación Docker
```bash
# Ver logs detallados durante el build
docker compose -f infrastructure/docker-compose.yml build --progress=plain --no-cache
```

---

## 📊 Arquitectura de Servicios

```
┌─────────────────┐
│   kairos-web    │  (Angular - Puerto 4200)
│   Dashboard     │
└────────┬────────┘
         │ HTTP
         ▼
┌─────────────────┐
│   kairos-api    │  (Rust GraphQL - Puerto 4000)
│   Gateway       │
└────────┬────────┘
         │ gRPC
         ▼
┌─────────────────┐
│  kairos-core    │  (Rust Trading Engine - Puerto 50051)
│  Trading Engine │
└────┬──────┬─────┘
     │      │
     │      └──────────┐
     ▼                 ▼
┌──────────┐    ┌──────────────┐
│Dragonfly │    │  TimescaleDB │
│ (Redis)  │    │  (Postgres)  │
│ :6379    │    │  :5432       │
└──────────┘    └──────────────┘
```

---

## 📝 Notas Adicionales

- **Modo desarrollo**: Los cambios en el código Rust requieren recompilación
- **Modo producción**: Usa `--release` para compilaciones optimizadas
- **Hot reload**: Solo disponible en el frontend Angular (`npm run start`)
- **Datos persistentes**: Los volúmenes Docker mantienen los datos entre reinicios
- **Primer inicio**: La compilación puede tomar varios minutos la primera vez

---

## 🔐 Configuración de Seguridad (Producción)

Para despliegue en producción, asegúrate de:

1. Cambiar las contraseñas predeterminadas en `docker-compose.yml`
2. Usar variables de entorno en lugar de valores hardcodeados
3. Habilitar TLS/SSL para todas las conexiones
4. Configurar firewalls y reglas de red apropiadas
5. Implementar autenticación y autorización en la API

---

## 📚 Más Información

- [Documentación de Rust](https://doc.rust-lang.org/)
- [TimescaleDB Docs](https://docs.timescale.com/)
- [DragonflyDB Docs](https://www.dragonflydb.io/docs)
- [Angular Docs](https://angular.io/docs)
- [Docker Compose Docs](https://docs.docker.com/compose/)
