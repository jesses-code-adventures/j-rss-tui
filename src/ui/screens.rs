use tui::{backend::Backend, Terminal};

use crate::{
    app::app::App, session_and_user::session_and_user::Session, ui::primitives::StatefulList,
};

use crossterm::event::{self, Event, KeyCode};
pub enum SelectedScreen {
    Home,
    Posts,
    BrowsePosts,
    EditFeeds,
    CreateSession,
    SelectSession,
    Procedures,
}

impl SelectedScreen {
    pub fn get_list_items(&self) -> StatefulList<String> {
        match self {
            SelectedScreen::Home => {
                return StatefulList::with_items(HomeScreenOptions::as_vec_of_strings())
            }
            SelectedScreen::Posts => {
                return StatefulList::with_items(PostsOptions::as_vec_of_strings())
            }
            SelectedScreen::EditFeeds => todo!(),
            SelectedScreen::CreateSession => todo!(),
            SelectedScreen::SelectSession => todo!(),
            SelectedScreen::Procedures => todo!(),
            SelectedScreen::BrowsePosts => StatefulList::with_items(vec![]),
        }
    }

    pub fn get_screen_name(&self) -> String {
        match self {
            SelectedScreen::Home => return String::from("Home"),
            SelectedScreen::Posts => return String::from("Posts"),
            SelectedScreen::EditFeeds => return String::from("Edit Feeds"),
            SelectedScreen::CreateSession => return String::from("Create Session"),
            SelectedScreen::SelectSession => return String::from("Select Session"),
            SelectedScreen::Procedures => return String::from("Procedures"),
            SelectedScreen::BrowsePosts => return String::from("Browse Posts"),
        }
    }
}

pub enum ProceduresOptions {
    UpdatePosts,
    AddSource,
    DumpSessionData,
    Home,
}

impl ProceduresOptions {
    pub fn as_string(&self) -> String {
        match self {
            ProceduresOptions::UpdatePosts => String::from("Update Posts"),
            ProceduresOptions::AddSource => String::from("Add Source"),
            ProceduresOptions::DumpSessionData => String::from("Save"),
            ProceduresOptions::Home => String::from("Home"),
        }
    }

    pub fn from_string(text: &str) -> ProceduresOptions {
        match text {
            "Update Posts" => ProceduresOptions::UpdatePosts,
            "Add Source" => ProceduresOptions::AddSource,
            "Save" => ProceduresOptions::DumpSessionData,
            "Home" => ProceduresOptions::Home,
            _ => panic!("This isn't an option!"),
        }
    }
    pub fn as_vec_of_strings() -> Vec<String> {
        vec![
            ProceduresOptions::UpdatePosts.as_string(),
            ProceduresOptions::AddSource.as_string(),
            ProceduresOptions::DumpSessionData.as_string(),
            ProceduresOptions::Home.as_string(),
        ]
    }
}

pub enum PostsOptions {
    Browse,
    Search,
    Categories,
    Authors,
    Home,
}

impl PostsOptions {
    pub fn as_string(&self) -> String {
        match self {
            PostsOptions::Browse => String::from("Browse Posts"),
            PostsOptions::Search => String::from("Search Posts"),
            PostsOptions::Categories => String::from("Browse Categories"),
            PostsOptions::Authors => String::from("Change Session"),
            PostsOptions::Home => String::from("Home"),
        }
    }

    pub fn from_string(text: &str) -> PostsOptions {
        match text {
            "Browse Posts" => PostsOptions::Browse,
            "Search Posts" => PostsOptions::Search,
            "Browse Categories" => PostsOptions::Categories,
            "Change Session" => PostsOptions::Authors,
            "Home" => PostsOptions::Home,
            _ => panic!("This isn't an option!"),
        }
    }
    pub fn as_vec_of_strings() -> Vec<String> {
        vec![
            PostsOptions::Home.as_string(),
            PostsOptions::Search.as_string(),
            PostsOptions::Browse.as_string(),
            PostsOptions::Authors.as_string(),
            PostsOptions::Categories.as_string(),
        ]
    }
}

pub enum HomeScreenOptions {
    ViewPosts,
    AddRemoveSources,
    CreateSession,
    ChangeSession,
    Procedures,
}

impl HomeScreenOptions {
    pub fn as_string(&self) -> String {
        match self {
            HomeScreenOptions::ViewPosts => String::from("View Posts"),
            HomeScreenOptions::CreateSession => String::from("Create New Session"),
            HomeScreenOptions::ChangeSession => String::from("Change Session"),
            HomeScreenOptions::AddRemoveSources => String::from("Add or Remove Sources"),
            HomeScreenOptions::Procedures => String::from("Procedures"),
        }
    }

    pub fn from_string(text: &str) -> HomeScreenOptions {
        match text {
            "View Posts" => HomeScreenOptions::ViewPosts,
            "Create New Session" => HomeScreenOptions::CreateSession,
            "Change Session" => HomeScreenOptions::ChangeSession,
            "Add or Remove Sources" => HomeScreenOptions::AddRemoveSources,
            "Procedures" => HomeScreenOptions::Procedures,
            _ => panic!("This isn't an option!"),
        }
    }

    pub fn as_vec_of_strings() -> Vec<String> {
        vec![
            HomeScreenOptions::ViewPosts.as_string(),
            HomeScreenOptions::CreateSession.as_string(),
            HomeScreenOptions::ChangeSession.as_string(),
            HomeScreenOptions::AddRemoveSources.as_string(),
            HomeScreenOptions::Procedures.as_string(),
        ]
    }
}
