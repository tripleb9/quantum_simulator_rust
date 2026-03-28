mod algorithms;
mod gates;
mod simulator;
mod viz;

use simulator::Simulator;
use viz::{BlochState, plot_bloch_sphere};

fn main() {
    // ── Terminal demos ───────────────────────────────────────────────────────
    println!();
    algorithms::demo_chsh(2000);
    println!();
    algorithms::demo_grover_scaling();
    println!();

    // ── Bloch sphere: key single-qubit states ────────────────────────────────
    // Show |0⟩, |+⟩, and the result of Rx(π/3) all on one sphere
    let states = vec![
        {
            let sim = Simulator::new(1);                          // |0⟩
            let v = sim.bloch_vector(0);
            BlochState::new("|0⟩ (ground)", v.0, v.1, v.2)
        },
        {
            let mut sim = Simulator::new(1);
            sim.apply(gates::hadamard(), 0);                      // |+⟩
            let v = sim.bloch_vector(0);
            BlochState::new("|+⟩ = H|0⟩", v.0, v.1, v.2)
        },
        {
            let mut sim = Simulator::new(1);
            sim.apply(gates::rx(std::f64::consts::PI / 3.0), 0); // Rx(π/3)
            let v = sim.bloch_vector(0);
            BlochState::new("Rx(π/3)|0⟩", v.0, v.1, v.2)
        },
        {
            let mut sim = Simulator::new(2);
            sim.apply(gates::hadamard(), 0);
            sim.cnot(0, 1);                                        // entangled qubit 0
            let v = sim.bloch_vector(0);
            BlochState::new("qubit 0 of |Φ+⟩ (mixed)", v.0, v.1, v.2)
        },
    ];

    println!("Opening Bloch sphere in browser...");
    plot_bloch_sphere(&states, "Bloch Sphere — Quantum Rust2");
}
