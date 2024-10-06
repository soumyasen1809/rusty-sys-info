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
    text::ToText,
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
    if event::poll(std::time::Duration::from_millis(50))? {
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
        // The try_recv method in Tokio is used to attempt to receive a value from
        // a channel without blocking.
        // Unlike recv, which waits for a message to be sent if the channel is
        // empty, try_recv returns immediately, making it useful for scenarios
        // where you want to avoid blocking your task.
        // If you have recv, use it in a while loop, if you have
        // try_recv, use it with if case.
        // while let Some(res) = rx.recv().await

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
                Paragraph::new(format!("{}", cpu_data.to_text()))
                    .block(Block::bordered().title("CpuInfo")),
                chunks[0],
            );
        } else if let Some(memory_data) = res.as_any().downcast_ref::<MemoryMeasurments>() {
            frame.render_widget(
                Paragraph::new(format!("{}", memory_data.to_text()))
                    .block(Block::bordered().title("Memory Info")),
                chunks[1],
            );
        } else if let Some(disk_data) = res.as_any().downcast_ref::<DiskStatMeasurements>() {
            frame.render_widget(
                Paragraph::new(format!("{}", disk_data.to_text()))
                    .block(Block::bordered().title("Disk Info")),
                chunks[2],
            );
        } else if let Some(socket_data) = res.as_any().downcast_ref::<SockStat>() {
            frame.render_widget(
                Paragraph::new(format!("{}", socket_data.to_text()))
                    .block(Block::bordered().title("Socket Info")),
                chunks[3],
            );
        }
    }
}
