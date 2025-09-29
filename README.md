# Rust FFI Demo

A demonstration project showing how to write a library in Rust,  
expose its API to C/C++ via FFI, and build everything with a unified CMake process.

---

## Project structure

```
.
├─ Cargo.toml           # Cargo manifest of the Rust library
├─ cbindgen.toml        # Config for generating C headers
├─ CMakeLists.txt       # Unified build (Rust + C++)
├─ src/
│   ├─ core.rs          # Pure Rust implementation (Counter)
│   ├─ ffi.rs           # FFI wrappers (extern "C")
│   └─ lib.rs           # Entry point, re-exports
├─ cpp/
│   └─ main.cpp         # C++ usage example
└─ build/               # CMake build directory
```

---

## Build

Build the project using CMake:

```bash
mkdir -p build
cd build
cmake -DCMAKE_BUILD_TYPE=Debug ..   # or Release
make
```

Results:

- Rust library (`.so`, `.dll`, `.dylib`) is built into `target/{debug,release}/`
- C header `rust_ffi_demo.h` is generated into `build/`
- Example binary `cpp_example` is built and linked against the Rust library

---

## Header generation

The header is generated automatically using [cbindgen](https://github.com/eqrion/cbindgen).
`cbindgen.toml` is configured to export only:

- `CounterHandle` (opaque handle)
- `RustFfiDemoStatus` (error/status enum)

The internal Rust structure `Counter` is not exposed to C API.

---

## Usage from C++

Example (`cpp/main.cpp`):

```cpp
#include "rust_ffi_demo.h"
#include <iostream>

int main() {
    CounterHandle* counter = nullptr;
    rust_ffi_demo_counter_new(10, &counter);

    rust_ffi_demo_counter_increment(counter, 5);

    long long value = 0;
    rust_ffi_demo_counter_value(counter, &value);
    std::cout << "Counter value = " << value << std::endl;

    rust_ffi_demo_counter_free(counter);
}
```

---

## Callbacks

The library supports **per-counter callbacks**: each `Counter` can be associated with a callback function that is automatically fired when the counter value changes.

### Rust usage

```rust
let mut c1 = Counter::new(10);
c1.set_label(Some("alpha".into()));

c1.set_callback(Some(Box::new(|val| {
    println!("[Rust callback] Counter changed to {}", val);
})));

println!("c1 before increment: {:?}", c1);

c1.increment(5);  // triggers callback
println!("c1 after increment: {:?}", c1);

let c2 = c1.clone(); // cloned without callback
println!("c2 clone: {:?}", c2);
```

Output:

```
c1 before increment: Counter { value: 10, label: Some("alpha"), .. }
[Rust callback] Counter changed to 15
c1 after increment: Counter { value: 15, label: Some("alpha"), .. }
c2 clone: Counter { value: 15, label: Some("alpha"), .. }
```

---

### C++ usage

Extend `main.cpp` with:

```cpp
// Our callback function
void on_value_changed(long long value) {
    std::cout << "[C++] Callback fired! Counter changed, new value = " << value << std::endl;
}

...

// Register callback
rust_ffi_demo_counter_set_callback(counter, on_value_changed);

// Trigger events
rust_ffi_demo_counter_increment(counter, 7);   // callback fires
rust_ffi_demo_counter_reset(counter);          // callback fires
```

Expected output:

```
Counter value = 15
Registering callback...
Incrementing by 7 (should trigger callback)...
[C++] Callback fired! Counter changed, new value = 22
Resetting counter (should trigger callback)...
[C++] Callback fired! Counter changed, new value = 0
```

---

## Debugging in VSCode

Recommended extensions:
- **rust-analyzer**
- **CodeLLDB**

Example `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug cpp_example + Rust",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/build/cpp_example",
            "cwd": "${workspaceFolder}/build",
            "env": {
                "LD_LIBRARY_PATH": "${workspaceFolder}/target/debug"
            },
            "sourceLanguages": ["rust", "cpp"]
        }
    ]
}
```

You can set breakpoints both in `cpp/main.cpp` and in Rust sources (`src/ffi.rs`).

---

## License

MIT
