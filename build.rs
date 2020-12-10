extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-w")
        // use for version of gfortran 10+
        //.flag("-fallow-argument-mismatch")
        .file("src/odepack.f")
        .compile("libodepack.a");

    println!("cargo:rustc-link-lib=static=odepack");
    println!("cargo:rustc-link-lib=dylib=gfortran");

}
