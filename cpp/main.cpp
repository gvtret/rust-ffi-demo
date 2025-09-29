/**
 * @file main.cpp
 * @brief Example of using a Rust library from C++.
 */

#include <iostream>
#include <vector>
#include <memory>
#include <cstring>

// Header — auto-generated from Rust through cbindgen
#include "rust_ffi_demo.h"

static void die_on_error(RustFfiDemoStatus st, const char* what) {
	if (st != RustffiOk) {
		const char* msg = rust_ffi_demo_last_error_message();
		std::cerr << what << " failed, code=" << (int)st
		          << (msg ? std::string(", err=") + msg : std::string()) << std::endl;
		std::exit(1);
	}
}

int main() {
	std::cout << "Using Rust lib: " << (const char*)rust_ffi_demo_version() << "\n";

	// Creating a counter
	CounterHandle* h = nullptr;
	die_on_error(rust_ffi_demo_counter_new(/*initial*/ 42, &h), "counter_new");

	// Smart "deleter" for automatic deallocation
	auto deleter = [](CounterHandle* p){ rust_ffi_demo_counter_free(p); };
	std::unique_ptr<CounterHandle, decltype(deleter)> holder(h, deleter);

	// Incrementations
	die_on_error(rust_ffi_demo_counter_increment(holder.get(), 5), "increment");
	die_on_error(rust_ffi_demo_counter_increment(holder.get(), -2), "increment");

	// Reading value
	int64_t val = -1;
	die_on_error(rust_ffi_demo_counter_value(holder.get(), &val), "value");
	std::cout << "Value after ops = " << val << "\n";

	// Label (UTF-8 string)
	die_on_error(rust_ffi_demo_counter_set_label(holder.get(), "demo-label"), "set_label");

	// Getting size
	size_t needed = 0;
	die_on_error(rust_ffi_demo_counter_get_label(holder.get(), nullptr, 0, &needed), "get_label(size)");
	std::vector<char> buf(needed);
	die_on_error(rust_ffi_demo_counter_get_label(holder.get(), buf.data(), buf.size(), &needed), "get_label");
	std::cout << "Label = '" << buf.data() << "'\n";

	// Resetting
	die_on_error(rust_ffi_demo_counter_reset(holder.get()), "reset");
	die_on_error(rust_ffi_demo_counter_value(holder.get(), &val), "value");
	std::cout << "After reset = " << val << "\n";

	std::cout << "OK\n";
	return 0;
}
