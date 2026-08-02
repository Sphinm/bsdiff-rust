use std::io::Cursor;
use std::path::Path;
use std::time::Instant;
use qbsdiff::{Bsdiff, Bspatch, ParallelScheme};
use qbsdiff::bsdiff::MAX_LENGTH;

use crate::utils::compression_ratio_percent;

/// Performance statistics.
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub elapsed_ms: u64,
    pub old_size: u64,
    pub new_size: u64,
    pub patch_size: u64,
    /// patch_size / new_size as a percentage.
    pub compression_ratio: f64,
}

/// Diff configuration options.
#[derive(Debug, Clone)]
pub struct DiffOptions {
    /// Compression level (0-9).
    pub compression_level: u32,
    /// Whether to enable parallel processing.
    pub enable_parallel: bool,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            compression_level: 6,
            enable_parallel: true,
        }
    }
}

/// Validate that a file path points to an existing regular file.
fn validate_input(path: &str, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(format!("{} not found: {}", label, path).into());
    }
    if !p.is_file() {
        return Err(format!("{} is not a file: {}", label, path).into());
    }
    Ok(())
}

pub struct BsdiffRust;

impl BsdiffRust {
    /// Generate a standard BSDIFF40 format patch file.
    pub fn diff(old_path: &str, new_path: &str, patch_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Self::diff_with_options(old_path, new_path, patch_path, &DiffOptions::default())
    }

    /// Generate a patch file with custom options.
    pub fn diff_with_options(
        old_path: &str,
        new_path: &str,
        patch_path: &str,
        options: &DiffOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        validate_input(old_path, "Old file")?;
        validate_input(new_path, "New file")?;

        let old_data = std::fs::read(old_path)?;
        let new_data = std::fs::read(new_path)?;

        if old_data.len() > MAX_LENGTH {
            return Err(format!(
                "Old file too large: {} bytes (max: {} bytes)",
                old_data.len(), MAX_LENGTH
            ).into());
        }
        if new_data.len() > MAX_LENGTH {
            return Err(format!(
                "New file too large: {} bytes (max: {} bytes)",
                new_data.len(), MAX_LENGTH
            ).into());
        }

        let parallel_scheme = if options.enable_parallel {
            ParallelScheme::Auto
        } else {
            ParallelScheme::Never
        };

        let mut patch_data = Vec::new();
        Bsdiff::new(&old_data, &new_data)
            .compression_level(options.compression_level)
            .parallel_scheme(parallel_scheme)
            .compare(Cursor::new(&mut patch_data))?;

        std::fs::write(patch_path, patch_data)?;

        Ok(())
    }

    /// Generate a patch file and return performance statistics.
    pub fn diff_with_stats(
        old_path: &str,
        new_path: &str,
        patch_path: &str,
    ) -> Result<PerformanceStats, Box<dyn std::error::Error>> {
        Self::diff_with_options_and_stats(old_path, new_path, patch_path, &DiffOptions::default())
    }

    /// Generate a patch file with custom options and return performance statistics.
    pub fn diff_with_options_and_stats(
        old_path: &str,
        new_path: &str,
        patch_path: &str,
        options: &DiffOptions,
    ) -> Result<PerformanceStats, Box<dyn std::error::Error>> {
        let start = Instant::now();

        Self::diff_with_options(old_path, new_path, patch_path, options)?;

        let elapsed = start.elapsed();
        let old_size = std::fs::metadata(old_path)?.len();
        let new_size = std::fs::metadata(new_path)?.len();
        let patch_size = std::fs::metadata(patch_path)?.len();

        Ok(PerformanceStats {
            elapsed_ms: elapsed.as_millis() as u64,
            old_size,
            new_size,
            patch_size,
            compression_ratio: compression_ratio_percent(patch_size, new_size),
        })
    }

    /// Apply a standard BSDIFF40 format patch file.
    pub fn patch(old_path: &str, new_path: &str, patch_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        validate_input(old_path, "Old file")?;
        validate_input(patch_path, "Patch file")?;

        let old_data = std::fs::read(old_path)?;

        if old_data.len() > MAX_LENGTH {
            return Err(format!(
                "Old file too large: {} bytes (max: {} bytes)",
                old_data.len(), MAX_LENGTH
            ).into());
        }

        let patch_data = std::fs::read(patch_path)?;
        let patcher = Bspatch::new(&patch_data)?;

        let hint = patcher.hint_target_size();
        let cap = usize::try_from(hint)
            .map_err(|_| format!("Target file size too large for this platform: {} bytes", hint))?;
        let mut new_data = Vec::with_capacity(cap);
        patcher.apply(&old_data, Cursor::new(&mut new_data))?;

        std::fs::write(new_path, new_data)?;

        Ok(())
    }

    /// Apply a patch file and return performance statistics.
    pub fn patch_with_stats(
        old_path: &str,
        new_path: &str,
        patch_path: &str,
    ) -> Result<PerformanceStats, Box<dyn std::error::Error>> {
        let start = Instant::now();

        Self::patch(old_path, new_path, patch_path)?;

        let elapsed = start.elapsed();
        let old_size = std::fs::metadata(old_path)?.len();
        let new_size = std::fs::metadata(new_path)?.len();
        let patch_size = std::fs::metadata(patch_path)?.len();

        Ok(PerformanceStats {
            elapsed_ms: elapsed.as_millis() as u64,
            old_size,
            new_size,
            patch_size,
            compression_ratio: compression_ratio_percent(patch_size, new_size),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_qbsdiff_diff_patch() {
        let old_content = b"Hello World! This is the old version with some content.";
        let new_content = b"Hello World! This is the new version with more content and changes.";

        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();

        fs::write(&old_file, old_content).unwrap();
        fs::write(&new_file, new_content).unwrap();

        BsdiffRust::diff(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let patch_data = fs::read(patch_file.path()).unwrap();
        assert_eq!(&patch_data[0..8], b"BSDIFF40");

        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let generated_content = fs::read(generated_file.path()).unwrap();
        assert_eq!(generated_content, new_content);
    }

    #[test]
    fn test_diff_with_stats() {
        let old_content = b"Hello World! This is the old version.";
        let new_content = b"Hello World! This is the new version with more data.";

        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();

        fs::write(&old_file, old_content).unwrap();
        fs::write(&new_file, new_content).unwrap();

        let stats = BsdiffRust::diff_with_stats(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        assert_eq!(stats.old_size, old_content.len() as u64);
        assert_eq!(stats.new_size, new_content.len() as u64);
        assert!(stats.patch_size > 0);
        assert!(stats.compression_ratio > 0.0);

        let expected_ratio = (stats.patch_size as f64 / stats.new_size as f64) * 100.0;
        assert!((stats.compression_ratio - expected_ratio).abs() < f64::EPSILON);
    }

    #[test]
    fn test_diff_with_options() {
        let old_content = b"Test data for parallel option";
        let new_content = b"Test data for parallel option modified";

        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();

        fs::write(&old_file, old_content).unwrap();
        fs::write(&new_file, new_content).unwrap();

        let options = DiffOptions {
            compression_level: 9,
            enable_parallel: false,
        };

        BsdiffRust::diff_with_options(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
            &options,
        ).unwrap();

        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let generated_content = fs::read(generated_file.path()).unwrap();
        assert_eq!(generated_content, new_content);
    }

    #[test]
    fn test_file_not_found_errors() {
        let temp = NamedTempFile::new().unwrap();

        let result = BsdiffRust::diff(
            "/nonexistent/old.bin",
            temp.path().to_str().unwrap(),
            temp.path().to_str().unwrap(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Old file not found"));

        let result = BsdiffRust::patch(
            temp.path().to_str().unwrap(),
            temp.path().to_str().unwrap(),
            "/nonexistent/patch.bin",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Patch file not found"));
    }

    #[test]
    fn test_identical_files() {
        let content = b"Identical content for both files";

        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();

        fs::write(&old_file, content).unwrap();
        fs::write(&new_file, content).unwrap();

        BsdiffRust::diff(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let generated_content = fs::read(generated_file.path()).unwrap();
        assert_eq!(generated_content, content);

        let patch_data = fs::read(patch_file.path()).unwrap();
        assert!(patch_data.len() >= 8);
        assert_eq!(&patch_data[0..8], b"BSDIFF40");
    }

    #[test]
    fn test_empty_files() {
        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();

        fs::write(&old_file, b"").unwrap();
        fs::write(&new_file, b"").unwrap();

        BsdiffRust::diff(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        let generated_content = fs::read(generated_file.path()).unwrap();
        assert!(generated_content.is_empty());

        let stats = BsdiffRust::diff_with_stats(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();
        assert_eq!(stats.old_size, 0);
        assert_eq!(stats.new_size, 0);
        assert_eq!(stats.compression_ratio, 0.0);
    }

    #[test]
    fn test_corrupted_patch() {
        let old_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();
        let output_file = NamedTempFile::new().unwrap();

        fs::write(&old_file, b"some original content").unwrap();
        fs::write(&patch_file, b"this is not a valid bsdiff patch").unwrap();

        let result = BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            output_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_patch_info_short_file() {
        let patch_file = NamedTempFile::new().unwrap();
        fs::write(&patch_file, b"SHORT").unwrap();

        let info = crate::utils::get_patch_info(patch_file.path().to_str().unwrap()).unwrap();
        assert_eq!(info.size, 5);
        assert!(!info.is_bsdiff40);
    }

    #[test]
    fn test_patch_info_valid_header() {
        let patch_file = NamedTempFile::new().unwrap();
        let mut data = b"BSDIFF40".to_vec();
        data.extend_from_slice(&[0u8; 24]);
        fs::write(&patch_file, &data).unwrap();

        let info = crate::utils::get_patch_info(patch_file.path().to_str().unwrap()).unwrap();
        assert_eq!(info.size, 32);
        assert!(info.is_bsdiff40);
    }

    #[test]
    fn test_directory_as_input() {
        let temp = NamedTempFile::new().unwrap();
        let dir = std::env::temp_dir();
        let dir_str = dir.to_str().unwrap();

        let result = BsdiffRust::diff(dir_str, temp.path().to_str().unwrap(), temp.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is not a file"));
    }
}
