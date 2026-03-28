use plotly::{Plot, Scatter3D};
use plotly::common::{Line, Marker, MarkerSymbol, Mode};
use plotly::layout::{Axis, Layout, LayoutScene};
use std::f64::consts::PI;

/// A labelled Bloch vector to draw on the sphere.
pub struct BlochState {
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BlochState {
    pub fn new(label: impl Into<String>, x: f64, y: f64, z: f64) -> Self {
        Self { label: label.into(), x, y, z }
    }
}

/// Build and open an interactive Bloch sphere with one or more state vectors.
pub fn plot_bloch_sphere(states: &[BlochState], title: &str) {
    let mut plot = Plot::new();

    // ── Wireframe sphere ────────────────────────────────────────────────────
    let steps = 60usize;
    let wire_color = "rgba(180,180,220,0.35)";

    // Latitude circles
    for lat in (-75..=75).step_by(15) {
        let theta = (90.0 - lat as f64).to_radians();
        let (xs, ys, zs) = (0..=steps).fold(
            (vec![], vec![], vec![]),
            |(mut ax, mut ay, mut az), k| {
                let phi = 2.0 * PI * k as f64 / steps as f64;
                ax.push(theta.sin() * phi.cos());
                ay.push(theta.sin() * phi.sin());
                az.push(theta.cos());
                (ax, ay, az)
            },
        );
        plot.add_trace(
            Scatter3D::new(xs, ys, zs)
                .mode(Mode::Lines)
                .line(Line::new().color(wire_color).width(1.0))
                .show_legend(false),
        );
    }

    // Longitude lines
    for lon in (0..180).step_by(30) {
        let phi = lon as f64 * PI / 180.0;
        let (xs, ys, zs) = (0..=steps).fold(
            (vec![], vec![], vec![]),
            |(mut ax, mut ay, mut az), k| {
                let theta = PI * k as f64 / steps as f64;
                ax.push(theta.sin() * phi.cos());
                ay.push(theta.sin() * phi.sin());
                az.push(theta.cos());
                (ax, ay, az)
            },
        );
        plot.add_trace(
            Scatter3D::new(xs, ys, zs)
                .mode(Mode::Lines)
                .line(Line::new().color(wire_color).width(1.0))
                .show_legend(false),
        );
    }

    // ── Axes ────────────────────────────────────────────────────────────────
    let axis_color = "rgba(100,100,100,0.6)";
    for (xs, ys, zs) in [
        (vec![-1.2f64, 1.2], vec![0.0f64, 0.0], vec![0.0f64, 0.0]),
        (vec![0.0, 0.0], vec![-1.2, 1.2], vec![0.0, 0.0]),
        (vec![0.0, 0.0], vec![0.0, 0.0], vec![-1.3, 1.3]),
    ] {
        plot.add_trace(
            Scatter3D::new(xs, ys, zs)
                .mode(Mode::Lines)
                .line(Line::new().color(axis_color).width(2.0))
                .show_legend(false),
        );
    }

    // ── Pole / axis labels ──────────────────────────────────────────────────
    for (lbl, lx, ly, lz) in [
        ("|0⟩",   0.0f64,  0.0f64,  1.4f64),
        ("|1⟩",   0.0,     0.0,    -1.4),
        ("|+⟩",   1.4,     0.0,     0.0),
        ("|−⟩",  -1.4,     0.0,     0.0),
        ("|i⟩",   0.0,     1.4,     0.0),
        ("|−i⟩",  0.0,    -1.4,     0.0),
    ] {
        plot.add_trace(
            Scatter3D::new(vec![lx], vec![ly], vec![lz])
                .mode(Mode::Text)
                .text_array(vec![lbl])
                .show_legend(false),
        );
    }

    // ── State vectors ───────────────────────────────────────────────────────
    let colors = ["#e63946", "#2a9d8f", "#e9c46a", "#f4a261", "#a8dadc"];
    for (i, state) in states.iter().enumerate() {
        let color = colors[i % colors.len()];

        // Line from origin to tip
        plot.add_trace(
            Scatter3D::new(
                vec![0.0, state.x],
                vec![0.0, state.y],
                vec![0.0, state.z],
            )
            .name(&state.label)
            .mode(Mode::Lines)
            .line(Line::new().color(color).width(6.0)),
        );

        // Dot at tip
        plot.add_trace(
            Scatter3D::new(vec![state.x], vec![state.y], vec![state.z])
                .mode(Mode::Markers)
                .marker(Marker::new().size(8).color(color).symbol(MarkerSymbol::Circle))
                .show_legend(false),
        );
    }

    // ── Layout ──────────────────────────────────────────────────────────────
    let silent_axis = Axis::new()
        .show_grid(false)
        .show_tick_labels(false)
        .show_line(false)
        .zero_line(false)
        .range(vec![-1.5, 1.5]);

    let layout = Layout::new()
        .title(title)
        .scene(
            LayoutScene::new()
                .x_axis(silent_axis.clone())
                .y_axis(silent_axis.clone())
                .z_axis(silent_axis),
        );
    plot.set_layout(layout);

    plot.show();
}
