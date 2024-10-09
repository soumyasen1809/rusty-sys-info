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
    fetch_all_data(tx).await?;
    ui_handle.await?;

    Ok(())
}
