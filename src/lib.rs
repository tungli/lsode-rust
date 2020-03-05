use libc::{c_double, c_int};

#[link(name = "gfortran")]
extern {
    pub fn dlsode_(f: extern fn(&c_int, &mut c_double, *mut c_double, *mut c_double),
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

