// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Terminal User Interface
//!
//! Full-screen interactive UI using Ratatui

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fusion_agent_core::AgentSession;
use fusion_settings::Settings;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io::{stdout, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal })
    }

    pub fn enter(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn draw(&mut self, session: &AgentSession, settings: &Settings) -> Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(1),    // Body
                    Constraint::Length(3), // Footer
                ])
                .split(frame.size());

            // HEADER
            let header_text = format!(
                " Fusion VSC CLI Coder | Mode: {} | ID: {} ",
                session.mode.mode_type,
                &session.id[..8]
            );

            let header = Paragraph::new(header_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center);

            frame.render_widget(header, chunks[0]);

            // BODY - Main Content
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70), // Task/Chat
                    Constraint::Percentage(30), // Context/Status
                ])
                .split(chunks[1]);

            // Chat/Output Area
            let output_block = Block::default()
                .title(" Activity Log ")
                .borders(Borders::ALL);
            let output = Paragraph::new("Agent initialized. Waiting for task...")
                .block(output_block)
                .wrap(Wrap { trim: true });

            frame.render_widget(output, content_chunks[0]);

            // Status/Details Area
            let status_block = Block::default().title(" Status ").borders(Borders::ALL);
            let status_text = format!(
                "Secure Mode: {}\nSettings Checked: true\n\nTask Group: None",
                session.is_secure()
            );
            let status = Paragraph::new(status_text).block(status_block);

            frame.render_widget(status, content_chunks[1]);

            // FOOTER - Input
            let input_block = Block::default()
                .title(" Input ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));

            let input = Paragraph::new("Type your instruction here... (Press 'Esc' to quit)")
                .block(input_block);

            frame.render_widget(input, chunks[2]);
        })?;
        Ok(())
    }
}

pub async fn run_interactive(mut session: AgentSession, settings: Settings) -> Result<()> {
    // Setup TUI
    let mut tui = Tui::new()?;
    tui.enter()?;

    // Main Loop
    loop {
        tui.draw(&session, &settings)?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            break;
                        }
                        // TODO: Handle input
                        _ => {}
                    }
                }
            }
        }
    }

    // Restore terminal
    tui.exit()?;
    Ok(())
}
