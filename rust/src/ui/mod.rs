mod widgets;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::io;

use crate::app::{App, InputMode};
use crate::model::Resource;

/// Enter the TUI event loop.
pub async fn run(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut table_state = TableState::default();
    table_state.select(Some(0));

    while app.running {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // header / crumbs
                    Constraint::Min(5),    // main resource table
                    Constraint::Length(1), // command bar / status
                ])
                .split(frame.area());

            // Header breadcrumb.
            let ns_label = app.namespace.as_deref().unwrap_or("all");
            let header = Line::from(vec![
                Span::styled("r9s", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" > "),
                Span::styled(ns_label, Style::default().fg(Color::Yellow)),
                Span::raw(" > "),
                Span::styled(
                    app.active_resource.to_string(),
                    Style::default().fg(Color::Green),
                ),
            ]);
            frame.render_widget(Paragraph::new(header), chunks[0]);

            // Resource table.
            let header_cells: Vec<Cell> = app
                .table
                .headers
                .iter()
                .map(|h| Cell::from(h.as_str()).style(Style::default().fg(Color::Yellow)))
                .collect();
            let header_row = Row::new(header_cells).height(1);

            let rows: Vec<Row> = app
                .table
                .rows
                .iter()
                .map(|r| {
                    let mut cells = vec![
                        Cell::from(r.namespace.as_str()),
                        Cell::from(r.name.as_str()),
                    ];
                    for col in &r.columns {
                        cells.push(Cell::from(col.as_str()));
                    }
                    cells.push(Cell::from(r.age.as_str()));
                    Row::new(cells)
                })
                .collect();

            let widths: Vec<Constraint> = app
                .table
                .headers
                .iter()
                .map(|_| Constraint::Percentage(100 / app.table.headers.len().max(1) as u16))
                .collect();

            let table = Table::new(rows, &widths)
                .header(header_row)
                .block(Block::default().borders(Borders::ALL).title(app.active_resource.to_string()))
                .row_highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

            table_state.select(Some(app.selected_row));
            frame.render_stateful_widget(table, chunks[1], &mut table_state);

            // Command / status bar.
            let status = match app.input_mode {
                InputMode::Command => {
                    Line::from(vec![
                        Span::styled(":", Style::default().fg(Color::Cyan)),
                        Span::raw(&app.command_input),
                    ])
                }
                InputMode::Filter => {
                    Line::from(vec![
                        Span::styled("/", Style::default().fg(Color::Cyan)),
                        Span::raw(&app.filter_input),
                    ])
                }
                InputMode::Normal => {
                    Line::from(Span::styled(
                        "<:> cmd  </>filter  <q>uit",
                        Style::default().fg(Color::DarkGray),
                    ))
                }
            };
            frame.render_widget(Paragraph::new(status), chunks[2]);
        })?;

        // Poll for keyboard events.
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.quit()
                        }
                        KeyCode::Char(':') => app.input_mode = InputMode::Command,
                        KeyCode::Char('/') => app.input_mode = InputMode::Filter,
                        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
                        _ => {}
                    },
                    InputMode::Command => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.command_input.clear();
                        }
                        KeyCode::Enter => {
                            let cmd = app.command_input.clone();
                            app.command_input.clear();
                            app.input_mode = InputMode::Normal;
                            if let Some(resource) = Resource::from_alias(cmd.trim()) {
                                app.switch_resource(resource);
                            }
                        }
                        KeyCode::Backspace => {
                            app.command_input.pop();
                        }
                        KeyCode::Char(c) => app.command_input.push(c),
                        _ => {}
                    },
                    InputMode::Filter => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.filter_input.clear();
                        }
                        KeyCode::Enter => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Backspace => {
                            app.filter_input.pop();
                        }
                        KeyCode::Char(c) => app.filter_input.push(c),
                        _ => {}
                    },
                }
            }
        }
    }

    // Restore terminal.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}