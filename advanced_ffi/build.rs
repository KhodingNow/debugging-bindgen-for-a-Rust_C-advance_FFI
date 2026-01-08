use std::env;
use std::path::PathBuf;


fn main() {
    let nginx_dir = "nginx";

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .allowlist_type("ngx_.*")
        .allowlist_function("ngx_.*")
        .allowlist_var("ngx_.*")        
        .clang_args(vec! [
            format!("-I{}/src/core", nginx_dir),
            format!("-I{}/src/event", nginx_dir),
            format!("-I{}/src/event/modules", nginx_dir),
            format!("-I{}/src/os/unix", nginx_dir),
            format!("-I{}/objs", nginx_dir),
            format!("-I{}/src/http", nginx_dir),
            format!("-I{}/src/http/v2", nginx_dir),
            format!("-I{}/src/http/modules", nginx_dir),
        ])
        .generate()
        .unwrap();

let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_dir.join("nginx.rs"))
        .expect("unable to write bindings");
}
