use simple_sys_info::{
    cpu::*, disk::disk_utility_meas, memory::memory_consumption_meas, socket::net_socket_read,
    Measurements,
};
use tokio::{sync::mpsc, task /*try_join*/};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(100);
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();

    // Notes: since this code (commented version) is spawning multiple tasks to
    // measure CPU usage, memory consumption, disk utility, and socket statistics,
    // using try_join! allows to wait for all these tasks to complete and gather
    // their results simultaneously.
    // let cpu_meas_handle = task::spawn(async {
    //     let cpu_meas = cpu_usage_meas().await.expect("Error in CpuMeasurement");
    //     cpu_meas // return the cpu_meas from the task::spawn()
    // });

    // Notes: In the uncommented version, we now want to handle streaming data or
    // multiple messages from these tasks. Hence we use mpsc now.
    // mpsc (multi-producer, single-consumer) channels are used for message
    // passing between asynchronous tasks. They are useful when you need to send
    // multiple messages from multiple producers to a single consumer.
    // Now since there is a for loop till 100, tasks were producing multiple
    // results over time and we need to process these results as they come in,
    // mpsc would be a better fit. In the previous scenario, each task produces a
    // single result, making try_join! more suitable in the previous case.
    task::spawn(async move {
        for _ in 0..100 {
            let cpu_meas: Box<dyn Measurements> =
                Box::new(cpu_usage_meas().await.expect("Error in CpuMeasurement"));
            tx.send(cpu_meas)
                .await
                .expect("Error in sending CpuMeasurement");
        }
    });

    task::spawn(async move {
        for _ in 0..100 {
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
        for _ in 0..100 {
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
        let socket_stat: Box<dyn Measurements> = Box::new(
            net_socket_read()
                .await
                .expect("Error in SocketStatMeasurement"),
        );
        tx4.send(socket_stat)
            .await
            .expect("Error in sending SocketStatMeasurement");
    });

    // Use try_join! to await both handles concurrently
    // let (cpu_meas, mem_cons, disk_stat, net_socket_stat) = try_join!(
    //     cpu_meas_handle,
    //     mem_cons_handle,
    //     disk_stats_handle,
    //     socket_stats_handle
    // )?;

    while let Some(res) = rx.recv().await {
        res.print_info();
    }

    Ok(())
}
