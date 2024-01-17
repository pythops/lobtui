use crate::{
    app::{App, AppResult},
    event::Event,
    notifications::{Notification, NotificationLevel},
};
use tokio::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    sender: mpsc::UnboundedSender<Event>,
) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Char('j') | KeyCode::Down => {
            if app.cursor < app.stories.len() - 1 {
                app.cursor = app.cursor.saturating_add(1);
            }
        }

        KeyCode::Char('k') | KeyCode::Up => {
            app.cursor = app.cursor.saturating_sub(1);
        }

        KeyCode::Char('r') => {
            let notif = match app.refresh().await {
                Ok(_) => Notification::new("Page refreshed".to_string(), NotificationLevel::Info),
                Err(e) => Notification::new(e.to_string(), NotificationLevel::Error),
            };
            sender.send(Event::Notification(notif)).unwrap();
        }

        KeyCode::Char('n') => {
            app.page = app.page.saturating_add(1);
            if let Err(e) = app.refresh().await {
                let notif = Notification::new(e.to_string(), NotificationLevel::Error);
                sender.send(Event::Notification(notif)).unwrap();
                app.page = app.page.saturating_sub(1);
            }
        }

        KeyCode::Char('p') => {
            if app.page > 1 {
                app.page = app.page.saturating_sub(1);

                if let Err(e) = app.refresh().await {
                    let notif = Notification::new(e.to_string(), NotificationLevel::Error);
                    sender.send(Event::Notification(notif)).unwrap();
                    app.page = app.page.saturating_add(1);
                }
            }
        }

        KeyCode::Char('o') => {
            if let Err(e) = app.open() {
                let notif = Notification::new(e.to_string(), NotificationLevel::Error);
                sender.send(Event::Notification(notif)).unwrap();
            }
        }

        _ => {}
    }
    Ok(())
}
