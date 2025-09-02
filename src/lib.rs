use napi::bindgen_prelude::*;
use napi_derive::napi;

mod bsdiff_rust;
mod utils;
use bsdiff_rust::BsdiffRust;
use utils::{verify_patch as verify_patch_util, get_patch_info, get_file_size, check_file_access, get_compression_ratio};

fn call_bsdiff(
  old_str: &str,
  new_str: &str,
  patch: &str,
) -> Result<()> {
  BsdiffRust::diff(old_str, new_str, patch)
    .map_err(|e| Error::from_reason(e.to_string()))
}

fn call_bspatch(
  old_str: &str,
  new_str: &str,
  patch: &str,
) -> Result<()> {
  BsdiffRust::patch(old_str, new_str, patch)
    .map_err(|e| Error::from_reason(e.to_string()))
}

#[napi]
pub fn diff_sync(old_str: String, new_str: String, patch: String) -> Result<()> {
  call_bsdiff(&old_str, &new_str, &patch)
}

#[napi]
pub fn patch_sync(old_str: String, new_str: String, patch: String) -> Result<()> {
  call_bspatch(&old_str, &new_str, &patch)
}

/// 验证补丁文件完整性
#[napi]
pub fn verify_patch_sync(old_str: String, new_str: String, patch: String) -> Result<bool> {
  verify_patch_util(&old_str, &new_str, &patch)
    .map_err(|e| Error::from_reason(e.to_string()))
}

/// 获取补丁文件信息
#[napi]
pub fn get_patch_info_sync(patch: String) -> Result<PatchInfoJs> {
  let info = get_patch_info(&patch)
    .map_err(|e| Error::from_reason(e.to_string()))?;
  
  Ok(PatchInfoJs {
    size: info.size as f64,
    compressed: info.compressed,
  })
}

/// 获取文件大小
#[napi]
pub fn get_file_size_sync(file_path: String) -> Result<f64> {
  get_file_size(&file_path)
    .map(|size| size as f64)
    .map_err(|e| Error::from_reason(e.to_string()))
}

/// 检查文件访问权限
#[napi]
pub fn check_file_access_sync(file_path: String) -> Result<()> {
  check_file_access(&file_path)
    .map_err(|e| Error::from_reason(e.to_string()))
}

/// 获取压缩比信息
#[napi]
pub fn get_compression_ratio_sync(old_str: String, new_str: String, patch: String) -> Result<CompressionRatioJs> {
  let ratio = get_compression_ratio(&old_str, &new_str, &patch)
    .map_err(|e| Error::from_reason(e.to_string()))?;
  
  Ok(CompressionRatioJs {
    old_size: ratio.old_size as f64,
    new_size: ratio.new_size as f64,
    patch_size: ratio.patch_size as f64,
    ratio: ratio.ratio,
  })
}

/// JavaScript 补丁信息结构
#[napi(object)]
pub struct PatchInfoJs {
  pub size: f64,
  pub compressed: bool,
}

/// JavaScript 压缩比信息结构
#[napi(object)]
pub struct CompressionRatioJs {
  pub old_size: f64,
  pub new_size: f64,
  pub patch_size: f64,
  pub ratio: f64,
}

// 简化的异步版本，暂时不包含进度回调
pub struct DiffTask {
  old_str: String,
  new_str: String,
  patch: String,
}

#[napi]
impl Task for DiffTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    call_bsdiff(&self.old_str, &self.new_str, &self.patch)
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct PatchTask {
  old_str: String,
  new_str: String,
  patch: String,
}

#[napi]
impl Task for PatchTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    call_bspatch(&self.old_str, &self.new_str, &self.patch)
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct VerifyPatchTask {
  old_str: String,
  new_str: String,
  patch: String,
}

#[napi]
impl Task for VerifyPatchTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    verify_patch_util(&self.old_str, &self.new_str, &self.patch)
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn diff(
  old_str: String,
  new_str: String,
  patch: String,
) -> Result<AsyncTask<DiffTask>> {
  Ok(AsyncTask::new(DiffTask { old_str, new_str, patch }))
}

#[napi]
pub fn patch(
  old_str: String,
  new_str: String,
  patch: String,
) -> Result<AsyncTask<PatchTask>> {
  Ok(AsyncTask::new(PatchTask { old_str, new_str, patch }))
}

#[napi]
pub fn verify_patch(
  old_str: String,
  new_str: String,
  patch: String,
) -> Result<AsyncTask<VerifyPatchTask>> {
  Ok(AsyncTask::new(VerifyPatchTask { old_str, new_str, patch }))
} 