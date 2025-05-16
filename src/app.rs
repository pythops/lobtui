use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{style::Stylize, text::Text, widgets::ListState};
use reqwest::Client;
use std::error;

use scraper::{Html, Selector, error::SelectorErrorKind};

use crate::notifications::Notification;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
};

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub client: Client,
    pub previous_key: KeyCode,
    pub stories: Vec<Story>,
    pub page: usize,
    pub notifications: Vec<Notification>,
    pub state: ListState,
    pub window_height: usize,
}

#[derive(Debug)]
pub struct Story {
    pub title: String,
    pub link: String,
    pub author: String,
    pub tags: Vec<String>,
    pub votes: usize,
    pub comment_count: String,
    pub comment_link: String,
}

impl App {
    pub async fn new() -> Result<Self> {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let response = client
            .get("https://lobste.rs/")
            .send()
            .await?
            .text()
            .await?;

        let stories = App::parse(response).unwrap();

        let mut state = ListState::default();

        if !stories.is_empty() {
            state.select(Some(0));
        }

        Ok(Self {
            running: true,
            client,
            previous_key: KeyCode::Null,
            stories,
            page: 1,
            notifications: Vec::new(),
            state,
            window_height: 0,
        })
    }

    pub fn parse(page: String) -> Result<Vec<Story>, SelectorErrorKind<'static>> {
        let mut stories: Vec<Story> = Vec::new();

        let html = Html::parse_fragment(&page);
        let story_selector = Selector::parse(r#"div[class="story_liner h-entry"]"#)?;
        let title_selector = Selector::parse(r#"a[class="u-url"]"#)?;
        let tags_selector = Selector::parse(r#"span[class="tags"]"#)?;
        let author_selector = Selector::parse("a.u-author")?;
        let votes_selector = Selector::parse("a.upvoter")?;
        let comments_selector = Selector::parse(r#"span[class="comments_label"]"#)?;

        for story in html.select(&story_selector) {
            let story_html = Html::parse_fragment(&story.html());

            let title_span = story_html.select(&title_selector).next().unwrap();

            let title: String = title_span.text().collect();

            let link: Option<&str> = title_span.value().attr("href");

            let tags: Vec<String> = story_html
                .select(&tags_selector)
                .next()
                .unwrap()
                .text()
                .filter(|t| t.trim() != "")
                .map(|t| t.to_string())
                .collect();

            let author: String = story_html
                .select(&author_selector)
                .next()
                .unwrap()
                .text()
                .collect();

            let votes: String = story_html
                .select(&votes_selector)
                .next()
                .unwrap()
                .text()
                .collect();

            let comment_span =
                Html::parse_fragment(&story_html.select(&comments_selector).next().unwrap().html());

            let comment_span = comment_span
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap();

            let comment_count = comment_span.text().collect::<String>();
            let comment_count = comment_count
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0);

            let comment_link = comment_span.value().attr("href");

            stories.push(Story {
                title,
                link: link.unwrap_or_default().to_string(),
                author,
                tags: tags.to_owned(),
                votes: votes.parse::<usize>().unwrap_or_default(),
                comment_count: comment_count.to_string(),
                comment_link: comment_link.unwrap_or_default().to_string(),
            });
        }

        Ok(stories)
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let (body_block, footer_block) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(frame.area());
            (chunks[0], chunks[1])
        };

        // footer
        let footer = Paragraph::new(Text::from(format!("Page {}", self.page)).bold())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        // Body

        // 2 because two lines for an item list
        self.window_height = body_block.height as usize / 2;

        let items: Vec<ListItem> = self
            .stories
            .iter()
            .enumerate()
            .map(|(index, story)| {
                let first_line = vec![
                    Span::styled(
                        format!("{}. ▲ ", index + 1),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::styled(
                        story.title.clone(),
                        Style::default().add_modifier(Modifier::BOLD).blue(),
                    ),
                ];

                let second_line = vec![
                    Span::styled(
                        format!("     {}", story.votes),
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
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ];

                let item = ListItem::new(vec![
                    Line::from(first_line),
                    Line::from(second_line).gray(),
                    Line::from(""),
                ]);

                item
            })
            .collect();

        let list = List::new(items.to_vec())
            .highlight_style(Style::new().bg(Color::DarkGray))
            .block(Block::default())
            .style(Style::default());

        frame.render_stateful_widget(list, body_block, &mut self.state);
        frame.render_widget(footer, footer_block);
    }

    pub async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("https://lobste.rs/page/{}", self.page))
            .send()
            .await?
            .text()
            .await?;

        match App::parse(response) {
            Ok(stories) => self.stories = stories,
            Err(_) => return Err("Can not parse the response".into()),
        }

        Ok(())
    }

    pub fn open(&mut self) -> Result<()> {
        let story = &self.stories[self.state.selected().unwrap()];
        open::that(&story.link)?;
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn tick(&mut self) {
        self.notifications.retain(|n| n.ttl > 0);
        self.notifications.iter_mut().for_each(|n| n.ttl -= 1);
    }
}
