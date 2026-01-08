# Architecture Decision Records (ADR)

Este directorio contiene el registro histórico de todas las decisiones arquitectónicas importantes tomadas en el proyecto KAIRÓS.

## ¿Qué es un ADR?

Un Architecture Decision Record documenta una decisión técnica significativa junto con su contexto y consecuencias. Cada ADR es inmutable una vez tomada la decisión - si cambiamos de opinión, creamos un nuevo ADR.

## Formato

Cada ADR sigue este formato:

```markdown
# ADR-XXX: [Título de la Decisión]

**Fecha:** YYYY-MM-DD
**Estado:** [Propuesto | Aceptado | Rechazado | Obsoleto | Superseded by ADR-YYY]
**Contexto:** [Entorno técnico, problema a resolver]
**Decisión:** [Qué decidimos hacer]
**Consecuencias:** [Impactos positivos y negativos]
**Alternativas Consideradas:** [Otras opciones que evaluamos]
```

## Índice de ADRs

1. [ADR-001: Selección de Rust para el Motor de Trading](./ADR-001-seleccion-rust-motor-trading.md)
2. [ADR-002: Arquitectura Hexagonal en kairos-core](./ADR-002-arquitectura-hexagonal.md)
3. [ADR-003: Uso de gRPC para Comunicación Interna](./ADR-003-grpc-comunicacion-interna.md)
4. [ADR-004: DragonflyDB vs Redis](./ADR-004-dragonflydb-vs-redis.md)
5. [ADR-005: TimescaleDB para Series Temporales](./ADR-005-timescaledb-series-temporales.md)
6. [ADR-006: Java Spring WebFlux para API Gateway](./ADR-006-java-spring-webflux-api.md)
7. [ADR-007: Monorepo vs Multirepo](./ADR-007-monorepo-vs-multirepo.md)
8. [ADR-008: Canales Tokio MPSC/Broadcast](./ADR-008-tokio-channels.md)

## Cómo Crear un Nuevo ADR

1. Copia la plantilla `ADR-TEMPLATE.md`
2. Asigna el siguiente número secuencial
3. Discute con el equipo antes de marcar como "Aceptado"
4. Actualiza este índice
5. Commitea el ADR como parte de tu PR

## Principios

- **Inmutabilidad:** No modifiques ADRs aceptados. Si cambia la decisión, crea un nuevo ADR que lo supersede.
- **Contexto Completo:** Explica el "por qué", no solo el "qué".
- **Alternativas:** Siempre documenta qué otras opciones consideraste y por qué las descartaste.
