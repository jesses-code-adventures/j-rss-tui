use crate::feeds_and_entry::feeds_and_entry::Entry;
use crate::session_and_user::session_and_user::Session;
use crate::ui::screens::{FeedsOptions, HomeScreenOptions, Options, PostsOptions, SelectedScreen};
use crate::ui::{primitives::StatefulList, screens::ProceduresOptions};
use crossterm::event::{self, Event, KeyCode};
use futures::executor::block_on;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::layout::{Alignment, Constraint, Direction};
use tui::widgets::{Paragraph, Wrap};
use tui::{
    backend::Backend,
    layout::{Layout, Rect},
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame, Terminal,
};

#[derive(Clone)]
pub struct App {
    scroll: u16,
    pub session: Option<Session>,
    // pub user: Option<User>,
    pub selected_screen: SelectedScreen,
    pub previous_screen: SelectedScreen,
    pub items: StatefulList<String>,
    pub show_popup: bool,
}

impl App {
    pub fn new(session: Option<Session>) -> App {
        App {
            scroll: 0,
            session,
            // user,
            selected_screen: SelectedScreen::Home,
            previous_screen: SelectedScreen::Home,
            items: SelectedScreen::Home.get_list_items(),
            show_popup: false,
        }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }

    pub fn select_screen(&mut self, screen: SelectedScreen) {
        self.previous_screen = self.selected_screen.clone();
        self.selected_screen = screen;
        self.items.items = match self.selected_screen {
            SelectedScreen::Home => HomeScreenOptions::as_vec_of_strings(),
            SelectedScreen::Posts => PostsOptions::as_vec_of_strings(),
            SelectedScreen::Procedures => ProceduresOptions::as_vec_of_strings(),
            SelectedScreen::BrowsePosts => {
                self.session.as_ref().unwrap().get_all_blog_entry_titles()
            }
            SelectedScreen::CreateSession => todo!(),
            SelectedScreen::Feeds => FeedsOptions::as_vec_of_strings(),
            SelectedScreen::SelectSession => todo!(),
        };
        self.items.state.select(Some(0));
    }

    pub fn go_to_previous_screen(&mut self) {
        let target_screen = self.previous_screen.clone();
        self.previous_screen = self.selected_screen.clone();
        self.select_screen(target_screen);
    }

    pub fn navigate<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        timeout: std::time::Duration,
    ) -> bool {
        match self.selected_screen {
            SelectedScreen::BrowsePosts => {
                terminal
                    .draw(|f| {
                        self.nav_list_for_blog_entries(
                            f,
                            self.selected_screen.get_screen_name().as_str(),
                            self.session.clone().unwrap().get_all_blog_entries(),
                        )
                    })
                    .unwrap();
            }
            _ => {
                terminal
                    .draw(|f| {
                        self.nav_list_generic(f, self.selected_screen.get_screen_name().as_str())
                    })
                    .unwrap();
            }
        };

        let mut resp = true;
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Left => {
                        self.go_to_previous_screen();
                    }
                    KeyCode::Char('q') => resp = false,
                    KeyCode::Char('j') => {
                        self.items.next();
                    }
                    KeyCode::Char('k') => {
                        self.items.previous();
                    }
                    KeyCode::Char('b') => {
                        self.go_to_previous_screen();
                    }
                    KeyCode::Char('p') => self.show_popup = !self.show_popup,
                    KeyCode::Down => {
                        self.items.next();
                    }
                    KeyCode::Up => {
                        self.items.previous();
                    }
                    KeyCode::Enter => {
                        let label = self.items.items[self.items.state.selected().unwrap()].as_str();
                        match self.selected_screen {
                            SelectedScreen::Home => {
                                match HomeScreenOptions::from_string(label) {
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
                                        self.select_screen(SelectedScreen::Feeds)
                                    }
                                    HomeScreenOptions::Procedures => {
                                        self.select_screen(SelectedScreen::Procedures)
                                    }
                                };
                            }
                            SelectedScreen::Posts => {
                                match PostsOptions::from_string(label) {
                                    PostsOptions::Home => self.select_screen(SelectedScreen::Home),
                                    PostsOptions::Browse => {
                                        self.select_screen(SelectedScreen::BrowsePosts)
                                    }
                                    PostsOptions::Search => {
                                        todo!("will be a self.run_procedure or something instead of select_screen")
                                    }
                                    PostsOptions::Authors => {
                                        todo!("screen with content");
                                    }
                                    PostsOptions::Categories => {
                                        todo!("screen with content");
                                    }
                                };
                            }
                            SelectedScreen::Feeds => match FeedsOptions::from_string(label) {
                                FeedsOptions::ViewFeeds => {
                                    todo!("screen with editable content");
                                }
                                FeedsOptions::AddFeed => {
                                    todo!("procedure");
                                }
                                FeedsOptions::Home => self.select_screen(SelectedScreen::Home),
                            },
                            SelectedScreen::CreateSession => {
                                todo!("procedure")
                            }
                            SelectedScreen::SelectSession => {
                                todo!("procedure")
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
                                    ProceduresOptions::UpdatePosts => block_on(
                                        self.session.to_owned().unwrap().fetch_all_blog_entries(),
                                    ),
                                };
                            }
                            SelectedScreen::BrowsePosts => {
                                todo!("view the content of a selected post")
                            }
                        }
                    }
                    _ => return resp,
                }
            }
        }
        resp
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

            let another = self.navigate(terminal, timeout);

            if !another {
                return Ok(());
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    pub fn nav_list_generic<B: Backend>(&mut self, f: &mut Frame<B>, title: &str) {
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

    pub fn nav_list_for_blog_entries<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        title: &str,
        preview_items: Vec<Entry>,
    ) {
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

        let displayed_item: Entry = preview_items
            .get(self.items.state.selected().unwrap())
            .unwrap()
            .clone();

        let paragraph = Paragraph::new(displayed_item.get_feed_content())
            .block(Block::default().title("Content").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(f.size());

        f.render_stateful_widget(items, chunks[0], &mut self.items.state);
        f.render_widget(paragraph, chunks[1]);
        if self.show_popup {
            let popup_block = Paragraph::new(displayed_item.get_feed_content())
                .block(Block::default().title("Popup").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            let popup_area = self.centered_rect(60, 80, size);
            f.render_widget(Clear, popup_area); //this clears out the background
            f.render_widget(popup_block, popup_area);
        }
    }
    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
