# kairos-web - Trading Dashboard (Angular)

Dashboard web para visualizar mercados, gestionar estrategias y monitorear el sistema KAIRÓS en tiempo real.

---

## 📖 Descripción

**kairos-web** es una aplicación Angular 21 que proporciona:
- Visualización de precios en tiempo real
- Gestión de órdenes de trading
- Monitoreo de estrategias activas
- Configuración del sistema

---

## 🚀 Instalación y Configuración

### Prerequisitos

- **Node.js 18+**
- **npm 9+**
- **kairos-api** corriendo en http://localhost:4000

### Configurar e Instalar

```bash
cd apps/kairos-web

# Instalar dependencias
npm install

# Ejecutar en desarrollo
npm start

# Abrir navegador en http://localhost:4200
```

### Build de Producción

```bash
npm run build

# Archivos generados en dist/kairos-web/
```

---

## 🧪 Testing

```bash
# Tests unitarios
npm test

# Tests e2e
npm run e2e

# Coverage
npm run test:coverage
```

Ver [TESTING.md](./TESTING.md) para más detalles.

---

## 🐳 Docker

```bash
# Desde la raíz del proyecto
docker build -f infrastructure/docker/Dockerfile.web -t kairos-web:latest .

# Ejecutar
docker run -p 4200:80 kairos-web:latest
```

---

## 📚 Referencias

- [Angular Docs](https://angular.io/docs)
- [RxJS Guide](https://rxjs.dev/guide/overview)

---

**Mantenido por:** KAIRÓS Team  
**Última actualización:** 2026-01-06
