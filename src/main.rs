mod algorithms;
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

    // --- Deutsch-Jozsa ---
    println!("\n=== Deutsch-Jozsa (n=2) ===");
    // Constant oracle: does nothing (f(x) = 0 for all x)
    let constant = algorithms::deutsch_jozsa(2, |_sim| {});
    println!("  Constant oracle -> {}", if constant { "constant" } else { "balanced" });
    // Balanced oracle: f(x) = x0 (CNOT from qubit 0 to ancilla)
    let balanced = algorithms::deutsch_jozsa(2, |sim| { sim.cnot(0, 2); });
    println!("  Balanced oracle -> {}", if balanced { "constant" } else { "balanced" });

    // --- Bernstein-Vazirani ---
    println!("\n=== Bernstein-Vazirani ===");
    let secret = vec![1u8, 0, 1, 1];
    let recovered = algorithms::bernstein_vazirani(&secret);
    println!("  Secret:    {:?}", secret);
    println!("  Recovered: {:?}", recovered);

    // --- Grover's search (2 qubits, target |11⟩ = index 3) ---
    println!("\n=== Grover's Search (2 qubits, target |11⟩) ===");
    let probs = algorithms::grover(2, |sim| { sim.cz(0, 1); });
    for (i, p) in probs.iter().enumerate() {
        println!("  |{:02b}⟩  p = {:.4}", i, p);
    }
}
