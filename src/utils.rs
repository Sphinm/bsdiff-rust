use std::fs::File;
use std::io::{Read, BufReader};
use zstd::stream::Decoder as ZstdDecoder;

/// 补丁文件信息
#[derive(Debug, Clone)]
pub struct PatchInfo {
    pub size: u64,
    pub compressed: bool,
}

/// 压缩比信息
#[derive(Debug, Clone)]
pub struct CompressionRatio {
    pub old_size: u64,
    pub new_size: u64,
    pub patch_size: u64,
    pub ratio: f64, // 百分比
}

/// 验证补丁文件完整性
pub fn verify_patch(old_file: &str, new_file: &str, patch_file: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // 读取文件
    let mut old_data = Vec::new();
    let mut reader = BufReader::new(File::open(old_file)?);
    reader.read_to_end(&mut old_data)?;
    
    let mut new_data = Vec::new();
    let mut reader = BufReader::new(File::open(new_file)?);
    reader.read_to_end(&mut new_data)?;
    
    // 应用补丁到临时数据
    let patch_file = File::open(patch_file)?;
    let mut reader = ZstdDecoder::new(patch_file)?;
    let mut patched_data = Vec::new();
    
    bsdiff::patch(&old_data, &mut reader, &mut patched_data)?;
    
    // 比较结果
    Ok(patched_data == new_data)
}

/// 获取补丁文件信息
pub fn get_patch_info(patch_file: &str) -> Result<PatchInfo, Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(patch_file)?;
    Ok(PatchInfo {
        size: metadata.len(),
        compressed: true, // 我们总是使用 zstd 压缩
    })
}

/// 计算文件大小（用于进度显示）
pub fn get_file_size(file_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.len())
}

/// 检查文件是否存在且可读
pub fn check_file_access(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }
    if !path.is_file() {
        return Err(format!("Path is not a file: {}", file_path).into());
    }
    // 尝试打开文件以验证可读性
    File::open(file_path)?;
    Ok(())
}

/// 获取压缩比信息
pub fn get_compression_ratio(old_file: &str, new_file: &str, patch_file: &str) -> Result<CompressionRatio, Box<dyn std::error::Error>> {
    let old_size = get_file_size(old_file)?;
    let new_size = get_file_size(new_file)?;
    let patch_size = get_file_size(patch_file)?;
    
    let total_size = old_size + new_size;
    let ratio = if total_size > 0 {
        (patch_size as f64 / total_size as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(CompressionRatio {
        old_size,
        new_size,
        patch_size,
        ratio,
    })
}

