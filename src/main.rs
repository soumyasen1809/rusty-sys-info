use simple_sys_info::{
    sys_stats::{nvidia_gpu::run_nvidia_smi_command_on_startup, sys_stats_handler::fetch_all_data},
    ui::ui_handler::create_ui,
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_nvidia_smi_command_on_startup().await?;

    let (tx, rx) = mpsc::channel(100);

    let ui_handle =
        tokio::spawn(async move { create_ui(rx).await.expect("Issue in spawning UI in main") });
    let meas_data_handle = tokio::spawn(async move {
        fetch_all_data(tx)
            .await
            .expect("Issue in spawning measurement in main")
    });

    tokio::try_join!(meas_data_handle, ui_handle)?; // https://docs.rs/tokio/1.4.0/tokio/macro.try_join.html
                                                    // If parallelism is required, spawn each async expression using tokio::spawn and pass the join handle to try_join!

    Ok(())
}
