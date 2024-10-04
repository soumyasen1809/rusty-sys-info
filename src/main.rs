use simple_sys_info::{cpu::*, memory::memory_consumption_meas};
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

    // Start awaiting the future handles here
    // let cpu_meas = cpu_meas_handle.await?;
    // for meas in cpu_meas.cpu_time() {
    //     println!("{}", meas);
    // }

    // let mem_cons = mem_cons_handle.await?;
    // println!("{}", mem_cons);

    // Alternatively, Use try_join! to await both handles concurrently
    let (cpu_meas, mem_cons) = try_join!(cpu_meas_handle, mem_cons_handle)?;
    for meas in cpu_meas.cpu_time() {
        println!("{}", meas);
    }
    println!("{}", mem_cons);

    Ok(())
}
