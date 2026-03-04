mod gates;
mod simulator;

use simulator::Simulator;

fn main() {
    // --- Bell state |Φ+⟩ = (|00⟩ + |11⟩) / √2 ---
    println!("=== Bell State |Φ+⟩ ===");
    let mut bell = Simulator::new(2);
    bell.apply(gates::hadamard(), 0);
    bell.cnot(0, 1);
    bell.print_state();

    println!("\nMeasure qubit 0...");
    let r = bell.measure(0);
    println!("  Result: {r}");
    bell.print_state(); // collapses to |00⟩ or |11⟩ with certainty

    // --- GHZ state (|000⟩ + |111⟩) / √2 ---
    println!("\n=== GHZ State (3 qubits) ===");
    let mut ghz = Simulator::new(3);
    ghz.apply(gates::hadamard(), 0);
    ghz.cnot(0, 1);
    ghz.cnot(0, 2);
    ghz.print_state();

    // --- Single-qubit rotations ---
    println!("\n=== Rx(π) on |0⟩ ≈ -i|1⟩ ===");
    let mut q = Simulator::new(1);
    q.apply(gates::rx(std::f64::consts::PI), 0);
    q.print_state();

    // --- Uniform superposition constructor ---
    println!("\n=== Uniform superposition (new_uniform, 2 qubits) ===");
    let uniform = Simulator::new_uniform(2);
    uniform.print_probabilities();
}
