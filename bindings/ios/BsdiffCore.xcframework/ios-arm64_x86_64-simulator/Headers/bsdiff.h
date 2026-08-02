// bsdiff.h — C header for bsdiff-core FFI
// Generated manually from core/src/ffi.rs
// Include this header in your iOS project and link libbsdiff_core.a

#ifndef BSDIFF_CORE_H
#define BSDIFF_CORE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

/// Retrieve the last error message (caller must free with bsdiff_free_string).
/// Returns NULL if no error.
char* bsdiff_last_error(void);

/// Free a string returned by bsdiff_last_error.
void bsdiff_free_string(char* s);

// ---------------------------------------------------------------------------
// File info
// ---------------------------------------------------------------------------

/// Get file size in bytes. Returns -1 on error.
int64_t bsdiff_file_size(const char* path);

/// Check if a file is a valid BSDIFF40 patch.
/// Returns 1 = valid, 0 = invalid, -1 = error.
int32_t bsdiff_is_valid_patch(const char* patch_path);

// ---------------------------------------------------------------------------
// Core operations
// ---------------------------------------------------------------------------

/// Generate a BSDIFF40 patch: old + new → patch.
/// Returns 0 on success, -1 on error.
int32_t bsdiff(const char* old_path, const char* new_path, const char* patch_path);

/// Apply a BSDIFF40 patch: old + patch → new.
/// Returns 0 on success, -1 on error.
int32_t bspatch(const char* old_path, const char* new_path, const char* patch_path);

// ---------------------------------------------------------------------------
// Advanced operations
// ---------------------------------------------------------------------------

/// Generate a patch with custom compression level (0-9).
/// Returns 0 on success, -1 on error.
int32_t bsdiff_with_compression(const char* old_path, const char* new_path,
                                const char* patch_path, int32_t compression_level);

/// Verify that applying patch to old produces expected_new.
/// Returns 1 = identical, 0 = different, -1 = error.
int32_t bsdiff_verify(const char* old_path, const char* expected_new_path,
                      const char* patch_path);

/// Compression ratio result.
typedef struct {
    uint64_t old_size;
    uint64_t new_size;
    uint64_t patch_size;
    double   ratio_percent;
} BsdiffRatio;

/// Compute compression ratio (patch_size / new_size * 100).
/// Returns 0 on success, -1 on error.
int32_t bsdiff_compression_ratio(const char* old_path, const char* new_path,
                                 const char* patch_path, BsdiffRatio* out);

#ifdef __cplusplus
}
#endif

#endif // BSDIFF_CORE_H
