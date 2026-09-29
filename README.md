# Rust Systems Component

Self-directed exploration of Rust's memory-safety model via a data-processing utility.

## Stack
- **Rust**: Ownership model, compile-time safety guarantees
- **Python**: Equivalent implementation for comparison

## Run

```bash
# Rust
cargo run --release

# Python
python3 processor.py
```

## What This Shows
- `src/main.rs`: Rust implementation using ownership to prevent use-after-free at **compile time**
- `processor.py`: Python equivalent showing runtime-only safety (garbage collection)
- `ANALYSIS.md`: Tradeoffs in memory guarantees vs iteration speed

**Demonstrates**: Understanding Rust's core value—eliminating memory-safety bugs at compile time—and honest comparison of language tradeoffs without overclaiming production experience.
