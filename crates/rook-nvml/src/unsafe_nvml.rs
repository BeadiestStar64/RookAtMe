unsafe extern "C" {
    pub(crate) fn rook_nvml_init() -> i32;
    pub(crate) fn rook_nvml_shutdown() -> i32;
    pub(crate) fn rook_nvml_device_count(p_count: *mut u32) -> i32;
}
