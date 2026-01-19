# Kauiros: Directrices del Repositorio & Configuración de Agentes

## 📘 Filosofía de Uso
**Este documento es la fuente de la verdad para el flujo de trabajo en Kauiros.**
Su objetivo es reducir la carga cognitiva: no memorices comandos, consulta las tablas de "Habilidades" según el contexto en el que te encuentres.

---

## 🛠 Habilidades Disponibles (Skillset)
Estas son las herramientas permitidas y los patrones de diseño aprobados para Kauiros.

### 🌐 Habilidades Tecnológicas (Tech Stack)
*Patrones técnicos puros, agnósticos al negocio.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `[LENGUAJE]` | [Ej: TypeScript, Python, Rust] - Tipado estricto, normas base. | `docs/skills/LANG.md` |
| `[FRAMEWORK_BACK]` | [Ej: FastAPI, Django, Node] - Estructura de APIs, controladores. | `docs/skills/BACKEND.md` |
| `[FRAMEWORK_FRONT]` | [Ej: React, Vue, Svelte] - Componentes, estado, UI. | `docs/skills/FRONTEND.md` |
| `[DB_ORM]` | [Ej: Prisma, SQLAlchemy] - Modelado de datos y migraciones. | `docs/skills/DB.md` |
| `[ESTILOS]` | [Ej: Tailwind, CSS Modules] - Sistema de diseño y tokens. | `docs/skills/STYLES.md` |

### 🧠 Habilidades de Dominio (Lógica de Kauiros)
*Reglas específicas de negocio y arquitectura de este proyecto.*

| Habilidad | Descripción | Archivo de Referencia |
| :--- | :--- | :--- |
| `kauiros-core` | Arquitectura hexagonal/limpia, entidades principales. | `docs/skills/CORE.md` |
| `kauiros-auth` | Flujos de autenticación, roles y permisos. | `docs/skills/AUTH.md` |
| `kauiros-ui` | Biblioteca de componentes visuales propios. | `docs/skills/UI.md` |
| `kauiros-algo` | [Si aplica] Algoritmos matemáticos o de optimización específicos. | `docs/skills/ALGO.md` |

---

## 🤖 Disparadores Automáticos (Auto-invoke)
**REGLA DE ORO:** Antes de realizar una acción de la columna izquierda, el agente o desarrollador DEBE cargar el contexto de la habilidad a la derecha.

### 🏗 Desarrollo & Arquitectura
| Acción (Lo que vas a hacer) | Habilidad Requerida (Lo que debes leer antes) |
| :--- | :--- |
| Crear nuevos modelos de base de datos | `[DB_ORM]` |
| Crear nuevos endpoints o rutas | `[FRAMEWORK_BACK]` + `kauiros-core` |
| Diseñar nuevas pantallas/vistas | `kauiros-ui` + `[ESTILOS]` |
| Implementar lógica de negocio compleja | `kauiros-core` |
| Modificar el sistema de usuarios | `kauiros-auth` |

### 🧪 Calidad & Testing
| Acción | Habilidad Requerida |
| :--- | :--- |
| Escribir tests unitarios | `[TESTING_LIB]` |
| Escribir tests de integración (E2E) | `[E2E_LIB]` |
| Reportar un bug | `kauiros-docs` |

### 🚀 Despliegue & Mantenimiento (DevOps)
| Acción | Habilidad Requerida |
| :--- | :--- |
| Crear un Pull Request | `kauiros-git-flow` |
| Actualizar documentación | `kauiros-docs` |
| Modificar CI/CD Pipelines | `kauiros-ci` |

---

## 🗺 Visión General del Proyecto
**Kauiros** es [Describe aquí en una frase la misión del proyecto].

### Estructura de Directorios
*Ubicación de los componentes clave para facilitar la navegación rápida.*

| Directorio | Propósito | Tecnologías Clave |
| :--- | :--- | :--- |
| `/apps` | Aplicaciones finales (Web, Móvil, API) | [Tech] |
| `/packages` | Librerías compartidas y utilidades | [Tech] |
| `/docs` | Documentación y definición de Habilidades | Markdown |
| `/infra` | Configuración de nube y despliegue | Terraform/Docker |

---

## ⚡ Flujo de Trabajo (Workflow)
1.  **Instalación:** `[Comando de instalación]`
2.  **Arrancar entorno:** `[Comando de start]`
3.  **Tests:** `[Comando de test]`

**Guía de Commits:** Usamos *Conventional Commits* (`feat`, `fix`, `chore`).