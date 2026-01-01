bindgen is a Rust library that parses C/C++ code and outputs Rust bindings automatically.It generates Rust-compatibledefinitions for C/C++ types and functionsloaded from a single header file. You add bindgen to a Cargo.toml file.


    [paeckage]
    name = "ngx_http_calculator_rs"
    varsion = "0.1.0"
    authors = ["xyz<xyz@yy.com>"]
    edition = "2024"

    [dependencies]

    [build-dependencies]
    bindgen = "0.56.0"


bindgen sits under the build-dependencies not in a finished binary as a normal dependency, we only need it to included in the dependencies of our build-script.

bindgen works by parsing a C/C++ header file (and allowing all 'include' directives) for type, variable, & function declarations and outputting Rust code that is compatible with those directives.

bindgen also needs the header file and that file needs to '#include' all the headers that Rust module might need access to.

We will need  a header file - 'wrapper.h' 

    #include <ngx_config.h>
    #include <ngx_core.h>
    #include <ngx_http.h>


This is a normal C header file, but instead of being used to compile C code, it will be used to generate Rust code. So, now we will need a 'build.rs' file - its contents reveals how it uses bindgen to create our bindings.



    fn main() {
  let nginx_dir = "nginx-1.19.3";

  let bindings = bindgen::builder()
    .header("wrapper.h")
    .clang_args(vec![
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

  bindings
    .write_to_file("nginx.rs")
    .unwrap();  

    }

    
MY PERSONAL experience debugging bindgen:

this Rust / C/C++ interface file run through a script to allow Rust and C to communicate sits at the heart of SYSTEM ENGINEERING.

this is my first error I came across when trying to compile the file via the above script from the variety of the 'CLANG_args':

The error:

    Compiling ngx_http_calculator_rs v0.1.0 (/home/mlawumba/Refactor_to_Rust/advanced_ffi) error: failed to run custom build command for ngx_http_calculator_rs v0.1.0 (/home/mlawumba/Refactor_to_Rust/advanced_ffi) Caused by: process didn't exit successfully: /home/mlawumba/Refactor_to_Rust/advanced_ffi/target/debug/build/ngx_http_calculator_rs-60534b844ea0f73a/build-script-build (exit status: 101) 

    --- stderr 

    thread 'main' (32937) panicked at /home/mlawumba/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bindgen-0.56.0/src/ir/context.rs:846:9: "in6_addr_union_(unnamed_at_/usr/include/netinet/in_h_225_5)" is not a valid Ident note: run with RUST_BACKTRACE=1 environment variable to display a backtrace
    


this error, led me to open and read a MAKEFILE of 1200+ code lines, and the offending line was a PATH line at 1207 code line deep. (Thanks to an LLM to discover this)

this to me reveals WHY Rust is a safe system language - if you read the error, you discover one of Rust's key features elated to error handling. this is the 'thread "main" (32937) panicked part' - Rust detects a tread related fatal issue and crashes instead of processing it to protect the memory in the heap and system at large.

The error did not come from the Rust code, not Cargo - it comes from bindgen itself as a result of an older version we put into our Cargo.toml file.

This is the line from the error that reveals the problem:

    "in6_addr_union_(unnamed_at_/usr/include/netinet/in_h_225_5)" is not a valid Ident


 The root cause is - 
    bindgen v0.56.0
    in our Cargo.toml file.

  Cargo is trying to generate through the ( header build script) Rust identifiers from ANONYMOUS C unions inside:

    usr/include/netinet/in.h

Specifically:

    union {
    uint8_t  __u6_addr8[16];
    uint16_t __u6_addr16[8];
    uint32_t __u6_addr32[4];
    } __in6_u;

Bindgen is trying to invent a Rust name like:

    in6_addr_union_(unnamed_at_/usr/include/netinet/in_h_225_5)

This is NOT a valid Rust identifier, Rust panicks (crashes). A key Rust safety feature to protect systems.

This is a class of issues in older bindgen versions when dealing with :

    - netinet/in.h
    - sys/socket.h
    - glibc header
    - anonymous unions / structs

A quick fix is a simple upgrade of bindgen.

    * I did so on Cargo.toml to ~ v0.69 , a current and stable version.


Another bindings protection we can add on the 'build.rs' file is setting it up with a couple of ALLOWLISTS for functions, variables, type, clang_arg and headers:

Like this;

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("ngx_.*")
        .allowlist_type("ngx_.*")
        .allowlist_var("NGX_.*")
        .blocklist_type("in6_addr")
        .blocklist_type("__.*")
        .clang_arg("-D__USE_MISC")
        .generate()
        .expect("Unable to generate bindings");


These methods will prevent bindgen from touching problematic LIBC internals.

Why is this happening with NGINX builds?

NGINX includes 

    #include <netinet/in.h>

This pulls in:
    - system networking structs
    - anonymous unions
    - GCC extensions

Bindgen was NEVER meant to fully parse libc unless you explicitly allow it.

What is teh BEST practice - create that 'wrapper.h' file as above, AND add an ALLOWLIST of ngx methods in the Rust' build.rs ' file. This keeps things safe and stable.


THE NEXT Part in the debugging of bindgen is laid out in this error - the Rust compiler names it an (os error) in the file 'lib.rs':


    Compiling rustc-hash v1.1.0 Compiling ngx_http_calculator_rs v0.1.0 (/home/mlawumba/Refactor_to_Rust/advanced_ffi) error: couldn't read /home/mlawumba/Refactor_to_Rust/advanced_ffi/target/debug/build/ngx_http_calculator_rs-8cff3ffef30793a0/out/nginx.rs: No such file or directory (os error 2) 

    --> src/lib.rs:1:1 | 1 | include!(concat!(env!("OUT_DIR"), "/nginx.rs")); | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

This error indicates the source of the problem (not Rust, not NGINX) but where the GENERATED file is written.


What this error means: 

    couldn't read .../out/nginx.rs: No such file or direc   tory

Rust is doing exactly what I rold it to do:
    include!(concat!(env!("OUT_DIR"), "/nginx.rs"));
    
Rust expects this file to exist IN the correct location, 
    target/debug/build/<crate-hash>/out/nginx.rs


But my file tree shows: 

    advanced_ffi/
├── nginx.rs   ❌  (wrong location)

ROOT CAUSE:

the 'build.rs' file is writing 'nginx.rs' to the project root, NOT to '$OUT_DIR'
Cargo build scripts must write generated files into 'OUT_DIR' or 'include!()' will fail.

In build.rs I had a line - 

    bindings.write_to_file("nginx.rs").unwrap();

So, I had to update that file, that line and aadd some protections that are available WITHIN Rust to protect when automatically generated file via scripts.

The protections are to to import std lib files into 'build.rs':

    - use std::env;
    - use std::path::PathBuf;

I also then had to add these lines of code.

    let out_path = PathBuf::from(env::var("OUT_DIR").unwr   ap());
   
   bindings
   
        .write_to_file(out_path.join("nginx.rs"))
        .expect("Couldn't write bindings!");

This matters because Cargo build scripts run in ISOLATED BUILD DIRECTORIES.
Writing to the project root breaks reproducibility and caching.

 - That is why GENERATED code must always go into '$OUT_DIR'


It is important to point out, that this build is 

    - Rust <-> C ABI boundary
    - NGINX module integration
    - bindgen safety constraints

Why 

    PathBuff::?

This is a Rust-specific type from 'std::path' that represents an owned, mutable path (like String is to &str). It's used to:

    - Store and manipulate file system paths
    - Append path components
    - Convert between different path representations
    - Handle OS-specific path differences


Common Pattens with bindgen

1. Converting C paths to PathBuff

    // If C gives a path string

    fn c_path_to_pathbuf(c_path: *const c_char) -> Option   <PathBuf> {
        unsafe {
            CStr::from_ptr(c_path)
            .to_str()
            .ok()
            .map(PathBuf::from)
        }
    }


   2. Passing Rust paths to C

    use std::ffi::CString;

    fn pass_path_to_c(path: &PathBuf) -> *mut c_char {

    // Convert Pathbuf to string, then to C string 

        if let Some(s) = path.to_str() {
            CString::new(s)
            .expect("Failed to create CString")
            
        } else {

            std::ptr::null_mut()
        }
    }

3. bindgen and paths

bindgen won't generate PathBuf types. It generates:

    - Raw pointers for C strings
    - Structs with '*const c_char' members   
    - You convert these to PathBuf manually


It is exactly the kind of setup used in systems work,  system production.
  
So, w spoke about PathBuf in the context of bindgen, earlier in this personal notebook on debugging bidgen - I had to edit my 'build.rs' file - because it works  on the Rust side to generate paths (networkin, file system path, http request, etc becuase of a script to interface Rust with C.

In that file, this line changed  

    .write_to_file(format!("{}nginx.rs", out_dir))

because it produces an invalid path. It changed to :

We need to think about what 'OUT_DIR' is - an envirnoment variable thak keeps automatically generated code, the problem I am facing, the behaviour of out_dir is import. This is how this variable looks like at runtime:

    /home/mlawumba/Refactor_to_Rust/advanced_ffi/target/debug/build/ngx_http_calculator_rs-8cff3ffef30793a0/out

When i write 

    format!("{}nginx.rs", out_dir)

Rust creates :

    /home/.../outnginx.rs

    please notice the missing '/'

the file is written as .../outnginx.rs  
    this is an invalid location

    Hence the - no such file or directory error.

This is the reason WHY we have to turn to std library and use

       use std::env
       use std::path::PathBuf

..and include / or refctor to this line 

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    
 in the build.rs file.

Why is this important?

Rust paths are not strings conceptually, they are platform-aware structures

Using:

    PathBuf::join("nginx.rs")

    - handles / vs \
    - avoids subtle path bugs
    - is required for cross-platform correctness

NB - I am not done - this debugging continues, as my project carries on - It was important for me to note down the eye opening moments I have encountered so far as far as the system internals are concerned.


