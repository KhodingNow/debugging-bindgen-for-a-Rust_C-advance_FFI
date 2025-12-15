Intro to C FFI and unsafe code
If we want to embed Rust code within an existing application, we need some very well defined semantics for how the two lanvguages communicate, how vakues are passed bween them, and how memory may or may not be shared between them.Ideally, this interface between the two languages and platforms so we can avoid re-writing code to perform a specific integration
