# Sladrehank (Gossip Glomers) Workloads 🚀

**Short:** A collection of Rust maelstrom workloads and helpers for running Maelstrom-style distributed systems tests and experiments. This repository includes example workloads (echo, broadcast, g-counter, kafka, unique-ids, etc.) and helper scripts to run them locally or in distributed environments such as Fly.io's dist-sys tests.

---

## Overview

This repository contains Rust implementations of several workloads designed to be exercised by Jepsen/Maelstrom-style chaos testing tooling. The project includes a small bundled `maelstrom` runner (in `maelstrom/`) and a set of binaries located under `src/bin/` for different workloads.

The repository is compatible with Fly.io's dist-sys tasks (https://fly.io/dist-sys), which provide a way to run distributed-system test scenarios.

---

## Requirements

- Rust toolchain (stable) with `cargo`
- `just` (optional but recommended) — recipes provided in the `Justfile`
- `maelstrom` test runner (binary included at `./maelstrom/maelstrom`)

---

## Quick Start

1. Build the project and binaries:

```bash
just build
# or
cargo build --bins
```

2. Run a ready-made example (uses the `Justfile` recipes):

```bash
just echo
just broadcast-multi
just g_counter
# Start the maelstrom server locally
just serve
```

Each `just` recipe runs `maelstrom test` with a particular workload and options; inspect the `Justfile` for available recipes and parameters.

---

## Running Maelstrom tests

You can run Maelstrom tests manually using the `maelstrom` runner in this repo. Example:

```bash
./maelstrom/maelstrom test --workload echo --bin ./target/debug/echo --node-count 3 --time-limit 10
```

Adjust `--node-count`, `--time-limit`, `--rate`, or `--nemesis` according to your test plan. The `Justfile` contains convenience recipes for common scenarios.

---

## Project structure

- `src/` — Rust source, including `src/bin` with workload binaries
- `justfile` — convenience recipes to build & run workloads

---

## Contributing

Contributions welcome — please open issues or PRs for new workloads, bug fixes, or improvements. When adding workloads, include `Justfile` recipes that make them easy to run locally and in the dist-sys environment.

---

## License

This repository includes a `LICENSE` file at the project root. Please review it before contributing.

---
