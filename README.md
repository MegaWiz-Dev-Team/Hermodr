# Hermóðr — Universal MCP Sidecar

> *Hermóðr, the messenger of the gods, who travels between realms.*

Lightweight Rust JSON-RPC 2.0 bridge that wraps any REST service as [MCP (Model Context Protocol)](https://modelcontextprotocol.io/) tools for the **Asgard AI Platform**.

### 🏥 Role in Multi-Agent Ecosystem

> **Universal MCP Sidecar (ผู้ส่งสาร)** — Hermóðr ครอบ Legacy Service (Eir, Heimdall, Yggdrasil) ให้พูดภาษา MCP ได้ทันที โดยไม่ต้องแก้โค้ดเดิม — เป็น Bridge ระหว่าง REST API กับ MCP Protocol
>
> **Bridges:** Eir (FHIR) • Heimdall (LLM) • Yggdrasil (Auth)
>
> 📖 [Full Architecture →](https://github.com/MegaWiz-Dev-Team/Asgard/blob/main/docs/roadmap/MultiAgent_Architecture_Plan.md) | [Sprint Plan →](https://github.com/MegaWiz-Dev-Team/Asgard/blob/main/docs/roadmap/MultiAgent_Sprint_Plan.md)

## Architecture

```
Bifrost (MCP Client) → Hermóðr (:8090/rpc) → Upstream REST API
                        JSON-RPC 2.0            HTTP GET/POST
```

Hermóðr receives MCP `tools/list` and `tools/call` JSON-RPC requests, translates them into standard REST API calls, and returns the results as MCP content.

## Supported Services

| Service | Env `SERVICE_NAME` | Tools | Upstream |
|---------|-------------------|-------|----------|
| **Yggdrasil** (Auth) | `yggdrasil` | `validate_token`, `get_user_roles` | Zitadel API |
| **Eir** (OpenEMR) | `eir` | `get_patient_medical_history`, `book_appointment`, `search_patients`, `get_patient_summary`, `create_encounter`, `get_sleep_reports` | Eir Gateway |

## Quick Start

```bash
# Build
cargo build --release

# Run for Yggdrasil
SERVICE_NAME=yggdrasil UPSTREAM_URL=http://localhost:8085 PORT=8090 ./target/release/hermodr

# Run for Eir
SERVICE_NAME=eir UPSTREAM_URL=http://localhost:8300 PORT=8091 ./target/release/hermodr
```

## Docker

```bash
docker build -t hermodr .

# Yggdrasil sidecar
docker run -e SERVICE_NAME=yggdrasil -e UPSTREAM_URL=http://yggdrasil:8080 -p 8090:8090 hermodr

# Eir sidecar
docker run -e SERVICE_NAME=eir -e UPSTREAM_URL=http://eir-gateway:8300 -p 8091:8091 hermodr
```

## Configuration

| Env Var | Default | Description |
|---------|---------|-------------|
| `SERVICE_NAME` | `mcp-sidecar` | Service to load tools for (`yggdrasil`, `eir`) |
| `UPSTREAM_URL` | `http://localhost:8080` | Base URL for upstream REST API |
| `PORT` | `8090` | Listen port |
| `RUST_LOG` | `info` | Log level |

## Testing

```bash
cargo test
```

## MCP Protocol

Hermóðr implements [MCP JSON-RPC 2.0](https://modelcontextprotocol.io/):

- `POST /rpc` — JSON-RPC endpoint
  - `initialize` — Protocol handshake
  - `tools/list` — Discover available tools
  - `tools/call` — Execute a tool
- `GET /health` — Health check

## Part of the Asgard AI Platform

| Service | Role |
|---------|------|
| **Mimir** | Knowledge Engine (Rust) |
| **Bifrost** | Agent Runtime (Python) |
| **Hermóðr** | MCP Sidecar (Rust) ← you are here |
| **Heimdall** | LLM Server (Python/MLX) |
| **Yggdrasil** | Auth (Zitadel) |
| **Eir** | OpenEMR (PHP) |
| **Fenrir** | Browser Agent (Python) |

## License

Licensed under the **GNU Affero General Public License v3.0** — see [LICENSE](./LICENSE).

A commercial license is available for organisations who cannot comply with AGPL-3.0; contact `paripol@megawiz.co` for terms.
