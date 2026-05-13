# Operiva

B2B SaaS para gestão de empresas — foco em operações e produtos.

> **Status:** Em desenvolvimento inicial. API de autenticação sendo construída como primeira fatia.

## Stack

- **Backend:** Rust (Axum, SQLx, Tokio)
- **Banco:** PostgreSQL 16+
- **Arquitetura:** workspace Cargo com 5 crates (clean architecture / DDD tático)

## Estrutura do workspace

```
crates/
├── domain/          # Tipos puros, sem I/O ou async
├── application/     # Casos de uso e ports (traits)
├── infrastructure/  # Adaptadores concretos (Postgres, Resend, JWT, Argon2)
├── api/             # Camada HTTP (Axum)
└── server/          # Composition root (binário)
```

## Desenvolvimento

Requisitos: Rust stable ≥ 1.78, PostgreSQL 16+, `sqlx-cli`, `cargo-deny`.

```bash
# Criar banco de testes local
createdb operiva_test
export DATABASE_URL=postgres://localhost/operiva_test

# Aplicar migrations (quando existirem)
sqlx migrate run --source migrations

# Rodar testes
cargo test --workspace

# Lints
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo deny check
```
