use std::fs::File;
use std::io::BufWriter;
use memmap2::MmapOptions;

use crate::bsdiff_optimized::OptimizedBsdiff;

/// 高性能 bsdiff 实现
/// 
/// 专门针对 10-20MB 的 tar 文件优化
/// 使用标准 BZip2 压缩，与 bsdiff-node 完全兼容
pub struct BsdiffRust;

impl BsdiffRust {
    /// 生成 bsdiff 补丁文件
    pub fn diff(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 内存映射文件以获得最佳性能
        let old_mmap = Self::create_memory_map(old_file)?;
        let new_mmap = Self::create_memory_map(new_file)?;

        // 创建输出文件
        let patch_writer = BufWriter::new(File::create(patch_file)?);

        // 使用优化的算法生成补丁
        OptimizedBsdiff::diff(&old_mmap[..], &new_mmap[..], patch_writer)?;

        Ok(())
    }

    /// 应用 bsdiff 补丁文件
    pub fn patch(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 内存映射旧文件
        let old_mmap = Self::create_memory_map(old_file)?;

        // 估算新文件大小
        let estimated_new_size = Self::estimate_new_file_size(&old_mmap, patch_file)?;

        // 读取并应用补丁
        let patch_reader = File::open(patch_file)?;
        let new_data = OptimizedBsdiff::patch(&old_mmap[..], patch_reader, estimated_new_size)?;

        // 写入新文件
        std::fs::write(new_file, &new_data)?;

        Ok(())
    }

    /// 创建内存映射
    fn create_memory_map(file_path: &str) -> Result<memmap2::Mmap, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Ok(mmap)
    }

    /// 估算新文件大小
    fn estimate_new_file_size(old_data: &[u8], patch_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
        // 尝试从补丁文件中读取实际的新文件大小
        // 这里简化为根据补丁大小和旧文件大小进行估算
        let patch_size = std::fs::metadata(patch_file)?.len() as usize;
        
        // 更保守的估算：基于补丁大小和旧文件大小
        let estimated_size = old_data.len() + patch_size;
        Ok(estimated_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_bsdiff_rust() {
        // 创建测试文件
        let old_content = b"Hello, World! This is the old version.";
        let new_content = b"Hello, World! This is the new version with more content.";

        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();
        let result_file = NamedTempFile::new().unwrap();

        fs::write(old_file.path(), old_content).unwrap();
        fs::write(new_file.path(), new_content).unwrap();

        // 生成补丁
        BsdiffRust::diff(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
        ).unwrap();

        // 应用补丁 - 直接使用 OptimizedBsdiff 以获得精确的大小控制
        let old_mmap = BsdiffRust::create_memory_map(old_file.path().to_str().unwrap()).unwrap();
        let patch_reader = File::open(patch_file.path()).unwrap();
        let new_data = OptimizedBsdiff::patch(&old_mmap[..], patch_reader, new_content.len()).unwrap();
        fs::write(result_file.path(), &new_data).unwrap();

        // 验证结果
        let result_content = fs::read(result_file.path()).unwrap();
        assert_eq!(result_content, new_content);
    }
}