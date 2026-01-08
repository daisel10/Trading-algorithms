# ADR-001: Selección de Rust para el Motor de Trading

**Fecha:** 2025-12-26  
**Estado:** Aceptado  
**Autores:** KAIRÓS Team

## Contexto

El núcleo del sistema KAIRÓS (kairos-core) debe procesar miles de eventos de mercado por segundo, ejecutar algoritmos de arbitraje triangular, y enviar órdenes con latencia ultra-baja (objetivo: <10ms). La selección del lenguaje de programación es crítica para el rendimiento del sistema.

### Requisitos Clave

- **Latencia Ultra-Baja:** Procesamiento de datos de mercado en microsegundos
- **Seguridad de Memoria:** Sin crashes en producción debido a errores de memoria
- **Concurrencia:** Manejo de múltiples feeds de datos simultáneos
- **Determinismo:** Comportamiento predecible en tiempo de ejecución
- **Zero-Cost Abstractions:** Alto nivel sin sacrificar rendimiento

## Decisión

**Elegimos Rust como lenguaje principal para kairos-core** (El Monolito - Trading Engine).

### Razones Principales

1. **Performance Nativo:** Compilación a código nativo sin garbage collector
2. **Sistema de Ownership:** Garantías de seguridad de memoria en tiempo de compilación
3. **Concurrencia Segura:** `Send` y `Sync` traits previenen data races
4. **Tokio Runtime:** Async I/O eficiente para manejar miles de conexiones WebSocket
5. **Ecosistema:** Crates de calidad para WebSocket, gRPC, Redis, PostgreSQL

## Consecuencias

### Positivas ✅

- Rendimiento comparable a C++ sin complejidad de gestión manual de memoria
- Errores de concurrencia detectados en compilación, no en producción
- Excelente ecosistema de networking asíncrono (tokio, tonic, tokio-tungstenite)
- Binarios standalone sin dependencias de runtime (fácil para despliegue)
- Comunidad activa en fintech y HFT

### Negativas ❌

- **Curva de aprendizaje empinada:** Ownership, lifetimes, borrowing
- **Tiempos de compilación lentos:** Especialmente en debug mode
- **Menos desarrolladores disponibles:** Pool de talento más pequeño que Java/Python
- **Debugging complejo:** Herramientas menos maduras que en Java/C++
- **Integración con ML:** Ecosistema de AI menos maduro que Python

## Alternativas Consideradas

### C++ (Rechazada)
**Por qué se descartó:**
- Gestión manual de memoria propensa a errores
- Sin garantías de thread-safety en compilación
- Tiempo de desarrollo más largo para características equivalentes
- Ausencia de package manager maduro (Conan/vcpkg no tan robustos como Cargo)

**Ventajas perdidas:**
- Ecosistema más maduro para HFT
- Más desarrolladores senior disponibles
- Mejor integración con hardware específico (FPGA, network cards)

### Go (Rechazada)
**Por qué se descartó:**
- **Garbage Collector:** Pausas impredecibles (deal-breaker para latencia ultra-baja)
- Menos control sobre layout de memoria
- Sin garantías de concurrencia en tiempo de compilación

**Ventajas perdidas:**
- Simplicidad del lenguaje
- Goroutines muy fáciles de usar
- Compilación rápida

### Java (Rechazada para el core, elegida para API)
**Por qué se descartó para el core:**
- JVM warmup time y JIT compilation introducen latencia
- GC pausas incluso con G1/ZGC
- Overhead de memoria mayor

**Por qué SÍ se usa en kairos-api:**
- Excelente para servicios I/O-bound (REST/WebSocket)
- Ecosistema Spring muy maduro
- Pool grande de desarrolladores

### Python (Rechazada para el core, candidata para AI)
**Por qué se descartó para el core:**
- GIL limita paralelismo real
- Rendimiento 50-100x más lento que Rust en CPU-bound tasks

**Por qué SÍ se considerará para research/:**
- Líder indiscutible en ML/AI (PyTorch, TensorFlow)
- Jupyter notebooks para análisis exploratorio

## Notas de Implementación

- Usamos **Rust Nightly** para aprovechar features experimentales
- Configuramos el workspace como un monorepo con Cargo Workspaces
- Aplicamos Arquitectura Hexagonal para aislar lógica de negocio de dependencias externas
- Usamos `tokio` como runtime asíncrono principal

## Referencias

- [Rust in Production - Trading Systems](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust/)
- [High-Frequency Trading in Rust](https://www.reddit.com/r/rust/comments/hft_discussion)
- [Tokio Performance Benchmarks](https://tokio.rs/blog/2019-10-scheduler)

## Estado Actual

Este ADR está **ACEPTADO** y en producción. El monolito kairos-core está implementado en Rust desde el inicio del proyecto.
