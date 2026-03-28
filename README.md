# Quantum Simulator — Rust

A statevector quantum computing simulator built from scratch in Rust, with quantum algorithm implementations and interactive visualization.

## Features

- **Statevector simulator** — exact simulation of n-qubit pure states
- **Gates** — H, X, Y, Z, S, T, Rx, Ry, Rz, CNOT, CZ, Toffoli
- **Measurement** — projective measurement with state collapse and renormalization
- **Algorithms** — Deutsch-Jozsa, Bernstein-Vazirani, Grover's search
- **Visualization** — interactive Bloch sphere (Plotly, opens in browser)
- **Demos** — CHSH Bell inequality test, Grover scaling table

## Getting Started

```sh
cargo run
```

Runs all demos in the terminal and opens the Bloch sphere in your browser.

## Structure

```
src/
├── simulator.rs   # Statevector simulator core
├── gates.rs       # Single-qubit gate definitions
├── algorithms.rs  # Quantum algorithms + terminal demos
├── viz.rs         # Interactive Bloch sphere (Plotly)
└── main.rs        # Entry point
```

## Demos

### Bell Inequality (CHSH)
Measures correlations between entangled qubits at four angle pairs and computes the CHSH value S. Demonstrates quantum violation of the classical bound |S| ≤ 2, approaching the quantum maximum of 2√2 ≈ 2.828.

### Grover Scaling
Compares classical search (N/2 average queries) against Grover's algorithm (⌈π/4 · √N⌉ queries) from 2 to 8 qubits, showing the quadratic speedup grow to ~10x at 8 qubits.

### Bloch Sphere
Plots single-qubit states as vectors on the unit sphere — including entangled qubits, whose Bloch vector collapses to the origin (maximally mixed state).

## Dependencies

- [`num-complex`](https://crates.io/crates/num-complex) — complex number arithmetic
- [`rand`](https://crates.io/crates/rand) — measurement randomness
- [`plotly`](https://crates.io/crates/plotly) — interactive 3D visualization
