# rfd-api: Oxide Computer 'Requests for Discussion' API

## Cargo commands

- check: `cargo check --quiet --all-features --workspace --all-targets`
- test: `cargo test --quiet --all-features --workspace --all-targets`
- format: `cargo fmt`
- clippy: `cargo clippy --quiet --fix --allow-dirty --all-features --workspace --all-targets`
- generate: `cargo xtask --quiet generate` (regen OpenAPI, Rust/TS SDKs, CLI)

## Crates

- `parse-rfd`: Lib crate that wraps document parser for parsing source files
- `rfd-api`: Dropshot API server crate for RFD endpoints, auth, and permissions
- `rfd-cli`: CLI crate for calling the `rfd-api` service
- `rfd-data`: Shared data types and content helpers for the workspace
- `rfd-github`: GitHub client crate for reading and updating RFD source
- `rfd-installer`: Migration runner crate for `rfd-model` and `v-api`
- `rfd-model`: Core RFD models, Diesel schema, and storage types
- `rfd-processor`: Worker crate for scanning and processing RFD updates
- `rfd-sdk`: Generated Rust SDK crate for the RFD API
- `trace-request`: Proc-macro crate for tracing Dropshot handlers
- `xtask`: Workspace task crate for version bumps and code generation

### Non-Cargo subprojects

- `rfd-ts` — TypeScript client generated from the OpenAPI spec (`@oxide/rfd.ts`)
  Optional MSW handlers and Zod validation.
- `remix-auth-rfd` — Remix Auth strategies package (`@oxide/remix-auth-rfd`)
  Built on top of `rfd-ts`, currently exporting both magic-link and OAuth flows.
