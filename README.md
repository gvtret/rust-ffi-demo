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
