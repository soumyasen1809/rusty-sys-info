use std::fmt::Display;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::Measurements;

const CPU_MEAS_PATH: &str = "/proc/stat";

#[derive(Default, Debug)]
pub struct CpuTime {
    cpu_id: String,
    user_time: u64,
    system_time: u64,
    idle_time: u64,
    utilization: f64,
}

impl CpuTime {
    pub fn new(
        cpu_id: String,
        user_time: u64,
        system_time: u64,
        idle_time: u64,
        utilization: f64,
    ) -> Self {
        Self {
            cpu_id,
            user_time,
            system_time,
            idle_time,
            utilization,
        }
    }

    pub fn system_time(&self) -> u64 {
        self.system_time
    }

    pub fn user_time(&self) -> u64 {
        self.user_time
    }

    pub fn idle_time(&self) -> u64 {
        self.idle_time
    }

    pub fn cpu_id(&self) -> &str {
        &self.cpu_id
    }

    pub fn utilization(&self) -> f64 {
        self.utilization
    }
}

impl Display for CpuTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} use: \t{:.3} %", self.cpu_id(), self.utilization()) // print till 3 digits
    }
}

#[derive(Default)]
pub struct CpuMeasurements {
    cpu_time: Vec<CpuTime>,
}

impl CpuMeasurements {
    pub fn new(cpu_time: Vec<CpuTime>) -> Self {
        Self { cpu_time }
    }

    pub fn cpu_time(&self) -> &[CpuTime] {
        &self.cpu_time
    }
}

impl Display for CpuMeasurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.cpu_time())
    }
}

impl Measurements for CpuMeasurements {
    fn print_info(&self) {
        for meas in self.cpu_time() {
            println!("{}", meas);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub async fn cpu_usage_meas() -> Result<CpuMeasurements, Box<dyn std::error::Error>> {
    let mut all_cpus_time: Vec<CpuTime> = Vec::new();

    let cpu_meas_file = File::open(CPU_MEAS_PATH).await?;
    let cpu_meas_contents = BufReader::new(cpu_meas_file);
    let mut line = cpu_meas_contents.lines();
    while let Some(l) = line.next_line().await? {
        if l.contains("cpu") && !l.contains("cpu ")
        // removing the first aggregate cpu value with "cpu "
        {
            let cpu_time = extract_cpu_utilization(l)?;
            all_cpus_time.push(cpu_time);
        }
    }

    let cpu_meas = CpuMeasurements::new(all_cpus_time);

    Ok(cpu_meas)
}

/// *user* : normal processes executing in user mode
/// *nice* : niced processes executing in user mode
/// *system* : processes executing in kernel mode
/// *idle* : twiddling thumbs
/// *iowait* : waiting for I/O to complete
/// *irq* : servicing interrupts
/// *softirq* : servicing softirqs
/// https://www.linuxhowtos.org/System/procstat.htm#:~:text=/proc/stat%20explained%20Various%20pieces%20of%20information%20about%20kernel
fn extract_cpu_utilization(line: String) -> Result<CpuTime, Box<dyn std::error::Error>> {
    let cpu_data = line
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| **s != "")
        .map(|s| s.to_string().clone())
        .collect::<Vec<String>>();
    let mut cpu_time = get_cpu_times(cpu_data)?;
    compute_cpu_utilization(&mut cpu_time);

    Ok(cpu_time)
}

/// awk '{usage=($2+$4)*100/($2+$4+$5)} END {print usage "%"}'
/// https://shreve.io/posts/calculating-current-cpu-usage-on-linux/
fn get_cpu_times(cpu_data: Vec<String>) -> Result<CpuTime, Box<dyn std::error::Error>> {
    let cpu_id = &cpu_data[0];
    let user_time = cpu_data[1].parse::<u64>()?;
    let system_time = cpu_data[3].parse::<u64>()?;
    let idle_time = cpu_data[4].parse::<u64>()?;
    let utilization = 0.0;

    let cpu_time = CpuTime::new(
        cpu_id.to_string(),
        user_time,
        system_time,
        idle_time,
        utilization,
    );

    Ok(cpu_time)
}

fn compute_cpu_utilization(cpu_time: &mut CpuTime) {
    cpu_time.utilization = (cpu_time.user_time() + cpu_time.system_time()) as f64 * 100.0
        / ((cpu_time.user_time() + cpu_time.system_time() + cpu_time.idle_time()) as f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "Values are not approximately equal: {} vs {}",
            a,
            b
        );
    }

    #[test]
    fn test_extract_cpu_utilization() {
        let line = "cpu   4705 0   1234 5678 0 0 0 0 0   0";
        let result = extract_cpu_utilization(line.to_string());

        assert!(result.is_ok());
        let cpu_time = result.unwrap();

        assert_eq!(cpu_time.cpu_id(), "cpu");
        assert_eq!(cpu_time.user_time(), 4705);
        assert_eq!(cpu_time.system_time(), 1234);
        assert_eq!(cpu_time.idle_time(), 5678);
        assert_approx_eq(cpu_time.utilization(), 51.1233, 1e-3);
    }
}
