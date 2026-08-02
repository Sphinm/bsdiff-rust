pub mod bsdiff;
pub mod ffi;
pub mod utils;

// Re-export for convenience
pub use bsdiff::{BsdiffRust, DiffOptions, PerformanceStats};
pub use utils::{
    check_file_access, compression_ratio_percent, get_compression_ratio, get_file_size,
    get_patch_info, verify_patch, CompressionRatio, PatchInfo,
};
