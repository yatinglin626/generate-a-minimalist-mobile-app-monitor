/// nhwm_generate_a_mini.rs
///
/// A minimalist mobile app monitor written in Rust.
///
/// This project aims to create a simple, lightweight mobile app that monitors
/// system resources and provides a clean, intuitive UI for users to view
/// essential information about their device.

// Import necessary crates
extern crate tokio;
extern crate tui;
extern crate sys_info;

use tokio::prelude::*;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use sys_info::{System, SystemExt};

// Define a struct to hold system information
struct SystemInfo {
    cpu_usage: f64,
    memory_usage: f64,
    disk_usage: f64,
    network_speed: f64,
}

impl SystemInfo {
    // Initialize system information
    fn new() -> Self {
        SystemInfo {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_speed: 0.0,
        }
    }

    // Update system information
    fn update(&mut self) {
        let sys = System::new_all();
        self.cpu_usage = sys.global_cpu_info().cpu_usage().unwrap_or(0.0);
        self.memory_usage = sys.memory_info().used_memory_percentage().unwrap_or(0.0);
        self.disk_usage = sys.disk_info("/").used_disk_percentage().unwrap_or(0.0);
        self.network_speed = sys.network_info().unwrap_or(0.0).bytes_recv_per-second();
    }
}

// Define a struct to handle the UI
struct App {
    info: SystemInfo,
}

impl App {
    // Initialize the app
    fn new() -> Self {
        App {
            info: SystemInfo::new(),
        }
    }

    // Run the app
    fn run(&mut self) {
        // Set up the TUI backend
        let backend = CrosstermBackend::new(std::io::stdout()).unwrap();
        let mut terminal = tui::Terminal::new(backend).unwrap();

        // Create a loop to update and draw the UI
        loop {
            self.info.update();

            // Create a layout for the UI
            let size = terminal.size().unwrap();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(2)].as_ref())
                .split(size);

            let title_chunk = chunks[0];
            let info_chunk = chunks[1];

            // Create a block for the title
            let title = Block::default()
                .title("System Monitor")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Rgb(255, 255, 255)));

            // Create a paragraph for the system information
            let info = Paragraph::new(format!(
                "CPU: {:.2}%\nMemory: {:.2}%\nDisk: {:.2}%\nNetwork: {:.2} Mbps",
                self.info.cpu_usage, self.info.memory_usage, self.info.disk_usage, self.info.network_speed
            ))
            .alignment(Alignment::Left)
            .wrap(true);

            // Draw the UI
            terminal
                .draw(|f| {
                    title.render-widget(f, &title_chunk);
                    f.render-widget(info, &info_chunk);
                })
                .unwrap();

            // Update the UI
            tokio::time::pause(std::time::Duration::from-millis(1000));
        }
    }
}

fn main() {
    // Initialize the app
    let mut app = App::new();

    // Run the app
    app.run();
}