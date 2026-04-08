# Provable Order Book Matching (RISC Zero)

A minimal Central Limit Order Book (CLOB) matching engine running inside the RISC Zero zkVM. The guest program proves that a batch of trades was matched correctly, and commits a SHA-256 state root of the post-match order book to the journal.

## Architecture

```text
training_r0
├── core/          Shared types (Order, OrderBook, Trade, Input, Output)
├── host/          Builds input, drives the prover, verifies the receipt
└── methods/
    └── guest/     Matching engine running inside the zkVM
```

### Host / Guest split

- **Host** constructs an `Input` (current `OrderBook` + new orders), sends it to the guest, then reads back the trades and updated book via the private stdout channel.
- **Guest** inserts new orders into BTreeMap-based bid/ask books, runs price-time priority matching, and:
  - Commits a SHA-256 hash of the post-match order book to the **journal** (public, proven).
  - Writes the trades and updated order book back to the host via `env::write()` (private).
- **Host** verifies the receipt and checks that the hash of the received book matches the journal commitment.

### Key design decisions

- **BTreeMap<u64, VecDeque<Order>>** for deterministic iteration (no HashMap in the guest).
- **Price-time priority**: bids matched highest-first (`last_entry`), asks lowest-first (`first_entry`), FIFO within each price level.
- **State root pattern**: only a 32-byte hash is committed to the journal; full state is sent privately.
- **Pre-allocated Vecs** and no `.clone()` on the hot path to minimize cycle count.

## Running

### Development mode (fast, no real proof)

```bash
RISC0_DEV_MODE=1 cargo run
```

### Real proving (with cycle count)

```bash
RISC0_DEV_MODE=0 cargo run
```
