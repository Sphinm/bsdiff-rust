use std::fs::File;
use std::io::{Read, Write};
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;

pub struct BsdiffRust;

impl BsdiffRust {
    /// 生成 bsdiff 补丁文件
    pub fn diff(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 读取旧文件
        let mut old_data = Vec::new();
        File::open(old_file)?.read_to_end(&mut old_data)?;
        
        // 读取新文件
        let mut new_data = Vec::new();
        File::open(new_file)?.read_to_end(&mut new_data)?;
        
        // 创建补丁文件
        let patch_file = File::create(patch_file)?;
        let mut writer = BzEncoder::new(patch_file, Compression::best());
        
        // 生成补丁 - 使用 bsdiff 库的简单 API
        bsdiff::diff(&old_data, &new_data, &mut writer)?;
        
        Ok(())
    }
    
    /// 应用 bsdiff 补丁文件
    pub fn patch(old_file: &str, new_file: &str, patch_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 读取旧文件
        let mut old_data = Vec::new();
        File::open(old_file)?.read_to_end(&mut old_data)?;
        
        // 读取补丁文件
        let patch_file = File::open(patch_file)?;
        let mut reader = BzDecoder::new(patch_file);
        
        // 应用补丁 - 使用 bsdiff 库的简单 API
        let mut new_data = Vec::new();
        bsdiff::patch(&old_data, &mut reader, &mut new_data)?;
        
        // 写入新文件
        let mut new_file = File::create(new_file)?;
        new_file.write_all(&new_data)?;
        
        Ok(())
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_bsdiff_basic() {
        // 创建测试文件
        let old_content = b"Hello World! This is the old version.";
        let new_content = b"Hello World! This is the new version with more content.";
        
        let old_file = NamedTempFile::new().unwrap();
        let new_file = NamedTempFile::new().unwrap();
        let patch_file = NamedTempFile::new().unwrap();
        
        // 写入测试数据
        fs::write(&old_file, old_content).unwrap();
        fs::write(&new_file, new_content).unwrap();
        
        // 生成补丁
        BsdiffRust::diff(
            old_file.path().to_str().unwrap(),
            new_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap()
        ).unwrap();
        
        // 应用补丁
        let generated_file = NamedTempFile::new().unwrap();
        BsdiffRust::patch(
            old_file.path().to_str().unwrap(),
            generated_file.path().to_str().unwrap(),
            patch_file.path().to_str().unwrap()
        ).unwrap();
        
        // 验证结果
        let generated_content = fs::read(generated_file.path()).unwrap();
        assert_eq!(generated_content, new_content);
    }
}
