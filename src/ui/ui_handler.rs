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
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use crate::{
    sys_stats::{
        cpu::CpuMeasurements, disk::DiskStatMeasurements, memory::MemoryMeasurments,
        socket::SockStat,
    },
    Measurements,
};

pub async fn create_ui(mut rx: Receiver<Box<dyn Measurements>>) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut rx))?;
        should_quit = handle_events().await?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

async fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_secs(1))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, rx: &mut Receiver<Box<dyn Measurements>>) {
    if let Ok(res) = rx.try_recv() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
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

        if let Some(cpu_data) = res.as_any().downcast_ref::<CpuMeasurements>() {
            frame.render_widget(
                Paragraph::new(format!("{}", cpu_data)).block(Block::bordered().title("CpuInfo")),
                chunks[0],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", MemoryMeasurments::default()))
                    .block(Block::bordered().title("MemoryInfo")),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", DiskStatMeasurements::default()))
                    .block(Block::bordered().title("DiskInfo")),
                chunks[2],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", SockStat::default()))
                    .block(Block::bordered().title("SocketInfo")),
                chunks[3],
            );
        }
        if let Some(memory_data) = res.as_any().downcast_ref::<MemoryMeasurments>() {
            frame.render_widget(
                Paragraph::new(format!("{}", memory_data))
                    .block(Block::bordered().title("MemoryInfo")),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", CpuMeasurements::default()))
                    .block(Block::bordered().title("CpuInfo")),
                chunks[0],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", DiskStatMeasurements::default()))
                    .block(Block::bordered().title("DiskInfo")),
                chunks[2],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", SockStat::default()))
                    .block(Block::bordered().title("SocketInfo")),
                chunks[3],
            );
        }
        if let Some(disk_data) = res.as_any().downcast_ref::<DiskStatMeasurements>() {
            frame.render_widget(
                Paragraph::new(format!("{}", disk_data)).block(Block::bordered().title("DiskInfo")),
                chunks[2],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", CpuMeasurements::default()))
                    .block(Block::bordered().title("CpuInfo")),
                chunks[0],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", MemoryMeasurments::default()))
                    .block(Block::bordered().title("MemoryInfo")),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", SockStat::default()))
                    .block(Block::bordered().title("SocketInfo")),
                chunks[3],
            );
        }
        if let Some(socket_data) = res.as_any().downcast_ref::<SockStat>() {
            frame.render_widget(
                Paragraph::new(format!("{}", socket_data))
                    .block(Block::bordered().title("Socket Info")),
                chunks[3],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", CpuMeasurements::default()))
                    .block(Block::bordered().title("CpuInfo")),
                chunks[0],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", MemoryMeasurments::default()))
                    .block(Block::bordered().title("MemoryInfo")),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(format!("{}", DiskStatMeasurements::default()))
                    .block(Block::bordered().title("DiskInfo")),
                chunks[2],
            );
        }
    }
}
