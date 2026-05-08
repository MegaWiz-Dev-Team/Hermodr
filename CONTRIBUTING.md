# Contributing to Hermóðr

Hermóðr is part of the [Asgard AI Platform](https://github.com/MegaWiz-Dev-Team/Asgard). For the high-level workflow, CLA, and code of conduct, see [Asgard's CONTRIBUTING.md](https://github.com/MegaWiz-Dev-Team/Asgard/blob/main/CONTRIBUTING.md).

## This repo specifically

### Layout

- `src/` — Rust JSON-RPC server (`jsonrpc.rs`, `proxy.rs`, `services/*.rs`)
- `services/<name>.rs` — per-service tool definitions (Eir/FHIR, Yggdrasil/Auth, Wazuh/SIEM, etc.)

### Development setup

```bash
cargo build --release

# Run for a specific upstream
SERVICE_NAME=yggdrasil UPSTREAM_URL=http://localhost:8085 PORT=8090 \
  ./target/release/hermodr
```

### Running tests

```bash
cargo test
```

### Adding a new service

1. Create `src/services/<your_service>.rs` with `tools()` and per-tool handlers
2. Register the module in `src/services/mod.rs`
3. Add a branch in the dispatcher (`src/main.rs` or `src/dispatcher.rs`)
4. Add tests under `#[cfg(test)] mod tests` in the service file

### Style

- `cargo fmt` + `cargo clippy --all-targets -- -D warnings`
- Conventional Commits (`feat:`, `fix:`, `docs:`, etc.)

### Reporting issues

- 🐛 Bugs: open an issue with the bug report template
- 💡 Features: open an issue with the feature request template
- 🔒 Security: see [SECURITY.md](SECURITY.md) (do **not** open public issues)

### License & CLA

By contributing, you agree to license your contribution under [AGPL-3.0](LICENSE) and the [Asgard CLA](https://github.com/MegaWiz-Dev-Team/Asgard/blob/main/CLA.md). Your first PR serves as your electronic signature.
