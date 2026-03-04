use crate::gates;
use crate::simulator::Simulator;

/// Deutsch-Jozsa: determines if a function is constant or balanced in one query.
/// Uses n+1 qubits: qubits 0..n-1 are input, qubit n is the |-> ancilla.
/// `oracle` must flip the ancilla (qubit n) conditioned on the input.
/// Returns true if constant, false if balanced.
pub fn deutsch_jozsa(n: usize, oracle: impl Fn(&mut Simulator)) -> bool {
    let mut sim = Simulator::new(n + 1);
    // Prepare ancilla as |->
    sim.apply(gates::pauli_x(), n);
    sim.apply(gates::hadamard(), n);
    // Hadamard on input qubits
    for q in 0..n {
        sim.apply(gates::hadamard(), q);
    }
    oracle(&mut sim);
    // Hadamard on input qubits again
    for q in 0..n {
        sim.apply(gates::hadamard(), q);
    }
    // Constant iff all input qubits measure 0
    (0..n).all(|q| sim.measure(q) == 0)
}

/// Bernstein-Vazirani: recovers a hidden bitstring `s` in one query.
/// Uses n+1 qubits; the oracle computes f(x) = s·x (mod 2).
/// `secret` is a slice of 0/1 values, one per input qubit (LSB first).
/// Returns the recovered bitstring.
pub fn bernstein_vazirani(secret: &[u8]) -> Vec<u8> {
    let n = secret.len();
    let mut sim = Simulator::new(n + 1);
    // Prepare ancilla as |->
    sim.apply(gates::pauli_x(), n);
    sim.apply(gates::hadamard(), n);
    // Hadamard on input qubits
    for q in 0..n {
        sim.apply(gates::hadamard(), q);
    }
    // Oracle: CNOT from qubit q to ancilla for each bit of secret that is 1
    for (q, &bit) in secret.iter().enumerate() {
        if bit == 1 {
            sim.cnot(q, n);
        }
    }
    // Hadamard on input qubits again
    for q in 0..n {
        sim.apply(gates::hadamard(), q);
    }
    // Measure input qubits — should deterministically return `secret`
    (0..n).map(|q| sim.measure(q)).collect()
}

/// Grover's search on n qubits.
/// `oracle` should apply a phase flip (multiply amplitude by -1) to the target state.
/// Returns probabilities after the optimal number of iterations.
pub fn grover(n: usize, oracle: impl Fn(&mut Simulator)) -> Vec<f64> {
    let mut sim = Simulator::new_uniform(n);
    let iterations = ((std::f64::consts::PI / 4.0) * (1usize << n) as f64).sqrt() as usize;
    let iterations = iterations.max(1);

    for _ in 0..iterations {
        // Oracle: phase flip on target state
        oracle(&mut sim);
        // Diffusion operator: inversion about average
        diffusion(&mut sim);
    }

    sim.probabilities()
}

/// Grover diffusion operator (inversion about average) for all qubits.
fn diffusion(sim: &mut Simulator) {
    let n = sim.n_qubits;
    // H on all qubits
    for q in 0..n {
        sim.apply(gates::hadamard(), q);
    }
    // Phase flip on |0...0⟩: X on all, multi-controlled-Z, X on all
    for q in 0..n {
        sim.apply(gates::pauli_x(), q);
    }
    phase_flip_zero(sim);
    for q in 0..n {
        sim.apply(gates::pauli_x(), q);
    }
    // H on all qubits
    for q in 0..n {
        sim.apply(gates::hadamard(), q);
    }
}

/// Phase-flip the |0...0⟩ state using CZ decomposition.
/// For n=1: Z gate. For n=2: CZ. For n>2: Toffoli ladder then CZ.
fn phase_flip_zero(sim: &mut Simulator) {
    let n = sim.n_qubits;
    match n {
        1 => { sim.apply(gates::pauli_z(), 0); }
        2 => { sim.cz(0, 1); }
        _ => {
            // Toffoli ladder: reduce multi-control to a single CZ on the last two qubits
            // Uses ancilla-free decomposition for up to ~4 qubits
            for c in 0..n - 2 {
                sim.toffoli(c, c + 1, c + 2);
            }
            sim.cz(n - 2, n - 1);
            // Uncompute Toffoli ladder
            for c in (0..n - 2).rev() {
                sim.toffoli(c, c + 1, c + 2);
            }
        }
    }
}
