use std::usize;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, notifications::NotificationLevel};

pub fn notification_rect(offset: u16, notification_height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1 + notification_height * offset),
                Constraint::Length(notification_height),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(30),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let (body_block, footer_block) = {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(frame.size());
        (chunks[0], chunks[1])
    };

    // footer
    let footer = Paragraph::new(Line::from(format!("Page {}", app.page)))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    // Body
    let items: Vec<ListItem> = app
        .stories
        .iter()
        .enumerate()
        .map(|(i, story)| {
            let first_line = vec![
                Span::styled(" ▲ ", Style::default().fg(Color::Gray)),
                Span::styled(
                    story.title.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ];

            let second_line = vec![
                Span::styled(
                    format!("   {}", story.votes),
                    Style::default().fg(Color::Gray),
                ),
                Span::raw(" votes. By "),
                Span::styled(
                    story.author.clone(),
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw(". "),
                Span::styled(
                    story.comment_count.clone(),
                    Style::default().fg(Color::Gray),
                ),
                Span::raw(" comments. Tags: "),
                Span::styled(
                    story.tags.join(", "),
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ];

            let item = ListItem::new(vec![
                Line::from(first_line),
                Line::from(second_line),
                Line::from(""),
            ]);

            if app.cursor == i {
                return item.style(Style::default().bg(Color::DarkGray));
            }

            item
        })
        .collect();

    let mut scroll = 0;

    let body_block_height = body_block.height as usize - 3;
    let items_length = items.len() * 3;

    if body_block_height < items_length && app.cursor > body_block_height / 3 {
        scroll = app.cursor - body_block_height / 3;
    }

    let list = List::new(items.as_slice()[scroll..].to_vec())
        .block(Block::default())
        .style(Style::default().fg(Color::White));

    frame.render_widget(list, body_block);
    frame.render_widget(footer, footer_block);

    // Notifs
    for (i, n) in app.notifications.iter().enumerate() {
        let (color, title) = match n.level {
            NotificationLevel::Info => (Color::Green, "Info"),
            NotificationLevel::Warning => (Color::Yellow, "Warning"),
            NotificationLevel::Error => (Color::Red, "Error"),
        };

        let text = Text::from(vec![
            Line::styled(
                title,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
            Line::raw(n.message.as_str()),
        ]);

        let block = Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(color)),
            );

        let notification_height = {
            let mut height: u16 = 4;
            for line in n.message.lines() {
                height += 1;
                height += line.len() as u16 / 20;
            }
            height
        };

        let area = notification_rect(i as u16, notification_height, frame.size());

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
    }
}
