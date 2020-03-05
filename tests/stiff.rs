extern crate lsode;
use libc::{c_int, c_double};
use std::slice;

// From Numerical Recipes - The Art of Scientific Computing by W.H. Press et al. in chapter about
// stiff ODEs.
extern fn rhs(
    n: &c_int,
    _t: &mut c_double,
    y_ptr: *mut c_double,
    dy_ptr: *mut c_double
    ) {
    let dy = unsafe { slice::from_raw_parts_mut(dy_ptr, *n as usize) }; 
    let y = unsafe { slice::from_raw_parts(y_ptr, *n as usize) }; 
    dy[0] = 998.0*y[0] + 1998.0*y[1];
    dy[1] = -999.0*y[0] - 1999.0*y[1];
}

fn solution(t: f64) -> [f64; 2] {
    [
        2.0*(-t).exp() - (-(1000.0*t)).exp(),
        -((-t).exp()) + (-(1000.0*t)).exp()
    ]
}

#[test]
fn main() {
    let mut y = [1.0, 0.0];
    const NEQ: i32 = 2;
    let mut t = 0.0;
    let tout = 1.0;
    let itol = 1;
    let itask = 1;
    let iopt = 0;
    let istate = 1;
    let mf = 22;
    const LRW: i32 = (22 + 9*NEQ + NEQ*NEQ);
    const LIW: i32 = 20 + NEQ;
    let atol = 1e-6;
    let rtol = 1e-8;
    let mut rwork: [c_double; LRW as usize] = [0.0; LRW as usize];
    let mut iwork: [c_int; LIW as usize] = [0; LIW as usize];

    unsafe { lsode::dlsode_(
            rhs,
            &NEQ,
            y.as_mut_ptr(),
            &mut t,
            &tout,
            &itol,
            &rtol,
            &atol,
            &itask,
            &istate,
            &iopt,
            rwork.as_mut_ptr(),
            &LRW,
            iwork.as_mut_ptr(),
            &LIW,
            lsode::fake_jacobian,
            &mf
           ) };

    let sol = solution(tout);
    assert!((sol[0] - y[0]).abs() < 1e-3, "|{} - {}| not matching the tolerance", sol[0], y[0]);
    assert!((sol[1] - y[1]).abs() < 1e-3, "|{} - {}| not matching the tolerance", sol[1], y[1]);
}


