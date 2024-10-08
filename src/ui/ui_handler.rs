use std::io::{self, stdout};
use tokio::sync::mpsc::Receiver;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use crate::{
    sys_stats::{
        cpu::CpuMeasurements, disk::DiskStatMeasurements, memory::MemoryMeasurments,
        nvidia_gpu::NvidiaGpuMeasurements, socket::SocketStatMeasurements,
    },
    Measurements,
};

use super::ui_measurements::UIMeasurements;

pub async fn create_ui(mut rx: Receiver<Box<dyn Measurements>>) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut ui_measurements_state = UIMeasurements::default();

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| draw_ui(frame, &mut rx, &mut ui_measurements_state))?;
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

fn draw_ui(
    frame: &mut Frame,
    rx: &mut Receiver<Box<dyn Measurements>>,
    ui_measurements_state: &mut UIMeasurements,
) {
    if let Ok(res) = rx.try_recv() {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(24),
                    Constraint::Percentage(24),
                    Constraint::Percentage(24),
                    Constraint::Percentage(24),
                ]
                .as_ref(),
            )
            .split(frame.area());
        let split_second_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        if let Some(cpu_data) = res.as_any().downcast_ref::<CpuMeasurements>() {
            ui_measurements_state.ui_cpu_data = cpu_data.clone();
        } else if let Some(memory_data) = res.as_any().downcast_ref::<MemoryMeasurments>() {
            ui_measurements_state.ui_memory_data = memory_data.clone();
        } else if let Some(disk_data) = res.as_any().downcast_ref::<DiskStatMeasurements>() {
            ui_measurements_state.ui_disk_data = disk_data.clone();
        } else if let Some(socket_data) = res.as_any().downcast_ref::<SocketStatMeasurements>() {
            ui_measurements_state.ui_socket_data = socket_data.clone();
        } else if let Some(nvidia_gpu_data) = res.as_any().downcast_ref::<NvidiaGpuMeasurements>() {
            ui_measurements_state.ui_nvidia_gpu_data = nvidia_gpu_data.clone();
        }

        frame.render_widget(
            Paragraph::new(format!("{}", ui_measurements_state.ui_cpu_data())).block(
                Block::bordered()
                    .title("CpuInfo")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .border_style(Style::new().red()),
            ),
            chunks[0],
        );
        frame.render_widget(
            Paragraph::new(format!("{}", ui_measurements_state.ui_memory_data())).block(
                Block::bordered()
                    .title("MemoryInfo")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .border_style(Style::new().green()),
            ),
            split_second_chunk[0],
        );
        frame.render_widget(
            Paragraph::new(format!("{}", ui_measurements_state.ui_disk_data())).block(
                Block::bordered()
                    .title("DiskInfo")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .border_style(Style::new().blue()),
            ),
            chunks[2],
        );
        frame.render_widget(
            Paragraph::new(format!("{}", ui_measurements_state.ui_socket_data())).block(
                Block::bordered()
                    .title("SocketInfo")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .border_style(Style::new().green()),
            ),
            split_second_chunk[1],
        );
        frame.render_widget(
            Paragraph::new(format!("{}", ui_measurements_state.ui_nvidia_gpu_data())).block(
                Block::bordered()
                    .title("NvidiaGPU")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .border_style(Style::new().yellow()),
            ),
            chunks[3],
        );
    }
}
