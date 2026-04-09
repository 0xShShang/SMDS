
# SMDS: Secure Multi-party Dataset Sorting

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust implementation of the Secure Multi-party Dataset Sorting (SMDS) protocol, enabling privacy-preserving ranking computations across multiple parties without revealing individual data values.

## 🚀 Features

- **Privacy-Preserving Ranking**: Secure multi-party computation for dataset ranking
- **Multiple Backends**: Local reference backend and rust-multi-she compatibility
- **Comprehensive Benchmarking**: Built-in performance evaluation framework
- **Flexible Scenarios**: Support for various dataset distribution patterns
- **Type-Safe Implementation**: Full Rust type safety with zero-cost abstractions

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Repository Structure](#repository-structure)
- [Benchmarking](#benchmarking)
- [Development](#development)
- [License](#license)

## 🛠️ Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

### Clone and Build

```bash
git clone https://github.com/0xShShang/SMDS.git
cd SMDS
cargo build --workspace
```

## 🏃‍♂️ Quick Start

### Basic Demo

```bash
# Run demo with local backend
cargo run -p smds_cli -- demo ../班加罗尔房地产价格数据集.csv

# Run demo with rust-multi-she backend
cargo run -p smds_cli -- demo ../班加罗尔房地产价格数据集.csv rust
```

### Run Tests

```bash
# Run all tests across workspace
cargo test --workspace

# Run tests with detailed output
cargo test --workspace -- --nocapture
```

## 📖 Usage

### CLI Commands

The SMDS CLI provides several commands for different use cases:

#### Demo Commands

```bash
# Basic demo with default parameters
cargo run -p smds_cli -- demo <csv-path>

# Demo with specific backend
cargo run -p smds_cli -- demo <csv-path> <backend>
# Backend options: local | rust
```

#### Reference Implementation

```bash
# Run plaintext reference ranking
cargo run -p smds_cli -- reference <csv-path>
```

#### Benchmark Commands

```bash
# Benchmark with default scenario
cargo run -p smds_cli -- benchmark <csv-path>

# Benchmark with specific scenario and backend
cargo run -p smds_cli -- benchmark <csv-path> <scenario> <backend>

# Scenario options:
# - baseline: Bangalore baseline scenario
# - overlap: Multi-user overlap scenario
# - disjoint: Multi-user disjoint scenario
# - duplicate: Duplicate-heavy scenario
# - single: Single user scenario
```

### Advanced Usage

#### Enable rust-multi-she Backend

```bash
# Compile with rust-multi-she backend feature
cargo run -p smds_cli --features backend-rust-multi-she -- benchmark <csv-path> baseline rust
```

#### Custom Parameters

```bash
# Example: Run benchmark with custom scenario
cargo run -p smds_cli -- benchmark ../班加罗尔房地产价格数据集.csv overlap local
```

## 📁 Repository Structure

```
smds/
├── crates/
│   ├── smds_types/           # Shared protocol and benchmark types
│   ├── smds_crypto_utils/    # CSV loading, sampling, polynomial helpers
│   ├── multi_she_adapter/    # Local backend and backend trait
│   ├── smds_core/           # Reference ranking and protocol pipeline
│   ├── smds_user/           # User-side preparation helpers
│   ├── smds_server/         # Server-side orchestration helpers
│   ├── smds_bench/          # Benchmark report builder
│   └── smds_cli/           # Binary entry point
├── vendor/                  # Vendored dependencies
├── Cargo.toml              # Workspace configuration
└── README.md               # This file
```

### Crate Overview

| Crate | Description |
|-------|-------------|
| `smds_types` | Core data structures and protocol types |
| `smds_crypto_utils` | Cryptographic utilities and data processing |
| `multi_she_adapter` | Backend abstraction and implementations |
| `smds_core` | Main SMDS protocol implementation |
| `smds_user` | User-side operations and helpers |
| `smds_server` | Server-side orchestration |
| `smds_bench` | Benchmarking framework and reporting |
| `smds_cli` | Command-line interface |

## 📊 Benchmarking

### Benchmark Scenarios

1. **BangaloreBaseline**: Baseline scenario using Bangalore real estate data
2. **MultiUserOverlap**: Multiple users with overlapping data
3. **MultiUserDisjoint**: Multiple users with disjoint data
4. **DuplicateHeavy**: Scenario with many duplicate values
5. **SingleUser**: Single user scenario

### Example Benchmark Output

```json
{
  "config": {
    "scenario": "BangaloreBaseline",
    "seed": 42,
    "repetitions": 3,
    "num_users": 3,
    "dataset_size": 8
  },
  "dataset_summary": {
    "total_values": 24,
    "unique_values": 20,
    "min_value": 23,
    "max_value": 410
  },
  "stage_durations": [
    {"stage": "offline_bootstrap", "millis": 0},
    {"stage": "pbe_encode", "millis": 0},
    {"stage": "server_stage", "millis": 0},
    {"stage": "encrypt_reference_ranks", "millis": 0},
    {"stage": "recovery", "millis": 0}
  ],
  "correctness": true
}
```

### Performance Metrics

The benchmark measures:
- **Offline Bootstrap**: Preprocessing and setup time
- **PBE Encode**: Privacy-preserving encoding time
- **Server Stage**: Server-side computation time
- **Encryption**: Reference rank encryption time
- **Recovery**: Rank recovery time

## 🔧 Development

### Building

```bash
# Build all crates
cargo build --workspace

# Build with optimizations
cargo build --workspace --release

# Build with specific features
cargo build --workspace --features backend-rust-multi-she
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p smds_core

# Run benchmarks
cargo test --workspace --release --benches
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run clippy lints
cargo clippy --workspace -- -D warnings

# Check documentation
cargo doc --workspace --no-deps --open
```

## 📚 Protocol Overview

SMDS implements a secure multi-party computation protocol for:

1. **Data Preprocessing**: Integerization and canonicalization of input values
2. **Privacy-Preserving Encoding**: Masked value encoding with permutations
3. **Secure Ranking**: Multi-party computation of global rankings
4. **Result Recovery**: Secure recovery of individual rankings

### Security Properties

- **Privacy**: Individual values remain hidden from other parties
- **Correctness**: Computed rankings match plaintext reference implementation
- **Verifiability**: Results can be verified against reference implementation

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Based on the SMDS (Secure Multi-party Dataset Sorting) protocol research
- Built with Rust's ecosystem for secure multi-party computation
- References to the original rust-multi-she implementation

## 📞 Contact

For questions and support, please open an issue on GitHub or contact the maintainers.

---

**Note**: This is a research implementation. For production use, please ensure thorough security auditing and testing.

