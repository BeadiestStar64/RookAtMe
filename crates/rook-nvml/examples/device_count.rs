use rook_nvml::Nvml;

fn main() -> Result<(), anyhow::Error> {
    let nvml = Nvml::init()?;

    let device_count = nvml.device_count()?;

    println!("Detected NVIDIA GPUs: {device_count}");

    Ok(())
}
