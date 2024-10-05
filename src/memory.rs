use std::fmt::Display;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

const MEMORY_MEAS_PATH: &str = "/proc/meminfo";

pub struct MemoryMeasurments {
    mem_total: u64,
    mem_free: u64,
    mem_avail: u64,
}

impl MemoryMeasurments {
    pub fn new(mem_total: u64, mem_free: u64, mem_avail: u64) -> Self {
        Self {
            mem_total,
            mem_free,
            mem_avail,
        }
    }

    pub fn mem_total(&self) -> u64 {
        self.mem_total
    }

    pub fn mem_free(&self) -> u64 {
        self.mem_free
    }

    pub fn mem_avail(&self) -> u64 {
        self.mem_avail
    }
}

impl Display for MemoryMeasurments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mem total: \t{} kB\nmem free: \t{} kB\nmem avail: \t{} kB",
            self.mem_total(),
            self.mem_free(),
            self.mem_avail()
        )
    }
}

pub async fn memory_consumption_meas() -> Result<MemoryMeasurments, Box<dyn std::error::Error>> {
    let mut mem_total_val: u64 = 0;
    let mut mem_free_val: u64 = 0;
    let mut mem_avail_val: u64 = 0;

    let mem_file = File::open(MEMORY_MEAS_PATH).await?;
    let mem_meas_content = BufReader::new(mem_file);
    let mut line = mem_meas_content.lines();
    while let Some(l) = line.next_line().await? {
        if l.contains("MemTotal:") {
            let mem_data = l.split(" ").collect::<Vec<&str>>();
            mem_total_val = mem_data[mem_data.len() - 2].parse::<u64>()?;
        } else if l.contains("MemFree:") {
            let mem_data = l.split(" ").collect::<Vec<&str>>();
            mem_free_val = mem_data[mem_data.len() - 2].parse::<u64>()?;
        } else if l.contains("MemAvailable:") {
            let mem_data = l.split(" ").collect::<Vec<&str>>();
            mem_avail_val = mem_data[mem_data.len() - 2].parse::<u64>()?;
        }
    }

    let mem_meas = MemoryMeasurments::new(mem_total_val, mem_free_val, mem_avail_val);

    Ok(mem_meas)
}
