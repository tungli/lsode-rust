use libc::{c_double, c_int};
use std::slice;
use libffi::high::ClosureMut4;

#[link(name = "gfortran")]
extern {
    pub fn dlsode_(f: extern fn(*const c_int, *const c_double, *mut c_double, *mut c_double),
               neq: &c_int,
               y: *mut c_double,
               t: &mut c_double,
               tout: &c_double,
               itol: &c_int,
               rtol: &c_double,
               atol: &c_double,
               itask: &c_int,
               istate: &c_int,
               iopt: &c_int,
               rwork: *mut c_double,// 22 + 9*N + N*N
               lrw: &c_int,
               iwork: *mut c_int,//20 + N
               liw: &c_int,
               jac: extern fn(&mut c_int,&mut c_double,&mut c_double,&mut c_int,&mut c_int,&mut c_double,&mut c_int),
               mf: &c_int
               );
}

pub extern fn fake_jacobian(
    _x1: &mut c_int,
    _x2: &mut c_double,
    _x3: &mut c_double,
    _x4: &mut c_int,
    _x5: &mut c_int,
    _x6: &mut c_double,
    _x7: &mut c_int,
    ) {
}

pub fn solve_ode(
    rhs: fn(&[f64], &f64) -> Vec<f64>, 
    y0: &[f64],
    t_dense: Vec<f64>,
    atol: f64,
    rtol :f64,
    ) -> Vec<Vec<f64>> {

    let mut f = | n: *const c_int, t: *const c_double, y_ptr: *mut c_double, dy_ptr: *mut c_double | {
        let dy = unsafe { slice::from_raw_parts_mut(dy_ptr, n as usize) }; 
        let y = unsafe { slice::from_raw_parts(y_ptr, n as usize) }; 
        let dy_new = unsafe {rhs(y, &*t) };
        for (i, deriv) in dy_new.iter().enumerate() {
            dy[i] = *deriv
        }
    };
    let closure = ClosureMut4::new(&mut f);
    let call = closure.code_ptr();

    let mut y: Vec<f64> = y0.to_vec();
    let n = y0.len();
    let mut t = t_dense[0];

    let itol = 1;
    let itask = 1;
    let iopt = 0;
    let istate = 1;
    let mf = 22;

    let lrw = 22 + 9*n + n*n;
    let liw = 20 + n;
    let mut rwork: Vec<f64> = (0..lrw).map(|_i| 0.0 as c_double).collect();
    let mut iwork: Vec<i32> = (0..liw).map(|_i| 0 as c_int).collect();

    let mut result = Vec::new();

    for tout in t_dense {
        unsafe { dlsode_(
                *call,
                &(n as i32),
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
                &(lrw as i32),
                iwork.as_mut_ptr(),
                &(liw as i32),
                fake_jacobian,
                &mf
                ); }
        result.push(y.clone());
    }
    result
}
