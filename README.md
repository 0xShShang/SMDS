<<<<<<< HEAD
# SMDS Workspace

This workspace contains the first reproducible skeleton for the SMDS project.

Current baseline:

- Dataset: `../班加罗尔房地产价格数据集.csv`
- Input field: `price`
- Domain upper bound: `M = 3600`
- Reference backend commit: `59badffc47656a41077dc3cae2a89b074991ca3e`
- Local adapter backend: deterministic reference implementation shaped after `rust-multi-she`

What is implemented now:

- CSV price-column loader
- deterministic dataset sampling
- polynomial helpers
- plaintext dedup-rank reference
- adapter trait and a concrete local backend for the two-party Multi-SHE shape
- user/server/core/bench crates for the SMDS pipeline
- CLI entry point for demo/reference/benchmark runs

Feature-gated backend:

- Enable `multi_she_adapter/backend-rust-multi-she` to switch from the local reference backend to the `rust-multi-she` compatibility binding.
- The workspace patches the public git dependency to a local compatibility copy because the upstream `HEAD` currently contains an unresolved merge conflict.

Repository layout:

- `crates/smds_types`: shared protocol and benchmark types
- `crates/smds_crypto_utils`: CSV loading, sampling, and polynomial helpers
- `crates/multi_she_adapter`: local backend and backend trait
- `crates/smds_core`: reference ranking and protocol pipeline
- `crates/smds_user`: user-side preparation helpers
- `crates/smds_server`: server-side orchestration helpers
- `crates/smds_bench`: benchmark report builder
- `crates/smds_cli`: binary entry point

Quick checks:

```bash
cd smds
cargo test --workspace
```

Example CLI usage:

```bash
cargo run -p smds_cli -- demo ../班加罗尔房地产价格数据集.csv
cargo run -p smds_cli -- demo ../班加罗尔房地产价格数据集.csv rust
cargo run -p smds_cli -- reference ../班加罗尔房地产价格数据集.csv
cargo run -p smds_cli -- benchmark ../班加罗尔房地产价格数据集.csv
cargo run -p smds_cli -- benchmark ../班加罗尔房地产价格数据集.csv disjoint rust
```

To enable the `rust-multi-she` compatibility backend at compile time:

```bash
cargo run -p smds_cli --features backend-rust-multi-she -- benchmark ../班加罗尔房地产价格数据集.csv baseline rust
```

Next step:

1. Add more benchmark scenarios beyond the Bangalore baseline.
2. Replace the local backend with a real binding layer when the upstream API is frozen.
=======
# SMDS
>>>>>>> 356065fc94e6f087a4cfc72b76b9788f29a66fbf
