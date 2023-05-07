use crate::session_and_user::session_and_user::Session;
use crate::ui::screens::{HomeScreenOptions, PostsOptions, SelectedScreen};
use crate::ui::{primitives::StatefulList, screens::ProceduresOptions};
use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

pub struct App {
    scroll: u16,
    pub session: Option<Session>,
    pub selected_screen: SelectedScreen,
    pub items: StatefulList<String>,
}

impl App {
    pub fn new(session: Option<Session>) -> App {
        return App {
            scroll: 0,
            session,
            selected_screen: SelectedScreen::Home,
            items: SelectedScreen::Home.get_list_items(),
        };
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }

    pub fn select_screen(&mut self, screen: SelectedScreen) {
        self.selected_screen = screen;
        self.items.items = match self.selected_screen {
            SelectedScreen::Home => HomeScreenOptions::as_vec_of_strings(),
            SelectedScreen::Posts => PostsOptions::as_vec_of_strings(),
            SelectedScreen::Procedures => ProceduresOptions::as_vec_of_strings(),
            SelectedScreen::BrowsePosts => {
                self.session.as_ref().unwrap().get_all_blog_entry_titles()[0..20].to_vec()
            }
            SelectedScreen::CreateSession => todo!(),
            SelectedScreen::EditFeeds => todo!(),
            SelectedScreen::SelectSession => todo!(),
        }
    }

    pub fn navigate<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        timeout: std::time::Duration,
    ) {
        terminal
            .draw(|f| self.list_of_options(f, self.selected_screen.get_screen_name().as_str()))
            .unwrap();
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Left => self.items.unselect(),
                    KeyCode::Char('q') => std::process::exit(0),
                    KeyCode::Down => {
                        self.items.next();
                    }
                    KeyCode::Up => self.items.previous(),
                    KeyCode::Enter => {
                        let label = self.items.items[self.items.state.selected().unwrap()].as_str();
                        match self.selected_screen {
                            SelectedScreen::Home => match HomeScreenOptions::from_string(label) {
                                HomeScreenOptions::ViewPosts => {
                                    self.select_screen(SelectedScreen::Posts);
                                }
                                HomeScreenOptions::CreateSession => {
                                    self.select_screen(SelectedScreen::CreateSession)
                                }
                                HomeScreenOptions::ChangeSession => {
                                    self.select_screen(SelectedScreen::SelectSession)
                                }
                                HomeScreenOptions::AddRemoveSources => {
                                    self.select_screen(SelectedScreen::EditFeeds)
                                }
                                HomeScreenOptions::Procedures => {
                                    self.select_screen(SelectedScreen::Procedures)
                                }
                            },
                            SelectedScreen::Posts => match PostsOptions::from_string(label) {
                                PostsOptions::Home => self.select_screen(SelectedScreen::Home),
                                PostsOptions::Browse => {
                                    self.select_screen(SelectedScreen::BrowsePosts)
                                }
                                PostsOptions::Search => {
                                    todo!()
                                }
                                PostsOptions::Authors => {
                                    todo!();
                                }
                                PostsOptions::Categories => {
                                    todo!();
                                }
                            },
                            SelectedScreen::EditFeeds => {
                                todo!()
                            }
                            SelectedScreen::CreateSession => {
                                todo!()
                            }
                            SelectedScreen::SelectSession => {
                                todo!()
                            }
                            SelectedScreen::Procedures => {
                                match ProceduresOptions::from_string(label) {
                                    ProceduresOptions::Home => {
                                        self.select_screen(SelectedScreen::Home);
                                    }
                                    ProceduresOptions::AddSource => todo!(),
                                    ProceduresOptions::DumpSessionData => {
                                        self.session.as_ref().unwrap().dump_to_json()
                                    }
                                    ProceduresOptions::UpdatePosts => todo!(),
                                }
                            }
                            SelectedScreen::BrowsePosts => {
                                todo!()
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            match self.items.state.selected() {
                Some(_) => {}
                None => self.items.state.select(Some(0)),
            };

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            self.navigate(terminal, timeout);

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    pub fn list_of_options<B: Backend>(&mut self, f: &mut Frame<B>, title: &str) {
        let size = f.size();

        let block = Block::default();
        f.render_widget(block, size);

        let items: Vec<ListItem> = self
            .items
            .items
            .iter()
            .map(|i| {
                let mut lines = vec![];
                lines.push(Spans::from(i.as_str()));
                ListItem::new(lines).style(Style::default())
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("🍄 ");

        f.render_stateful_widget(items, size, &mut self.items.state)
    }
}
