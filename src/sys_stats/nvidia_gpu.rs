use std::fmt::Display;

use tokio::{fs::File, io::AsyncWriteExt, process};

pub async fn run_nvidia_smi_command_on_startup() -> Result<(), Box<dyn std::error::Error>> {
    let command_output = process::Command::new("nvidia-smi").output().await?;

    let result = String::from_utf8_lossy(&command_output.stdout);
    let mut file = File::create("gpu_info.txt").await?;
    file.write_all(result.as_bytes()).await?;

    Ok(())
}

#[derive(Debug, Default, Clone)]
pub struct NvidiaGpuMeasurements {
    gpu_name: String,
    gpu_temp: u64,
    power_usage: u64,
    power_capacity: u64,
    memory_usage: u64,
    memory_capacity: u64,
    gpu_utilization: u64,
}

impl NvidiaGpuMeasurements {
    pub fn new(
        gpu_name: String,
        gpu_temp: u64,
        power_usage: u64,
        power_capacity: u64,
        memory_usage: u64,
        memory_capacity: u64,
        gpu_utilization: u64,
    ) -> Self {
        Self {
            gpu_name,
            gpu_temp,
            power_usage,
            power_capacity,
            memory_usage,
            memory_capacity,
            gpu_utilization,
        }
    }

    pub fn gpu_name(&self) -> &str {
        &self.gpu_name
    }

    pub fn gpu_temp(&self) -> u64 {
        self.gpu_temp
    }

    pub fn power_usage(&self) -> u64 {
        self.power_usage
    }

    pub fn memory_usage(&self) -> u64 {
        self.memory_usage
    }

    pub fn memory_capacity(&self) -> u64 {
        self.memory_capacity
    }

    pub fn power_capacity(&self) -> u64 {
        self.power_capacity
    }

    pub fn gpu_utilization(&self) -> u64 {
        self.gpu_utilization
    }
}

impl Display for NvidiaGpuMeasurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GPU name: \t{} \nGPU temp: \t{} \nGPU util: \t{} \n",
            self.gpu_name(),
            self.gpu_temp(),
            self.gpu_utilization()
        )
    }
}
