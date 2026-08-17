pub mod safe_nvml;
mod unsafe_nvml;

pub use safe_nvml::{Nvml, NvmlError};
