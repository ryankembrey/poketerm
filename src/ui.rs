use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(_app: &mut App, frame: &mut Frame) {
    let frame_size = frame.size();

    // Split the frame into two horizontal chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame_size);

    // Left block
    let left_block = Block::bordered()
        .title("Player 1")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    frame.render_widget(
        Paragraph::new(format!("Details here"))
            .block(left_block)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        chunks[0],
    );

    // Right block
    let right_block = Block::bordered()
        .title("Player 2")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    frame.render_widget(
        Paragraph::new("Details here")
            .block(right_block)
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .centered(),
        chunks[1],
    );
}
