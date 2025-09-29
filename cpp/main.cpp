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

/**
 * @brief Handles errors by printing an error message and exiting the program.
 * 
 * @param st The status code returned from a Rust function call.
 * @param what A string describing the operation that failed.
 */
static void die_on_error(RustFfiDemoStatus st, const char* what) {
	if (st != RustffiOk) {
		const char* msg = rust_ffi_demo_last_error_message();
		std::cerr << what << " failed, code=" << (int)st
		          << (msg ? std::string(", err=") + msg : std::string()) << std::endl;
		std::exit(1);
	}
}

/**
 * @brief Callback function to handle updates from the Rust library.
 * 
 * This function is called by the Rust library whenever a counter value changes.
 * It prints the new value to the console.
 * 
 * @param value The new value of the counter.
 */
void my_callback(int64_t value) {
    std::cout << "[C++] Callback fired! New value = " << value << std::endl;
}

int main() {
	std::cout << "Using Rust lib: " << rust_ffi_demo_version() << "\n";

	// Creating a counter
	CounterHandle* h = nullptr;
	die_on_error(rust_ffi_demo_counter_new(/*initial*/ 42, &h), "counter_new");

	// Smart "deleter" for automatic deallocation
	auto deleter = [](CounterHandle* p){ rust_ffi_demo_counter_free(p); };
	std::unique_ptr<CounterHandle, decltype(deleter)> holder(h, deleter);

	// Registering callback
	die_on_error(rust_ffi_demo_counter_set_callback(holder.get(), my_callback), "set_callback");

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
