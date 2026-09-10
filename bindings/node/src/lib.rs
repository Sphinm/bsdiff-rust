use napi::bindgen_prelude::*;
use napi_derive::napi;

use bsdiff_core::bsdiff::{BsdiffRust, DiffOptions, PerformanceStats};
use bsdiff_core::utils::{
    check_file_access, get_compression_ratio, get_file_size, get_patch_info, verify_patch as verify_patch_util,
};

// ============================================================
// Helpers
// ============================================================

fn to_napi_err(e: impl std::fmt::Display) -> Error {
    Error::from_reason(e.to_string())
}

// ============================================================
// JS ↔ Rust structs
// ============================================================

#[napi(object)]
pub struct PatchInfoJs {
    pub size: f64,
    /// Whether the file has a valid BSDIFF40 format header.
    pub is_bsdiff40: bool,
}

#[napi(object)]
pub struct CompressionRatioJs {
    pub old_size: f64,
    pub new_size: f64,
    pub patch_size: f64,
    pub ratio: f64,
}

#[napi(object)]
pub struct PerformanceStatsJs {
    pub elapsed_ms: f64,
    pub old_size: f64,
    pub new_size: f64,
    pub patch_size: f64,
    pub compression_ratio: f64,
}

#[napi(object)]
pub struct DiffOptionsJs {
    /// Compression level (0-9, default 6).
    pub compression_level: Option<u32>,
    /// Enable parallel processing (default true).
    pub enable_parallel: Option<bool>,
}

impl From<PerformanceStats> for PerformanceStatsJs {
    fn from(s: PerformanceStats) -> Self {
        Self {
            elapsed_ms: s.elapsed_ms as f64,
            old_size: s.old_size as f64,
            new_size: s.new_size as f64,
            patch_size: s.patch_size as f64,
            compression_ratio: s.compression_ratio,
        }
    }
}

impl From<DiffOptionsJs> for DiffOptions {
    fn from(js: DiffOptionsJs) -> Self {
        Self {
            compression_level: js.compression_level.unwrap_or(6),
            enable_parallel: js.enable_parallel.unwrap_or(true),
        }
    }
}

// ============================================================
// Synchronous API
// ============================================================

#[napi]
pub fn diff_sync(old_path: String, new_path: String, patch_path: String) -> Result<()> {
    BsdiffRust::diff(&old_path, &new_path, &patch_path).map_err(to_napi_err)
}

#[napi]
pub fn patch_sync(old_path: String, new_path: String, patch_path: String) -> Result<()> {
    BsdiffRust::patch(&old_path, &new_path, &patch_path).map_err(to_napi_err)
}

#[napi]
pub fn diff_with_stats_sync(old_path: String, new_path: String, patch_path: String) -> Result<PerformanceStatsJs> {
    BsdiffRust::diff_with_stats(&old_path, &new_path, &patch_path)
        .map(Into::into)
        .map_err(to_napi_err)
}

#[napi]
pub fn patch_with_stats_sync(old_path: String, new_path: String, patch_path: String) -> Result<PerformanceStatsJs> {
    BsdiffRust::patch_with_stats(&old_path, &new_path, &patch_path)
        .map(Into::into)
        .map_err(to_napi_err)
}

#[napi]
pub fn diff_with_options_sync(
    old_path: String,
    new_path: String,
    patch_path: String,
    options: DiffOptionsJs,
) -> Result<()> {
    let opts: DiffOptions = options.into();
    BsdiffRust::diff_with_options(&old_path, &new_path, &patch_path, &opts).map_err(to_napi_err)
}

#[napi]
pub fn diff_with_options_and_stats_sync(
    old_path: String,
    new_path: String,
    patch_path: String,
    options: DiffOptionsJs,
) -> Result<PerformanceStatsJs> {
    let opts: DiffOptions = options.into();
    BsdiffRust::diff_with_options_and_stats(&old_path, &new_path, &patch_path, &opts)
        .map(Into::into)
        .map_err(to_napi_err)
}

#[napi]
pub fn verify_patch_sync(old_path: String, new_path: String, patch_path: String) -> Result<bool> {
    verify_patch_util(&old_path, &new_path, &patch_path).map_err(to_napi_err)
}

#[napi]
pub fn get_patch_info_sync(patch_path: String) -> Result<PatchInfoJs> {
    let info = get_patch_info(&patch_path).map_err(to_napi_err)?;
    Ok(PatchInfoJs {
        size: info.size as f64,
        is_bsdiff40: info.is_bsdiff40,
    })
}

#[napi]
pub fn get_file_size_sync(file_path: String) -> Result<f64> {
    get_file_size(&file_path).map(|s| s as f64).map_err(to_napi_err)
}

#[napi]
pub fn check_file_access_sync(file_path: String) -> Result<()> {
    check_file_access(&file_path).map_err(to_napi_err)
}

#[napi]
pub fn get_compression_ratio_sync(
    old_path: String,
    new_path: String,
    patch_path: String,
) -> Result<CompressionRatioJs> {
    let ratio = get_compression_ratio(&old_path, &new_path, &patch_path).map_err(to_napi_err)?;
    Ok(CompressionRatioJs {
        old_size: ratio.old_size as f64,
        new_size: ratio.new_size as f64,
        patch_size: ratio.patch_size as f64,
        ratio: ratio.ratio,
    })
}

// ============================================================
// Async Task definitions
// ============================================================

macro_rules! define_task {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? } => $output:ty, $js_value:ty, |$self:ident| $compute:expr, |$out:ident| $resolve:expr) => {
        pub struct $name { $(pub $field: $ty),* }
        #[napi]
        impl Task for $name {
            type Output = $output;
            type JsValue = $js_value;
            fn compute(&mut $self) -> Result<Self::Output> { $compute }
            fn resolve(&mut self, _env: Env, $out: Self::Output) -> Result<Self::JsValue> {
                Ok($resolve)
            }
        }
    };
}

define_task!(DiffTask { old_path: String, new_path: String, patch_path: String } => (), (), |self| {
    BsdiffRust::diff(&self.old_path, &self.new_path, &self.patch_path).map_err(to_napi_err)
}, |_output| ());

define_task!(PatchTask { old_path: String, new_path: String, patch_path: String } => (), (), |self| {
    BsdiffRust::patch(&self.old_path, &self.new_path, &self.patch_path).map_err(to_napi_err)
}, |_output| ());

define_task!(VerifyPatchTask { old_path: String, new_path: String, patch_path: String } => bool, bool, |self| {
    verify_patch_util(&self.old_path, &self.new_path, &self.patch_path).map_err(to_napi_err)
}, |output| output);

define_task!(DiffWithStatsTask { old_path: String, new_path: String, patch_path: String } => PerformanceStats, PerformanceStatsJs, |self| {
    BsdiffRust::diff_with_stats(&self.old_path, &self.new_path, &self.patch_path).map_err(to_napi_err)
}, |output| output.into());

define_task!(PatchWithStatsTask { old_path: String, new_path: String, patch_path: String } => PerformanceStats, PerformanceStatsJs, |self| {
    BsdiffRust::patch_with_stats(&self.old_path, &self.new_path, &self.patch_path).map_err(to_napi_err)
}, |output| output.into());

define_task!(DiffWithOptionsTask { old_path: String, new_path: String, patch_path: String, options: DiffOptions } => (), (), |self| {
    BsdiffRust::diff_with_options(&self.old_path, &self.new_path, &self.patch_path, &self.options).map_err(to_napi_err)
}, |_output| ());

define_task!(DiffWithOptionsAndStatsTask { old_path: String, new_path: String, patch_path: String, options: DiffOptions } => PerformanceStats, PerformanceStatsJs, |self| {
    BsdiffRust::diff_with_options_and_stats(&self.old_path, &self.new_path, &self.patch_path, &self.options).map_err(to_napi_err)
}, |output| output.into());

// ============================================================
// Async API exports
// ============================================================

#[napi]
pub fn diff(old_path: String, new_path: String, patch_path: String) -> Result<AsyncTask<DiffTask>> {
    Ok(AsyncTask::new(DiffTask {
        old_path,
        new_path,
        patch_path,
    }))
}

#[napi]
pub fn patch(old_path: String, new_path: String, patch_path: String) -> Result<AsyncTask<PatchTask>> {
    Ok(AsyncTask::new(PatchTask {
        old_path,
        new_path,
        patch_path,
    }))
}

#[napi]
pub fn verify_patch(old_path: String, new_path: String, patch_path: String) -> Result<AsyncTask<VerifyPatchTask>> {
    Ok(AsyncTask::new(VerifyPatchTask {
        old_path,
        new_path,
        patch_path,
    }))
}

#[napi]
pub fn diff_with_stats(old_path: String, new_path: String, patch_path: String) -> Result<AsyncTask<DiffWithStatsTask>> {
    Ok(AsyncTask::new(DiffWithStatsTask {
        old_path,
        new_path,
        patch_path,
    }))
}

#[napi]
pub fn patch_with_stats(
    old_path: String,
    new_path: String,
    patch_path: String,
) -> Result<AsyncTask<PatchWithStatsTask>> {
    Ok(AsyncTask::new(PatchWithStatsTask {
        old_path,
        new_path,
        patch_path,
    }))
}

#[napi]
pub fn diff_with_options(
    old_path: String,
    new_path: String,
    patch_path: String,
    options: DiffOptionsJs,
) -> Result<AsyncTask<DiffWithOptionsTask>> {
    let opts: DiffOptions = options.into();
    Ok(AsyncTask::new(DiffWithOptionsTask {
        old_path,
        new_path,
        patch_path,
        options: opts,
    }))
}

/// Generate a patch file with custom options and return performance statistics (async).
#[napi]
pub fn diff_with_options_and_stats(
    old_path: String,
    new_path: String,
    patch_path: String,
    options: DiffOptionsJs,
) -> Result<AsyncTask<DiffWithOptionsAndStatsTask>> {
    let opts: DiffOptions = options.into();
    Ok(AsyncTask::new(DiffWithOptionsAndStatsTask {
        old_path,
        new_path,
        patch_path,
        options: opts,
    }))
}
