# Justfile for running Maelstrom "Gossip Glomers" style tests
# Usage: `just <recipe>`

set shell := ["bash", "-cu"]

# Paths (adjust if needed)
MAELSTROM := "./maelstrom/maelstrom"


# Build recipes
build:
	@echo "Building project (debug binaries)"
	cargo build --bins

build-release:
	@echo "Building project (release)"
	cargo build --release --bins

echo: build
	@echo "Running gossip-echo (3 nodes, 10s)"
	{{MAELSTROM}} test --workload "echo" --bin "./target/debug/echo" --node-count 1 --time-limit 10

uids: build
    @echo "Running gossip-uids (3 nodes, 10s)"
    {{MAELSTROM}} test --workload "unique-ids" --bin "./target/debug/uids" --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

# broadcast
BROADCAST_BIN := "./target/debug/broadcast"
broadcast-single: build
    @echo "Running gossip-sanity (3 nodes, 10s)"
    {{MAELSTROM}} test --workload broadcast --bin {{BROADCAST_BIN}} --node-count 1 --time-limit 20 --rate 10
broadcast-multi: build
    @echo "Running gossip-sanity (3 nodes, 10s)"
    {{MAELSTROM}} test --workload broadcast --bin {{BROADCAST_BIN}} --node-count 5 --time-limit 20 --rate 10
broadcast-partition: build
    @echo "Running gossip-sanity (3 nodes, 10s)"
    {{MAELSTROM}} test --workload broadcast --bin {{BROADCAST_BIN}} --node-count 5 --time-limit 20 --rate 10 --nemesis partition
broadcast-efficient: build
    @echo "Running gossip-sanity (3 nodes, 10s)"
    {{MAELSTROM}} test --workload broadcast --bin {{BROADCAST_BIN}} --node-count 25 --time-limit 20 --rate 100 --latency 100


g_counter: build
    @echo "Running g-counter (3 nodes, 20s)"
    {{MAELSTROM}} test -w g-counter --bin "./target/debug/g_counter" --node-count 3 --rate 100 --time-limit 20 --nemesis partition



serve:
    @echo "Starting maelstrom server"
    {{MAELSTROM}} serve 