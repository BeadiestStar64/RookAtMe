unsafe extern "C" {
    pub(crate) fn rook_nvml_init() -> i32;
    pub(crate) fn rook_nvml_shutdown() -> i32;
    pub(crate) fn rook_nvml_device_count(p_count: *mut u32) -> i32;
    pub(crate) fn rook_nvml_device_memory(index: u32, rook_nvml_memory: *mut RookNvmlMemory)
    -> i32;
}

#[repr(C)]
pub(crate) struct RookNvmlMemory {
    pub(crate) free: u64,
    pub(crate) reserved: u64,
    pub(crate) total: u64,
    pub(crate) used: u64,
}
