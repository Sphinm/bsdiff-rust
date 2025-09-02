use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use zstd::stream::{Encoder as ZstdEncoder, Decoder as ZstdDecoder};
use memmap2::MmapOptions;

/// 最优配置结构体 - 简化版本，只保留核心参数
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Zstd 压缩级别 (1-22，推荐3)
    pub compression_level: i32,
    /// 是否使用快速临时目录
    pub use_fast_temp_dir: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            compression_level: 3,    // 平衡速度和压缩比的最佳选择
            use_fast_temp_dir: true, // 默认启用快速临时目录
        }
    }
}

pub struct BsdiffRust;

impl BsdiffRust {
    /// 生成 bsdiff 补丁文件 (使用最优配置)
    pub fn diff(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        Self::diff_optimized(old_file, new_file, patch_file, &OptimizationConfig::default())
    }

    /// 使用最优配置生成补丁 (内部优化实现)
    pub fn diff_optimized(
        old_file: &str, 
        new_file: &str, 
        patch_file: &str,
        config: &OptimizationConfig
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 快速验证输入文件
        Self::validate_files(old_file, new_file)?;

        // 内存映射文件 - 零拷贝高性能I/O
        let (old_mmap, new_mmap) = Self::create_memory_maps(old_file, new_file)?;

        // 智能选择输出路径 (临时目录优化)
        let patch_path = Self::get_optimal_output_path(patch_file, config.use_fast_temp_dir)?;

        // 创建高性能Zstd编码器
        let mut encoder = Self::create_zstd_encoder(&patch_path, config.compression_level)?;

        // 执行核心diff算法
        bsdiff::diff(&old_mmap[..], &new_mmap[..], &mut encoder)?;
        encoder.finish()?;

        // 原子性移动到最终位置
        Self::finalize_output(&patch_path, patch_file)?;

        Ok(())
    }

    /// 应用 bsdiff 补丁文件 (使用最优配置)
    pub fn patch(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        Self::patch_optimized(old_file, new_file, patch_file, &OptimizationConfig::default())
    }

    /// 使用最优配置应用补丁 (内部优化实现)
    pub fn patch_optimized(
        old_file: &str, 
        new_file: &str, 
        patch_file: &str,
        config: &OptimizationConfig
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 快速验证输入文件
        Self::validate_patch_files(old_file, patch_file)?;

        // 内存映射旧文件 - 零拷贝读取
        let old_mmap = Self::create_single_memory_map(old_file)?;

        // 创建高性能Zstd解码器并应用补丁
        let new_data = Self::decode_and_patch(&old_mmap, patch_file)?;

        // 智能选择输出路径并写入
        Self::write_patched_data(&new_data, new_file, config.use_fast_temp_dir)?;

        Ok(())
    }

    // === 核心优化方法 ===

    /// 创建内存映射 (双文件版本)
    #[inline]
    fn create_memory_maps(old_file: &str, new_file: &str) -> Result<(memmap2::Mmap, memmap2::Mmap), Box<dyn std::error::Error>> {
        let old_file_handle = File::open(old_file)?;
        let new_file_handle = File::open(new_file)?;
        
        let old_mmap = unsafe { MmapOptions::new().map(&old_file_handle)? };
        let new_mmap = unsafe { MmapOptions::new().map(&new_file_handle)? };
        
        Ok((old_mmap, new_mmap))
    }

    /// 创建内存映射 (单文件版本)
    #[inline]
    fn create_single_memory_map(file_path: &str) -> Result<memmap2::Mmap, Box<dyn std::error::Error>> {
        let file_handle = File::open(file_path)?;
        Ok(unsafe { MmapOptions::new().map(&file_handle)? })
    }

    /// 创建高性能Zstd编码器
    #[inline]
    fn create_zstd_encoder(output_path: &Path, compression_level: i32) -> Result<ZstdEncoder<'_, BufWriter<File>>, Box<dyn std::error::Error>> {
        let file_handle = File::create(output_path)?;
        let writer = BufWriter::with_capacity(64 * 1024, file_handle); // 64KB 缓冲区
        Ok(ZstdEncoder::new(writer, compression_level)?)
    }

    /// 解码补丁并应用
    #[inline]
    fn decode_and_patch(old_data: &[u8], patch_file: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let patch_file_handle = File::open(patch_file)?;
        let mut decoder = ZstdDecoder::new(patch_file_handle)?;
        
        let mut new_data = Vec::new();
        bsdiff::patch(old_data, &mut decoder, &mut new_data)?;
        
        Ok(new_data)
    }

    /// 写入补丁数据到文件
    #[inline]
    fn write_patched_data(data: &[u8], output_file: &str, use_fast_temp: bool) -> Result<(), Box<dyn std::error::Error>> {
        let output_path = Self::get_optimal_output_path(output_file, use_fast_temp)?;
        
        let mut writer = BufWriter::with_capacity(64 * 1024, File::create(&output_path)?);
        writer.write_all(data)?;
        writer.flush()?;
        
        Self::finalize_output(&output_path, output_file)
    }

    /// 获取最优输出路径
    #[inline]
    fn get_optimal_output_path(original_path: &str, use_fast_temp: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if use_fast_temp {
            let fast_temp_dir = Self::get_fast_temp_dir();
            let file_name = Path::new(original_path)
                .file_name()
                .ok_or("Invalid file path")?;
            Ok(fast_temp_dir.join(format!("bsdiff_{}", file_name.to_string_lossy())))
        } else {
            Ok(PathBuf::from(original_path))
        }
    }

    /// 原子性完成输出
    #[inline]
    fn finalize_output(temp_path: &Path, final_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if temp_path != Path::new(final_path) {
            std::fs::rename(temp_path, final_path)?;
        }
        Ok(())
    }

    /// 获取最快的临时目录
    #[inline]
    fn get_fast_temp_dir() -> PathBuf {
        // Linux: 内存盘优先
        if cfg!(target_os = "linux") && Path::new("/dev/shm").exists() {
            return PathBuf::from("/dev/shm");
        }
        
        // macOS: 检查RAM盘
        if cfg!(target_os = "macos") {
            if let Ok(entries) = std::fs::read_dir("/Volumes") {
                for entry in entries.flatten() {
                    if entry.file_name().to_string_lossy().contains("RAM") {
                        return entry.path();
                    }
                }
            }
        }
        
        std::env::temp_dir()
    }

    // === 验证方法 ===

    /// 验证diff输入文件
    #[inline]
    fn validate_files(old_file: &str, new_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(old_file).exists() {
            return Err(format!("Old file not found: {}", old_file).into());
        }
        if !Path::new(new_file).exists() {
            return Err(format!("New file not found: {}", new_file).into());
        }
        Ok(())
    }

    /// 验证patch输入文件
    #[inline]
    fn validate_patch_files(old_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(old_file).exists() {
            return Err(format!("Old file not found: {}", old_file).into());
        }
        if !Path::new(patch_file).exists() {
            return Err(format!("Patch file not found: {}", patch_file).into());
        }
        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_optimized_diff_patch() {
        let old_content = b"Hello World! This is the old version with some content.";
        let new_content = b"Hello World! This is the new version with more content and changes.";
        
        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();
        
        fs::write(&old_file, old_content).unwrap();
        fs::write(&new_file, new_content).unwrap();
        
        // 测试最优配置
        BsdiffRust::diff_optimized(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
            &OptimizationConfig::default()
        ).unwrap();
        
        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch_optimized(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap(),
            &OptimizationConfig::default()
        ).unwrap();
        
        let generated_content = fs::read(generated_file.path()).unwrap();
        assert_eq!(generated_content, new_content);
    }



    #[test]
    fn test_default_methods() {
        let old_content = b"Test content for default methods.";
        let new_content = b"Test content for default methods with changes.";
        
        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();
        
        fs::write(&old_file, old_content).unwrap();
        fs::write(&new_file, new_content).unwrap();
        
        // 测试默认方法 (内部使用最优配置)
        BsdiffRust::diff(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap()
        ).unwrap();
        
        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap()
        ).unwrap();
        
        let generated_content = fs::read(generated_file.path()).unwrap();
        assert_eq!(generated_content, new_content);
    }
}