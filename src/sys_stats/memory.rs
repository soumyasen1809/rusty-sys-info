use std::fmt::Display;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::Measurements;

const MEMORY_MEAS_PATH: &str = "/proc/meminfo";

#[derive(Default)]
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

impl Measurements for MemoryMeasurments {
    fn print_info(&self) {
        println!("{}", self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
            mem_total_val = extract_mem_consumption(l)?;
        } else if l.contains("MemFree:") {
            mem_free_val = extract_mem_consumption(l)?;
        } else if l.contains("MemAvailable:") {
            mem_avail_val = extract_mem_consumption(l)?;
        }
    }

    let mem_meas = MemoryMeasurments::new(mem_total_val, mem_free_val, mem_avail_val);

    Ok(mem_meas)
}

fn extract_mem_consumption(line: String) -> Result<u64, Box<dyn std::error::Error>> {
    let mem_data = line
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| **s != "")
        .map(|s| s.to_string().clone())
        .collect::<Vec<String>>();
    Ok(mem_data[mem_data.len() - 2].parse::<u64>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mem_consumption() {
        let line = "MemTotal:   16384   kB";
        let result = extract_mem_consumption(line.to_string());

        assert!(result.is_ok());
        let mem_consumption = result.unwrap();

        assert_eq!(mem_consumption, 16384);
    }
}
