use std::{error::Error, fmt::Display};

use crate::unsafe_nvml;

const NVML_SUCCESS: i32 = 0;

/// NVML操作中に発生したエラー
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmlError {
    code: i32,
}

impl NvmlError {
    /// NVMLのエラーコードを返す
    pub fn code(&self) -> i32 {
        self.code
    }
}

impl Display for NvmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NVML operation failed with code {}", self.code)
    }
}

impl Error for NvmlError {}

/// 初期化済みのNVMLコンテキスト
#[derive(Debug)]
pub struct Nvml {
    _private: (),
}

// NVMLから返されたエラーコードから、Rust側で成功か失敗かを判定する
fn check_result(result_code: i32) -> Result<(), NvmlError> {
    if result_code == NVML_SUCCESS {
        Ok(())
    } else {
        Err(NvmlError { code: result_code })
    }
}

impl Nvml {
    /// NVMLを初期化する
    ///
    /// # Error
    ///
    /// NVMLの初期化に失敗した場合は、 [`NvmlError`] を返す
    pub fn init() -> Result<Self, NvmlError> {
        // SAFETY:
        // `rook_nvml_init` は引数を取らず、C側で `nvmlInit_v2` を呼び出すだけである。
        let result = unsafe { unsafe_nvml::rook_nvml_init() };

        check_result(result)?;

        Ok(Self { _private: () })
    }

    /// NVMLが認識しているGPUの総数を返す
    ///
    /// # Error
    ///
    /// GPU数の取得に失敗した場合は、 [`NvmlError`] を返す
    pub fn device_count(&self) -> Result<u32, NvmlError> {
        let mut count = 0_u32;

        // SAFETY:
        // `count` は有効な `u32` であり、呼び出し中は書き込み可能な領域として存在する
        let result = unsafe { unsafe_nvml::rook_nvml_device_count(&mut count) };

        check_result(result)?;

        Ok(count)
    }
}

impl Drop for Nvml {
    fn drop(&mut self) {
        // SAFETY:
        // `Nvml` は `Nvml::init()` 成功時にのみ生成されるため、対応するNVML初期化が存在する
        let _ = unsafe { unsafe_nvml::rook_nvml_shutdown() };
    }
}
