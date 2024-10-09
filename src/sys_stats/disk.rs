use std::fmt::Display;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::Measurements;

const DISK_STAT_PATH: &str = "/proc/diskstats";

#[derive(Debug, Clone)]
pub struct Sd {
    name: String,
    version: String, // Major number.Minor number
    reads_completed: u64,
    writes_completed: u64,
    io_in_progress: u64,
}

impl Sd {
    pub fn new(
        name: String,
        version: String,
        reads_completed: u64,
        writes_completed: u64,
        io_in_progress: u64,
    ) -> Self {
        Self {
            name,
            version,
            reads_completed,
            writes_completed,
            io_in_progress,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn reads_completed(&self) -> u64 {
        self.reads_completed
    }

    pub fn writes_completed(&self) -> u64 {
        self.writes_completed
    }

    pub fn io_in_progress(&self) -> u64 {
        self.io_in_progress
    }
}

impl Display for Sd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, \nversion: {}, \nreads completed successfully: {}, \nwrites completed successfully: {}, \nI/Os currently in progress: {}", self.name(), self.version(), self.reads_completed(), self.writes_completed(), self.io_in_progress())
    }
}

/// Note: /proc/diskstats
/// Major number: The major number of the device.
/// Minor number: The minor number of the device.
/// Device name: The name of the device (e.g., sda, sdb, etc.)
/// Reads completed successfully: The number of reads that have been completed successfully.
/// Reads merged: The number of reads that have been merged into a single request.
/// Sectors read: The number of sectors that have been read.
/// Time spent reading (ms): The amount of time (in milliseconds) that has been spent reading.
/// Writes completed: The number of writes that have been completed successfully.
/// Writes merged: The number of writes that have been merged into a single request.
/// Sectors written: The number of sectors that have been written.
/// Time spent writing (ms): The amount of time (in milliseconds) that has been spent writing.
/// I/Os currently in progress: The number of I/O operations that are currently in progress.
/// Time spent doing I/Os (ms): The amount of time (in milliseconds) that has been spent doing I/O operations.
/// Weighted time spent doing I/Os (ms): The amount of time (in milliseconds) that has been spent doing I/O operations, weighted by the time that the I/O operations take.
/// https://cleveruptime.com/docs/files/proc-diskstats
#[derive(Default, Clone)]
pub struct DiskStatMeasurements {
    sd_utilization: Vec<Sd>,
}

impl DiskStatMeasurements {
    pub fn new(sd_utilization: Vec<Sd>) -> Self {
        Self { sd_utilization }
    }

    pub fn sd_utilization(&self) -> &[Sd] {
        &self.sd_utilization
    }
}

impl Display for DiskStatMeasurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.sd_utilization())
    }
}

impl Measurements for DiskStatMeasurements {
    fn print_info(&self) {
        for sd in self.sd_utilization() {
            println!("{}", sd);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub async fn disk_utility_meas() -> Result<DiskStatMeasurements, Box<dyn std::error::Error>> {
    let mut all_disk_utilization: Vec<Sd> = Vec::new();

    let disk_stat_file = File::open(DISK_STAT_PATH).await?;
    let disk_content = BufReader::new(disk_stat_file);
    let mut lines = disk_content.lines();

    while let Some(l) = lines.next_line().await? {
        if l.contains(" sd")
        /*|| l.contains(" hd")*/
        {
            let sd = extract_disk_statistics(l)?;
            all_disk_utilization.push(sd);
        }
    }

    let disk_stat_meas = DiskStatMeasurements::new(all_disk_utilization);

    Ok(disk_stat_meas)
}

fn extract_disk_statistics(line: String) -> Result<Sd, Box<dyn std::error::Error>> {
    let line_values = line
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| **s != "") // filter any empty elemets in the vector
        .map(|s| s.to_string().clone())
        .collect::<Vec<String>>();

    let name: String = line_values[2].clone();
    let version: String = line_values[0].clone() + "." + &line_values[1].clone(); // Major number.Minor number
    let reads_completed: u64 = line_values[3].parse::<u64>()?;
    let writes_completed: u64 = line_values[7].parse::<u64>()?;
    let io_in_progress: u64 = line_values[10].parse::<u64>()?;

    let sd = Sd::new(
        name,
        version,
        reads_completed,
        writes_completed,
        io_in_progress,
    );

    Ok(sd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_disk_statistics() {
        let line = "8   0 sda   157698 0 123456 0 789012 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let result = extract_disk_statistics(line.to_string());

        assert!(result.is_ok());
        let sd = result.unwrap();

        assert_eq!(sd.name, "sda");
        assert_eq!(sd.version, "8.0");
        assert_eq!(sd.reads_completed, 157698);
        assert_eq!(sd.writes_completed, 789012);
        assert_eq!(sd.io_in_progress, 0);
    }
}
