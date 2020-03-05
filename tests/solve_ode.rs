extern crate lsode;
use libc::{c_int, c_double};

fn solution(t: f64) -> [f64; 2] {
    [
        2.0*(-t).exp() - (-(1000.0*t)).exp(),
        -((-t).exp()) + (-(1000.0*t)).exp()
    ]
}

fn rhs(y: &[f64], _t: &f64) -> Vec<f64> {
    let mut dy = vec![0.0, 0.0];
    dy[0] = 998.0*y[0] + 1998.0*y[1];
    dy[1] = -999.0*y[0] - 1999.0*y[1];
    dy
}

#[test]
fn main() {
    let y0 = [1.0, 0.0];
    let ts: Vec<f64> = (0..10).map(|i| 0.1*i as f64).collect();
    let atol = 1e-6;
    let rtol = 1e-8;

    let sol = lsode::solve_ode(rhs, &y0, ts.clone(), atol, rtol);

    for (analytical, calculated) in ts.iter().map(|x| solution(*x)).zip(sol) {
        assert!((analytical[0] - calculated[0]).abs() < 1e-3, "|{} - {}| not matching the tolerance", analytical[0], calculated[0]);
        assert!((analytical[1] - calculated[1]).abs() < 1e-3, "|{} - {}| not matching the tolerance", analytical[1], calculated[1]);
    }
}


