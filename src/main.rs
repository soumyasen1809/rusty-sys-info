use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use simple_sys_info::{
    cpu::cpu_usage_meas, disk::disk_utility_meas, memory::memory_consumption_meas,
    socket::net_socket_read, Measurements,
};
use tokio::{sync::mpsc, task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui_handle = tokio::spawn(async {
        spawn_ui_thread()
            .await
            .expect("Issue in spawning UI in main")
    });
    fetch_all_data().await?;
    ui_handle.await?;

    Ok(())
}

async fn fetch_all_data() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(100);
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();

    task::spawn(async move {
        for _ in 0..100 {
            // loop{    // continuously poll data
            let cpu_meas: Box<dyn Measurements> =
                Box::new(cpu_usage_meas().await.expect("Error in CpuMeasurement"));
            tx.send(cpu_meas)
                .await
                .expect("Error in sending CpuMeasurement");
        }
    });

    task::spawn(async move {
        for _ in 0..100 {
            // loop{
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
            // loop{
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
        for _ in 0..100 {
            // loop{
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

    while let Some(res) = rx.recv().await {
        res.print_info();
    }

    Ok(())
}

async fn spawn_ui_thread() -> Result<(), Box<dyn std::error::Error>> {
    let ui_handle = tokio::spawn(async {
        create_ui().await.expect("Issue in opening TUI");
    });
    ui_handle.await?;

    Ok(())
}

async fn create_ui() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events().await?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

async fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}
