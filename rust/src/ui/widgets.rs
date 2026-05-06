use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render the r9s logo / splash used at startup.
pub fn render_logo(frame: &mut Frame, area: Rect) {
    let logo_lines = vec![
        Line::from(Span::styled(
            r"        ___          ",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            r"  _ __ / _ \ ___    ",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            r" | '__| (_) / __|   ",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            r" | |   \__, \__ \   ",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            r" |_|     /_/|___/   ",
            Style::default().fg(Color::Cyan),
        )),
    ];
    frame.render_widget(Paragraph::new(logo_lines), area);
}