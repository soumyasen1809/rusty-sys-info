use tokio::{sync::mpsc::Sender, task};

use crate::sys_stats::{
    cpu::cpu_usage_meas, disk::disk_utility_meas, memory::memory_consumption_meas,
    socket::net_socket_read,
};
use crate::Measurements;

use super::nvidia_gpu::nvidia_gpu_measurements;

pub async fn fetch_all_data(
    tx: Sender<Box<dyn Measurements>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();
    let tx5 = tx.clone();

    task::spawn(async move {
        loop {
            // continuously poll data
            let cpu_meas: Box<dyn Measurements> =
                Box::new(cpu_usage_meas().await.expect("Error in CpuMeasurement"));
            tx.send(cpu_meas)
                .await
                .expect("Error in sending CpuMeasurement");
        }
    });

    task::spawn(async move {
        loop {
            let mem_cons: Box<dyn Measurements> = Box::new(
                memory_consumption_meas()
                    .await
                    .expect("Error in MemoryMeasurement"),
            );
            tx2.send(mem_cons)
                .await
                .expect("Error in sending MemoryMeasurement");
        }
    });

    task::spawn(async move {
        loop {
            let disk_util: Box<dyn Measurements> = Box::new(
                disk_utility_meas()
                    .await
                    .expect("Error in DiskStatMeasurement"),
            );
            tx3.send(disk_util)
                .await
                .expect("Error in sending DiskStatMeasurement");
        }
    });

    tokio::spawn(async move {
        loop {
            let socket_stat: Box<dyn Measurements> = Box::new(
                net_socket_read()
                    .await
                    .expect("Error in SocketStatMeasurement"),
            );
            tx4.send(socket_stat)
                .await
                .expect("Error in sending SocketStatMeasurement");
        }
    });

    tokio::spawn(async move {
        loop {
            let nvidia_gpu_stat: Box<dyn Measurements> = Box::new(
                nvidia_gpu_measurements()
                    .await
                    .expect("Error in NvidiaGPUMeasurement"),
            );
            tx5.send(nvidia_gpu_stat)
                .await
                .expect("Error in sending NvidiaGPUMeasurement");
        }
    });

    Ok(())
}
