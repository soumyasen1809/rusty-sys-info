use simple_sys_info::{cpu::*, disk::disk_utility_meas, memory::memory_consumption_meas};
use tokio::{task, try_join};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu_meas_handle = task::spawn(async {
        let cpu_meas = cpu_usage_meas().await.expect("Error in CpuMeasurement");
        cpu_meas // return the cpu_meas from the task::spawn()
    });

    let mem_cons_handle = task::spawn(async {
        let mem_cons = memory_consumption_meas()
            .await
            .expect("Error in MemoryMeasurement");
        mem_cons // return the mem_cons from the task::spawn()
    });

    let disk_stats_handle = task::spawn(async {
        let disk_util = disk_utility_meas()
            .await
            .expect("Error in DiskStatMeasurement");
        disk_util // return the disk_util from the task::spawn()
    });

    // Use try_join! to await both handles concurrently
    let (cpu_meas, mem_cons, disk_stat) =
        try_join!(cpu_meas_handle, mem_cons_handle, disk_stats_handle)?;
    for meas in cpu_meas.cpu_time() {
        println!("{}", meas);
    }
    println!("{}", mem_cons);
    for sd in disk_stat.sd_utilization() {
        println!("{}", sd);
    }

    Ok(())
}
