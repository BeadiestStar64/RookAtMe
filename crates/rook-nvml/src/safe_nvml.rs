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

    /// VRAM情報を取得する
    ///
    /// # Error
    ///
    /// VRAM情報の取得に失敗した場合は、 [`NvmlError`] を返す
    pub fn get_vram_info(&self, index: u32) -> Result<VramInfo, NvmlError> {
        // FFI用の構造体を作成
        let mut raw_memory = unsafe_nvml::RookNvmlMemory {
            free: 0,
            reserved: 0,
            total: 0,
            used: 0,
        };

        // SAFETY:
        // 1. `RookNvmlMemory` は `#[repr(C)]` により、
        // C側の `rook_nvml_memory_t` と互換なレイアウトを持つ。
        // 2. `&mut raw_memory` は呼び出し中では、
        // 有効かつ書き込み可能な `RookNvmlMemory` の領域を指している。
        let result = unsafe { unsafe_nvml::rook_nvml_device_memory(index, &mut raw_memory) };

        check_result(result)?;

        Ok(VramInfo {
            free: raw_memory.free,
            reserved: raw_memory.reserved,
            total: raw_memory.total,
            used: raw_memory.used,
        })
    }
}

impl Drop for Nvml {
    fn drop(&mut self) {
        // SAFETY:
        // `Nvml` は `Nvml::init()` 成功時にのみ生成されるため、対応するNVML初期化が存在する
        let _ = unsafe { unsafe_nvml::rook_nvml_shutdown() };
    }
}

/// VRAMのデータを格納する構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VramInfo {
    free: u64,
    reserved: u64,
    total: u64,
    used: u64,
}

impl VramInfo {
    /// 選択したGPUで、現在利用可能なVRAMをバイト単位で返す
    pub fn get_free_vram(&self) -> u64 {
        self.free
    }

    /// 選択したGPUの搭載VRAMをバイト単位で返す
    pub fn get_total_vram(&self) -> u64 {
        self.total
    }
}
