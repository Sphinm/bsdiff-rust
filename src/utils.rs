use std::fs::File;
use std::io::Read;

/// Patch file information.
#[derive(Debug, Clone)]
pub struct PatchInfo {
    pub size: u64,
    /// Whether the file has a valid BSDIFF40 format header.
    pub is_bsdiff40: bool,
}

/// Compression ratio information.
#[derive(Debug, Clone)]
pub struct CompressionRatio {
    pub old_size: u64,
    pub new_size: u64,
    pub patch_size: u64,
    /// patch_size / new_size as a percentage.
    pub ratio: f64,
}

/// Compute compression ratio: patch_size relative to new_size (percentage).
pub fn compression_ratio_percent(patch_size: u64, new_size: u64) -> f64 {
    if new_size > 0 {
        (patch_size as f64 / new_size as f64) * 100.0
    } else {
        0.0
    }
}

/// Verify patch integrity by applying it and comparing with the expected new file.
///
/// Returns `Ok(true)` if the patch produces identical output, `Ok(false)` if content differs.
/// Returns `Err` if input files are missing or the patch is corrupt/unapplyable.
pub fn verify_patch(old_path: &str, new_path: &str, patch_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let new_data = std::fs::read(new_path)?;

    let temp_file = tempfile::NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().ok_or("Invalid temp path")?;

    crate::bsdiff_rust::BsdiffRust::patch(old_path, temp_path, patch_path)?;

    let patched_data = std::fs::read(temp_path)?;

    Ok(patched_data == new_data)
}

/// Get patch file information (size and whether it has a valid BSDIFF40 header).
pub fn get_patch_info(patch_path: &str) -> Result<PatchInfo, Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(patch_path)?;
    let size = metadata.len();

    if size < 8 {
        return Ok(PatchInfo { size, is_bsdiff40: false });
    }

    let mut file = File::open(patch_path)?;
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;

    Ok(PatchInfo {
        size,
        is_bsdiff40: &header == b"BSDIFF40",
    })
}

/// Get file size in bytes.
pub fn get_file_size(file_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.len())
}

/// Check whether a file exists and is readable.
pub fn check_file_access(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }
    if !path.is_file() {
        return Err(format!("Path is not a file: {}", file_path).into());
    }
    File::open(file_path)?;
    Ok(())
}

/// Get compression ratio information.
pub fn get_compression_ratio(old_path: &str, new_path: &str, patch_path: &str) -> Result<CompressionRatio, Box<dyn std::error::Error>> {
    let old_size = get_file_size(old_path)?;
    let new_size = get_file_size(new_path)?;
    let patch_size = get_file_size(patch_path)?;

    Ok(CompressionRatio {
        old_size,
        new_size,
        patch_size,
        ratio: compression_ratio_percent(patch_size, new_size),
    })
}
