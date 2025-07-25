#



## Sample Design 

This sample is designed with the following structure:
1. `pure_rust_types.rs`
    1. This implements the objects with normal Rust structs and traits.
1. `bindings.rs`
    1. This contains the generated WinRT structs and traits, this should not be modified by the developer.
1. `winrt_rust_types.rs`
    1. This contains the developer's implementation of the WinRT structs and traits.
    1. This is split from the pure implementation because these objects may be hard to implement cleanly
    1. This is much harder to debug because of the generated code.
1. `lib.rs`
    1. Sets up the globals to make this DLL publicly export the WinRT types