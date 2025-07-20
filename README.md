## Learning Rust FFI

Plan is to work through:

    * Vanilla Rust <-> C interop with `std::ffi` and the core language [✅]
        * Calling a static C library from Rust [✅]
        * Calling a dynamic C library from Rust [✅]
        * Calling a static Rust library from C [✅]
        * Calling a dynamic Rust library from C [✅]

    * Rust <-> C interop using `bindgen and `cbindgen` []
        * Calling a static C library from Rust []
        * Calling a dynamic C library from Rust []
        * Calling a static Rust library from C []
        * Calling a dynamic Rust library from C []

    * Rust <-> Java interop with `jni` []
        * Basic Java -> Rust interop [✅]
    * Rust <-> Android interop with `ndk-context` []
    * Rust <-> iOS interop with `objc` []
    * Rust <-> C++ interop with `cxx` []
    * Deepdive into C stdlib interop with `libc` []