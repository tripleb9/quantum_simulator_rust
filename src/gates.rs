use num_complex::Complex64;

pub type Gate2 = [[Complex64; 2]; 2];

fn c(re: f64, im: f64) -> Complex64 {
    Complex64::new(re, im)
}

pub fn hadamard() -> Gate2 {
    let s = std::f64::consts::FRAC_1_SQRT_2;
    [[c(s, 0.0), c(s, 0.0)],
     [c(s, 0.0), c(-s, 0.0)]]
}

pub fn pauli_x() -> Gate2 {
    [[c(0.0, 0.0), c(1.0, 0.0)],
     [c(1.0, 0.0), c(0.0, 0.0)]]
}

pub fn pauli_y() -> Gate2 {
    [[c(0.0,  0.0), c(0.0, -1.0)],
     [c(0.0,  1.0), c(0.0,  0.0)]]
}

pub fn pauli_z() -> Gate2 {
    [[c(1.0, 0.0), c( 0.0, 0.0)],
     [c(0.0, 0.0), c(-1.0, 0.0)]]
}

pub fn phase_s() -> Gate2 {
    [[c(1.0, 0.0), c(0.0, 0.0)],
     [c(0.0, 0.0), c(0.0, 1.0)]]
}

pub fn t_gate() -> Gate2 {
    let angle = std::f64::consts::FRAC_PI_4;
    [[c(1.0, 0.0), c(0.0,         0.0        )],
     [c(0.0, 0.0), c(angle.cos(), angle.sin())]]
}

/// Rotation around X axis by theta radians
pub fn rx(theta: f64) -> Gate2 {
    let c_t = (theta / 2.0).cos();
    let s_t = (theta / 2.0).sin();
    [[c(c_t, 0.0 ), c(0.0, -s_t)],
     [c(0.0, -s_t), c(c_t,  0.0)]]
}

/// Rotation around Y axis by theta radians
pub fn ry(theta: f64) -> Gate2 {
    let c_t = (theta / 2.0).cos();
    let s_t = (theta / 2.0).sin();
    [[c( c_t, 0.0), c(-s_t, 0.0)],
     [c( s_t, 0.0), c( c_t, 0.0)]]
}

/// Rotation around Z axis by theta radians
pub fn rz(theta: f64) -> Gate2 {
    let half = theta / 2.0;
    [[c(half.cos(), -half.sin()), c(0.0, 0.0         )],
     [c(0.0,         0.0      ), c(half.cos(), half.sin())]]
}
