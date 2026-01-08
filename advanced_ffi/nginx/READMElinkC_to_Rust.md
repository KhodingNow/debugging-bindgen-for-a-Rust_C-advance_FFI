
LINKING C to Rust.

Previously we had written a forward declaration for an HTTP handler :

        ngx_int_t ngx_http_calculator_handler(ngx_http_req          uest_t *r);

We understood that we will later provide this function in our Rust library.Translating that C function declaration to a Rust function declaration is straightforward, e.g;


        #[no_mangle]
        pub unsafe extern "C" fn ngx_http_calculator_handl          er(
          r: *mut ngx_http_request_t) -> ngx_int_t {
            0
            
            }
This function needs to exist to be callable from NGINX, but a few things need to happen first.You may have noticed some types in that function signature start with the prefix 'ngx_'.
These types are exposed by the NGINX module API in its header files in the C code, and the types would be available to the developer.

Since we are writing our handler function in C, we need to do some work to get these types into Rust.

We are going to need to generate Rust bindings for the C types in NGINX. A binding is essentially metadata about an API that exeists for a library implemented in a different programming language. It's the metadata about all the functions, types, and global variables that exeist in that library - without the implementation of any of those things.

We have already created C bindings for the Rust calculate library with a C-compatible solve function as a part of that library.
Bindings don't always exist as a part of a library itself, they are often provided by separate ibraries. For example, the 'openssl' library is written in C; to directly interact with the C functions from Rust, you can use the openssl-sys Rust crate.
This crate provides Rust bindidngs for the openssl C library.
 
BUILD scripts.

A build script is a small Rust program that Cargo compiles and runs just before our larger library or executable is compiled.It can do anything that a normal Rust program would do.It is useful to us because it can generate Rust code dynamically at build time, which is fed back into the compiler.

A BIG NOTE TO MYSELF:

- we are at the boundary where
 - C and ABI (NGINX module)

- meets Rust FFI

- compiled as a shared object (.so)

This is the foundation needed to later:

- expose C functions
link Rust with a #[no_mangle] extern "C"
- safely integrate Rust logic into NGINX. 


A note on the PCRE library error (especially when comipling from source)

  "the HTTP rewrite module requires the PCRE lib" 

NGINX enables http_rewrite_module by default, and that module requires PCRE (PERL Compatible Regular Expressions).

(I must install this onto my Kali VM so that NGINX finds it) to build.

- A note on the SEGfault from the Cargo build command:

the script has environment variables - in the nature of Rust / C build script..cargo executes the script PRE compile time. I cannot call it DIRECTLY - I need to export the file.


    $  export GREET_LANG=en
       cargo build

After managing to run the build script, this is the output
    $ ls src
    main.rs greet.rs

    $ cat src/greet.rs
    en


Bindgen

bindgen is a Rust library that parses C/C++ and outputs Rust bindings automatically. In its simplest form, bindgen generates Rust-compatible definitions for C/C++ types and functions loaded from a single header file.

Add bindgen to Cargo.toml. 

We do not include bindgen under the dependencies section but rather the new-to-us 'build-dependencies' section.

Since the bindgen will only be used from the build script to generate Rust code, it does not need to be included in our finished binary as a normal dependency; we only need it to be included in the dependencies of our build script.

