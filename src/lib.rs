use napi::bindgen_prelude::*;
use napi_derive::napi;

mod bsdiff_rust;
use bsdiff_rust::BsdiffRust;

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