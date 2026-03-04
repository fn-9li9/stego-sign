# stego-app

Frontend SSR/WASM para StegoSign, construido con [Leptos](https://leptos.dev) 0.8 y Axum.

## Stack

- **Leptos 0.8** — framework full-stack Rust con SSR + hydration
- **Axum** — servidor HTTP para SSR y proxy de API
- **WASM** — cliente compilado a WebAssembly via `wasm32-unknown-unknown`
- **Tailwind CSS** — estilos via CDN browser build
- **gloo-net** — cliente HTTP para llamadas desde el browser

## Arquitectura

```
browser
  │
  └─► localhost:3000  (stego-app — Leptos SSR)
        │
        └─► /api/*  →  proxy interno  →  localhost:4000  (stego-server)
```

El browser **nunca habla directamente con el server**. Todas las llamadas a `/api/*`
pasan por el proxy interno del app, que las reenvía al server en la red interna.

### Variables de entorno clave

| Variable | Uso | Ejemplo |
|---|---|---|
| `API_BASE_URL` | SSR → server (solo en el proceso Rust) | `http://localhost:4000` |
| `PUBLIC_API_BASE_URL` | CSR → proxy del app (inyectado en meta tag) | `http://localhost:3000` |
| `LEPTOS_SITE_ADDR` | Dirección donde escucha el app | `127.0.0.1:3000` |

## Desarrollo local

### Prerequisitos

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

### Setup

```bash
cp .env.example .env
# edita .env con tus valores
```

Asegúrate de que el server esté corriendo en `localhost:4000`:

```bash
# en /server
cargo run
```

### Correr el app

```bash
cargo leptos watch
```

Abre [http://localhost:3000](http://localhost:3000).

## Build para producción

```bash
cargo leptos build --release
```

El binario queda en `target/release/stego-app` y los assets en `target/site/`.

## Docker

El app se construye y sirve via Docker. Los ARGs de build son provistos por el
`docker-compose.yml` del proyecto raíz:

```yaml
args:
  API_BASE_URL:        http://server:4000
  PUBLIC_API_BASE_URL: http://localhost:${APP_PORT}
```

> `API_BASE_URL` se usa en compile-time solo como fallback. En runtime
> el valor real viene del `.env` o del compose via variable de entorno.

### Levantar solo el app con Docker Compose

```bash
# desde la raíz del proyecto
docker compose up -d app
```

## Estructura

```
src/
├── main.rs          # entry point SSR + proxy /api/*
├── lib.rs           # entry point WASM (hydrate)
├── app.rs           # shell HTML + router principal
├── config.rs        # api_base_url() — SSR lee env, CSR lee meta tag
├── features/
│   ├── home/        # página principal + stats
│   ├── sign/        # firma de documentos
│   ├── verify/      # verificación de integridad
│   └── documents/   # listado y descarga de documentos
└── shared/
    ├── components/  # navbar, footer
    └── models.rs    # tipos compartidos
```

## Proxy de API

El `main.rs` registra una ruta `/api/*` que reenvía todas las peticiones al server:

```
GET  /api/v1/stats      →  http://server:4000/api/v1/stats
POST /api/v1/sign       →  http://server:4000/api/v1/sign
POST /api/v1/verify     →  http://server:4000/api/v1/verify
GET  /api/v1/documents  →  http://server:4000/api/v1/documents
```

Esto elimina la necesidad de CORS y mantiene el server fuera de la red pública.