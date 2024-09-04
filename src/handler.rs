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

        KeyCode::Char('G') => {
            *app.state.offset_mut() = (app.stories.len() - 1).saturating_sub(app.window_height);
            app.state.select(Some(app.stories.len() - 1));
        }

        KeyCode::Char('g') => {
            if app.previous_key == KeyCode::Char('g') {
                *app.state.offset_mut() = 0;
                app.state.select(Some(0));
            }
        }

        KeyCode::Char('j') | KeyCode::Down => {
            let i = match app.state.selected() {
                Some(i) => {
                    if i < app.window_height {
                        i + 1
                    } else if i == app.window_height - 1 {
                        *app.state.offset_mut() += 1;
                        i + 1
                    } else {
                        i
                    }
                }
                None => 0,
            };

            app.state.select(Some(i));
        }

        KeyCode::Char('k') | KeyCode::Up => {
            let i = match app.state.selected() {
                Some(i) => {
                    if i > app.state.offset() {
                        i - 1
                    } else if i == app.state.offset() && app.state.offset() > 0 {
                        *app.state.offset_mut() -= 1;
                        i - 1
                    } else {
                        0
                    }
                }
                None => 0,
            };

            app.state.select(Some(i));
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
            app.state.select(Some(0));
        }

        KeyCode::Char('p') => {
            if app.page > 1 {
                app.page = app.page.saturating_sub(1);

                if let Err(e) = app.refresh().await {
                    let notif = Notification::new(e.to_string(), NotificationLevel::Error);
                    sender.send(Event::Notification(notif)).unwrap();
                    app.page = app.page.saturating_add(1);
                }
                app.state.select(Some(0));
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

    app.previous_key = key_event.code;

    Ok(())
}
