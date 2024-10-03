use simple_sys_info::cpu::*;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu_meas_handle = task::spawn(async {
        let cpu_meas = cpu_usage_meas().await.expect("Error in CpuMeasurement");
        cpu_meas // return the cpu_meas from the task::spawn()
    });
    let cpu_meas = cpu_meas_handle.await?;
    for meas in cpu_meas.cpu_time() {
        println!("{}", meas);
    }

    Ok(())
}
