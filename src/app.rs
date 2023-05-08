use crate::feeds_and_entry::entry::Entry;
use crate::session_and_user::{session::Session, user::User};
use crate::ui::screens::{FeedsOptions, HomeScreenOptions, Options, PostsOptions, SelectedScreen};
use crate::ui::{primitives::StatefulList, screens::ProceduresOptions};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use futures::executor::block_on;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::layout::{Alignment, Constraint, Direction};
use tui::style::Color;
use tui::widgets::{Paragraph, Wrap};
use tui::{
    backend::Backend,
    layout::{Layout, Rect},
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame, Terminal,
};

#[derive(Clone, Debug)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone, Debug)]
pub struct QuestionWithResponse {
    question: String,
    response: Option<String>,
}

impl QuestionWithResponse {
    pub fn new(question: String) -> QuestionWithResponse {
        QuestionWithResponse {
            question,
            response: None,
        }
    }
}

#[derive(Clone)]
pub struct App {
    scroll: u16,
    pub session: Option<Session>,
    pub selected_screen: SelectedScreen,
    pub previous_screen: SelectedScreen,
    pub items: StatefulList<String>,
    pub show_popup: bool,
    pub should_open_link: bool,
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
    current_form_questions: Option<Vec<QuestionWithResponse>>,
}

/// Pass the app no session to initiate at the create session screen.
impl App {
    pub fn new(session: Option<Session>) -> App {
        match session {
            Some(x) => {
                App {
                    scroll: 0,
                    session: Some(x),
                    selected_screen: SelectedScreen::Home,
                    previous_screen: SelectedScreen::Home,
                    items: SelectedScreen::Home.get_list_items(),
                    show_popup: false,
                    should_open_link: false,
                    input: String::from(""),
                    input_mode: InputMode::Normal,
                    messages: vec![],
                    current_form_questions: None,
                }
            }
            None => {
                return App {
                    scroll: 0,
                    session,
                    selected_screen: SelectedScreen::CreateSession,
                    previous_screen: SelectedScreen::CreateSession,
                    items: SelectedScreen::CreateSession.get_list_items(),
                    show_popup: false,
                    should_open_link: false,
                    input: String::from(""),
                    input_mode: InputMode::Normal,
                    messages: vec![],
                    current_form_questions: Some(
                        SelectedScreen::CreateSession
                            .get_list_items()
                            .items
                            .iter()
                            .map(|question| {
                                QuestionWithResponse::new(question.to_owned())
                            })
                            .collect(),
                    ),
                };
            }
        }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }

    fn save_current_buffer_to_selected_response(&mut self) {
        match &mut self.current_form_questions {
            None => {}
            Some(questions) => {
                questions[self.items.state.selected().unwrap()].response = Some(self.input.clone());
                self.input = String::from("");
            }
        }
    }

    fn proceed_with_question_responses(&mut self) -> Result<()> {
        match self.selected_screen {
            SelectedScreen::CreateSession => {
                self.session = Some(Session::new(
                    User::new(
                        self.current_form_questions.as_ref().unwrap()[0]
                            .response
                            .as_ref()
                            .unwrap()
                            .as_str(),
                    ),
                    vec![],
                    self.current_form_questions.as_ref().unwrap()[1]
                        .response
                        .as_ref()
                        .unwrap()
                        .as_str(),
                ));
            }
            _ => {}
        }
        self.current_form_questions = None;
        Ok(())
    }

    /// Move the user to a different screen.
    fn select_screen(&mut self, screen: SelectedScreen) {
        self.previous_screen = self.selected_screen.clone();
        self.selected_screen = screen;
        // Populate the app's current items with the corresponding screen's values.
        self.items.items = match self.selected_screen {
            SelectedScreen::Home => HomeScreenOptions::as_vec_of_strings(),
            SelectedScreen::Posts => PostsOptions::as_vec_of_strings(),
            SelectedScreen::Procedures => ProceduresOptions::as_vec_of_strings(),
            SelectedScreen::BrowsePosts => {
                self.session.as_ref().unwrap().get_all_blog_entry_titles()
            }
            // SelectedScreen::CreateSession => SelectedScreen::CreateSession.get_list_items().items,
            SelectedScreen::CreateSession => SelectedScreen::CreateSession.get_list_items().items,
            SelectedScreen::Feeds => FeedsOptions::as_vec_of_strings(),
            SelectedScreen::SelectSession => todo!(),
            SelectedScreen::Authors => self.session.as_ref().unwrap().get_unique_authors(),
        };
        // Move the cursor to the first available position.
        self.items.state.select(Some(0));
    }

    /// Move the user to the previously visited screen.
    fn go_to_previous_screen(&mut self) {
        // Todo: Make an undo tree rather than just having one previous screen.
        let target_screen = self.previous_screen.clone();
        self.previous_screen = self.selected_screen.clone();
        self.select_screen(target_screen);
    }
    /// To be used in the main loop, allows user navigation.
    /// Returns true if the user hasn't pressed q.
    fn navigate<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        timeout: std::time::Duration,
    ) -> bool {
        self.handle_screen_selection(terminal);
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                return self.handle_keyboard_input(key);
            } else {
                return true;
            };
        }
        true
    }

    /// The main loop
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

    /// Utility function for calling the appropriate ui screen type
    fn handle_screen_selection<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        match self.selected_screen {
            SelectedScreen::CreateSession => {
                terminal
                    .draw(|f| {
                        self.user_input_flow(f, self.selected_screen.get_screen_name().as_str())
                    })
                    .unwrap();
            }
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
    }

    fn handle_keyboard_input(&mut self, key: KeyEvent) -> bool {
        let mut resp = true;
        match self.input_mode {
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    match self.selected_screen {
                        SelectedScreen::CreateSession => {
                            self.save_current_buffer_to_selected_response();
                        }
                        _ => {}
                    }
                    self.messages.push(self.input.drain(..).collect());
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.save_current_buffer_to_selected_response();
                }
                _ => {}
            },
            InputMode::Normal => match key.code {
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
                KeyCode::Char('i') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Down => {
                    self.items.next();
                }
                KeyCode::Up => {
                    self.items.previous();
                }
                KeyCode::Enter => {
                    let label = self.items.items[self.items.state.selected().unwrap()].as_str();
                    match self.selected_screen {
                        SelectedScreen::Authors => {}
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
                                    self.select_screen(SelectedScreen::Authors)
                                }
                                PostsOptions::Categories => {
                                    todo!()
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
                            match self.proceed_with_question_responses() {
                                Ok(_) => self.select_screen(SelectedScreen::Home),
                                Err(_) => panic!("creation failed"), //self.select_screen(SelectedScreen::CreateSession),
                            }
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
                        SelectedScreen::BrowsePosts => self.should_open_link = true,
                    }
                }
                _ => {}
            },
        }
        resp
    }

    fn nav_list_generic<B: Backend>(&mut self, f: &mut Frame<B>, title: &str) {
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

    fn get_input_block(
        &self,
        title: String,
        question: &QuestionWithResponse,
        question_index: usize,
    ) -> Paragraph {
        let mut paragraph_text = format!(
            "{}{}",
            question.question,
            question.response.as_ref().unwrap_or(&String::from(""))
        );
        if self.items.state.selected().unwrap() == question_index {
            paragraph_text = format!("{}{}", paragraph_text, self.input);
            Paragraph::new(paragraph_text)
                .style(match self.input_mode {
                    InputMode::Normal => Style::default().fg(Color::Magenta),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title(title))
        } else {
            Paragraph::new(paragraph_text)
                .style(match self.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default(),
                })
                .block(Block::default().borders(Borders::ALL).title(title))
        }
    }

    fn user_input_flow<B: Backend>(&mut self, f: &mut Frame<B>, title: &str) {
        let size = f.size();

        let block = Block::default().title(title);

        f.render_widget(block, size);

        let questions_display_percentage =
            if self.current_form_questions.as_ref().unwrap().len() * 10 < 50 {
                self.current_form_questions.as_ref().unwrap().len() * 10
            } else {
                50
            };

        let items: Vec<Paragraph> = self
            .current_form_questions
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, question)| self.get_input_block(format!("{:?}", i + 1), question, i))
            .collect();

        let per_constraint = ((questions_display_percentage as f64 / 100.0)
            / self.current_form_questions.as_ref().unwrap().len() as f64)
            * 100.0;

        assert!(
            0.0 < per_constraint && per_constraint < 100.0,
            "constraint percentage: {}",
            per_constraint
        );

        let mut question_chunk_constraints: Vec<Constraint> = self
            .current_form_questions
            .as_ref()
            .unwrap()
            .iter()
            .map(|_| Constraint::Percentage(per_constraint.round() as u16))
            .collect();

        question_chunk_constraints.append(&mut vec![Constraint::Percentage(
            100 - questions_display_percentage as u16,
        )]);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(question_chunk_constraints)
            .split(f.size());

        for (i, item) in items.iter().enumerate() {
            f.render_widget(item.to_owned(), chunks[i])
        }

        f.render_widget(
            Paragraph::new(format!(
                "{:?}",
                self.current_form_questions.as_ref().unwrap()
            )),
            chunks[chunks.len() - 1],
        )
    }

    /// Initially made for the BrowsePosts page but could be repurposed for any page with
    /// selectable items.
    fn nav_list_for_blog_entries<B: Backend>(
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
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
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
        if self.should_open_link {
            let url = displayed_item.url;
            open::that(url).unwrap();
            self.should_open_link = false;
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
