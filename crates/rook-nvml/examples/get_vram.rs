use rook_nvml::Nvml;

fn main() -> Result<(), anyhow::Error> {
    let nvml = Nvml::init()?;

    let vram_info = nvml.get_vram_info(0)?;

    println!(
        "デバイスの物理VRAM: {:.2}GB",
        vram_info.get_total_vram() as f64 / 1024_f64.powi(3)
    );

    println!(
        "利用可能なVRAM: {:.2} GB",
        vram_info.get_free_vram() as f64 / 1024_f64.powi(3)
    );

    Ok(())
}
