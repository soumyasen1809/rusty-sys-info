use std::fmt::Display;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process,
};

use crate::Measurements;

const NVIDIA_GPU_OUTPUT_PATH: &str = "gpu_info.txt";
const NVIDIA_SMI_COMMAND: &str = "nvidia-smi";

pub async fn run_nvidia_smi_command_on_startup() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(NVIDIA_GPU_OUTPUT_PATH)
        .await
        .expect("ERROR: Can not create file to write NVIDIA GPU stats");
    if let Err(e) = process::Command::new(NVIDIA_SMI_COMMAND).output().await {
        println!("nvidia_smi_command not supported: {}", e);
        return Ok(());
    }
    let command_output = process::Command::new(NVIDIA_SMI_COMMAND)
        .args(&["-q"])
        .output()
        .await
        .expect("ERROR: nvdia-smi command does not work!");

    let result = String::from_utf8_lossy(&command_output.stdout);
    file.write_all(result.as_bytes()).await?;

    Ok(())
}

#[derive(Debug, Default, Clone)]
pub struct NvidiaGpuMeasurements {
    gpu_name: String,
    gpu_architecture: String,
    gpu_temp: u64,
    gpu_max_temp: u64,
    power_usage: f64,
    power_capacity: f64,
    gpu_utilization: f64,
}

impl NvidiaGpuMeasurements {
    pub fn new(
        gpu_name: String,
        gpu_architecture: String,
        gpu_temp: u64,
        gpu_max_temp: u64,
        power_usage: f64,
        power_capacity: f64,
        gpu_utilization: f64,
    ) -> Self {
        Self {
            gpu_name,
            gpu_architecture,
            gpu_temp,
            gpu_max_temp,
            power_usage,
            power_capacity,
            gpu_utilization,
        }
    }

    pub fn gpu_name(&self) -> &str {
        &self.gpu_name
    }

    pub fn gpu_architecture(&self) -> &str {
        &self.gpu_architecture
    }

    pub fn gpu_temp(&self) -> u64 {
        self.gpu_temp
    }

    pub fn gpu_max_temp(&self) -> u64 {
        self.gpu_max_temp
    }

    pub fn power_usage(&self) -> f64 {
        self.power_usage
    }

    pub fn power_capacity(&self) -> f64 {
        self.power_capacity
    }

    pub fn gpu_utilization(&self) -> f64 {
        self.gpu_utilization
    }
}

impl Display for NvidiaGpuMeasurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GPU name: \t{} \nGPU temp: \t{} \nGPU util: \t{} \nGPU power: \t{}",
            self.gpu_name(),
            self.gpu_temp(),
            self.gpu_utilization(),
            self.power_usage()
        )
    }
}

impl Measurements for NvidiaGpuMeasurements {
    fn print_info(&self) {
        println!("{}", self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub async fn nvidia_gpu_measurements() -> Result<NvidiaGpuMeasurements, Box<dyn std::error::Error>>
{
    let mut nvidia_gpu_meas = NvidiaGpuMeasurements::default();

    let gpu_file = File::open(NVIDIA_GPU_OUTPUT_PATH).await?;
    let gpu_file_content = BufReader::new(gpu_file);
    let mut lines = gpu_file_content.lines();

    while let Some(l) = lines.next_line().await? {
        if l.contains("Product Name") {
            nvidia_gpu_meas.gpu_name = extract_nvidia_gpu_data(l);
        } else if l.contains("Product Architecture") {
            nvidia_gpu_meas.gpu_architecture = extract_nvidia_gpu_data(l);
        } else if l.contains("GPU Current Temp") {
            nvidia_gpu_meas.gpu_temp = extract_nvidia_gpu_data_numeric_value(l)?;
        } else if l.contains("GPU Max Operating Temp") {
            nvidia_gpu_meas.gpu_max_temp = extract_nvidia_gpu_data_numeric_value(l)?;
        } else if l.contains("Power Draw") && l.contains(" W") {
            nvidia_gpu_meas.power_usage = extract_nvidia_gpu_data_numeric_value(l)?;
        } else if l.contains("Current Power Limit") && l.contains(" W") {
            nvidia_gpu_meas.power_capacity = extract_nvidia_gpu_data_numeric_value(l)?;
        } else if l.contains("Memory") && l.contains(" %") {
            nvidia_gpu_meas.gpu_utilization = extract_nvidia_gpu_data_numeric_value(l)?;
        }
    }

    Ok(nvidia_gpu_meas)
}

fn extract_nvidia_gpu_data(line: String) -> String {
    let splitted_line = line.split(": ").collect::<Vec<&str>>();
    splitted_line[1].to_string()
}

fn extract_nvidia_gpu_data_numeric_value<T>(line: String) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let gpu_val_str = extract_nvidia_gpu_data(line);
    let gpu_val_numeric = gpu_val_str.split(" ").collect::<Vec<&str>>();
    Ok(gpu_val_numeric[0]
        .parse::<T>()
        .expect("ERROR: Can not parse to numeric value"))
}
