use lobtui::app::{App, AppResult};
use lobtui::event::{Event, EventHandler};
use lobtui::handler::handle_key_events;
use lobtui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    color_eyre::install()?;

    let mut app = App::new().await?;

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1_000);

    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app, tui.events.sender.clone()).await?
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Notification(notification) => {
                app.notifications.push(notification);
            }
        }
    }

    tui.exit()?;
    Ok(())
}
