extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-w")
        .file("src/odepack.f")
        .compile("libodepack.a");

    println!("cargo:rustc-link-lib=static=odepack");
    println!("cargo:rustc-link-lib=dylib=gfortran");

}
