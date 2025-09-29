/* Generated automatically by cbindgen. Do not edit. */

#ifndef RUST_FFI_DEMO_H
#define RUST_FFI_DEMO_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Status / error codes returned by API.
 */
typedef enum RustFfiDemoStatus {
  RustffiOk = 0,
  RustffiNullArg = 1,
  RustffiInvalidArg = 2,
  RustffiInternalError = 3,
} RustFfiDemoStatus;

/**
 * Opaque handle for C/C++.
 */
typedef struct CounterHandle {
  uint8_t _private[0];
} CounterHandle;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

enum RustFfiDemoStatus rust_ffi_demo_counter_new(int64_t initial,
                                                 struct CounterHandle **out_counter);

void rust_ffi_demo_counter_free(struct CounterHandle *handle);

enum RustFfiDemoStatus rust_ffi_demo_counter_increment(struct CounterHandle *handle, int64_t delta);

enum RustFfiDemoStatus rust_ffi_demo_counter_value(const struct CounterHandle *handle,
                                                   int64_t *out_value);

enum RustFfiDemoStatus rust_ffi_demo_counter_reset(struct CounterHandle *handle);

enum RustFfiDemoStatus rust_ffi_demo_counter_set_label(struct CounterHandle *handle,
                                                       const char *label_utf8);

enum RustFfiDemoStatus rust_ffi_demo_counter_get_label(const struct CounterHandle *handle,
                                                       char *out_buf,
                                                       size_t buf_len,
                                                       size_t *out_needed);

const char *rust_ffi_demo_last_error_message(void);

const char *rust_ffi_demo_version(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* RUST_FFI_DEMO_H */
