use tokio::sync::mpsc::{self};

use simple_sys_info::{sys_stats::sys_stats_handler::fetch_all_data, ui::ui_handler::create_ui};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(100);

    let ui_handle =
        tokio::spawn(async move { create_ui(rx).await.expect("Issue in spawning UI in main") });
    fetch_all_data(tx).await?;
    ui_handle.await?;

    Ok(())
}
