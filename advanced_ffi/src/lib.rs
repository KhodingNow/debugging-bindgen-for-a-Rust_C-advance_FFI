include!(concat!(env!("OUT_DIR"), "/nginx.rs"));

#[unsafe(no_mangle)]

pub unsafe extern "C" fn ngx_http_calculator_handler(
    r: *mut ngx_http_request_t) -> ngx_int_t {
    eprintln!("Hello from Rust");
    0
    
 }

