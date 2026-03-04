use num_complex::Complex64;
use rand::Rng;
use crate::gates::Gate2;

pub struct Simulator {
    pub n_qubits: usize,
    state: Vec<Complex64>,
}

impl Simulator {
    /// Initialize to ground state |0...0⟩
    pub fn new(n_qubits: usize) -> Self {
        let len = 1 << n_qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); len];
        state[0] = Complex64::new(1.0, 0.0);
        Self { n_qubits, state }
    }

    /// Initialize to uniform superposition (|+...+⟩)
    pub fn new_uniform(n_qubits: usize) -> Self {
        let len = 1usize << n_qubits;
        let amp = Complex64::new((len as f64).powf(-0.5), 0.0);
        Self {
            n_qubits,
            state: vec![amp; len],
        }
    }

    /// Initialize from an explicit normalized state vector.
    /// Panics if length is not a power of 2.
    pub fn from_state(state: Vec<Complex64>) -> Self {
        let len = state.len();
        assert!(len.is_power_of_two(), "State vector length must be a power of 2");
        let n_qubits = len.trailing_zeros() as usize;
        let norm: f64 = state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-9, "State vector must be normalized (norm = {norm:.6})");
        Self { n_qubits, state }
    }

    pub fn state(&self) -> &[Complex64] {
        &self.state
    }

    /// Apply a single-qubit gate to `qubit`.
    pub fn apply(&mut self, gate: Gate2, qubit: usize) {
        assert!(qubit < self.n_qubits, "Qubit index {qubit} out of range (n_qubits={})", self.n_qubits);
        let n = self.state.len();
        for i in 0..n {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                let a = self.state[i];
                let b = self.state[j];
                self.state[i] = gate[0][0] * a + gate[0][1] * b;
                self.state[j] = gate[1][0] * a + gate[1][1] * b;
            }
        }
    }

    /// Apply a CNOT gate: flips `target` when `control` is |1⟩.
    pub fn cnot(&mut self, control: usize, target: usize) {
        assert_ne!(control, target, "Control and target must be different qubits");
        assert!(control < self.n_qubits && target < self.n_qubits, "Qubit index out of range");
        let n = self.state.len();
        for i in 0..n {
            if (i >> control) & 1 == 1 && (i >> target) & 1 == 0 {
                let j = i | (1 << target);
                self.state.swap(i, j);
            }
        }
    }

    /// Apply a CZ gate: applies a phase of -1 when both `control` and `target` are |1⟩.
    pub fn cz(&mut self, control: usize, target: usize) {
        assert_ne!(control, target, "Control and target must be different qubits");
        assert!(control < self.n_qubits && target < self.n_qubits, "Qubit index out of range");
        for i in 0..self.state.len() {
            if (i >> control) & 1 == 1 && (i >> target) & 1 == 1 {
                self.state[i] = -self.state[i];
            }
        }
    }

    /// Apply a Toffoli (CCX) gate: flips `target` when both `c0` and `c1` are |1⟩.
    pub fn toffoli(&mut self, c0: usize, c1: usize, target: usize) {
        assert!(c0 != c1 && c0 != target && c1 != target, "All qubits must be distinct");
        assert!(c0 < self.n_qubits && c1 < self.n_qubits && target < self.n_qubits, "Qubit index out of range");
        let n = self.state.len();
        for i in 0..n {
            if (i >> c0) & 1 == 1 && (i >> c1) & 1 == 1 && (i >> target) & 1 == 0 {
                let j = i | (1 << target);
                self.state.swap(i, j);
            }
        }
    }

    /// Probability of measuring qubit in state |1⟩ without collapsing.
    pub fn prob_one(&self, qubit: usize) -> f64 {
        (0..self.state.len())
            .filter(|&i| (i >> qubit) & 1 == 1)
            .map(|i| self.state[i].norm_sqr())
            .sum()
    }

    /// Measure `qubit`: collapse state, return 0 or 1.
    pub fn measure(&mut self, qubit: usize) -> u8 {
        let p1 = self.prob_one(qubit);
        let outcome = if rand::thread_rng().gen::<f64>() < p1 { 1u8 } else { 0u8 };

        // Zero out amplitudes inconsistent with the outcome
        for i in 0..self.state.len() {
            if ((i >> qubit) & 1) as u8 != outcome {
                self.state[i] = Complex64::new(0.0, 0.0);
            }
        }

        // Renormalize
        let norm: f64 = self.state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        for amp in &mut self.state {
            *amp /= norm;
        }

        outcome
    }

    /// Measure all qubits at once. Returns a bitstring as a Vec<u8> (qubit 0 first).
    pub fn measure_all(&mut self) -> Vec<u8> {
        (0..self.n_qubits).map(|q| self.measure(q)).collect()
    }

    /// Density matrix ρ = |ψ⟩⟨ψ|, returned as a flat row-major Vec.
    pub fn density_matrix(&self) -> Vec<Vec<Complex64>> {
        let n = self.state.len();
        (0..n)
            .map(|i| (0..n).map(|j| self.state[i] * self.state[j].conj()).collect())
            .collect()
    }

    pub fn probabilities(&self) -> Vec<f64> {
        self.state.iter().map(|c| c.norm_sqr()).collect()
    }

    pub fn print_state(&self) {
        println!("State vector ({} qubit{}):", self.n_qubits, if self.n_qubits == 1 { "" } else { "s" });
        for (i, amp) in self.state.iter().enumerate() {
            let prob = amp.norm_sqr();
            if prob > 1e-10 {
                println!("  |{:0>width$b}⟩  {:+.4} {:+.4}i  (p = {:.4})",
                    i, amp.re, amp.im, prob, width = self.n_qubits);
            }
        }
    }

    pub fn print_probabilities(&self) {
        println!("Probabilities ({} qubit{}):", self.n_qubits, if self.n_qubits == 1 { "" } else { "s" });
        for (i, prob) in self.probabilities().iter().enumerate() {
            if *prob > 1e-10 {
                println!("  |{:0>width$b}⟩  {:.4}", i, prob, width = self.n_qubits);
            }
        }
    }
}
