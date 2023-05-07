use crate::ui::primitives::StatefulList;

#[derive(Clone)]
pub enum SelectedScreen {
    Home,
    Posts,
    BrowsePosts,
    Feeds,
    CreateSession,
    SelectSession,
    Procedures,
}

impl SelectedScreen {
    pub fn get_list_items(&self) -> StatefulList<String> {
        match self {
            SelectedScreen::Home => {
                StatefulList::with_items(HomeScreenOptions::as_vec_of_strings())
            }
            SelectedScreen::Posts => StatefulList::with_items(PostsOptions::as_vec_of_strings()),
            SelectedScreen::Feeds => StatefulList::with_items(FeedsOptions::as_vec_of_strings()),
            SelectedScreen::CreateSession => todo!(),
            SelectedScreen::SelectSession => todo!(),
            SelectedScreen::Procedures => todo!(),
            SelectedScreen::BrowsePosts => StatefulList::with_items(vec![]),
        }
    }

    pub fn get_screen_name(&self) -> String {
        match self {
            SelectedScreen::Home => String::from("Home"),
            SelectedScreen::Posts => String::from("Posts"),
            SelectedScreen::Feeds => String::from("Edit Feeds"),
            SelectedScreen::CreateSession => String::from("Create Session"),
            SelectedScreen::SelectSession => String::from("Select Session"),
            SelectedScreen::Procedures => String::from("Procedures"),
            SelectedScreen::BrowsePosts => String::from("Browse Posts"),
        }
    }
}

pub enum ProceduresOptions {
    UpdatePosts,
    AddSource,
    DumpSessionData,
    Home,
}

impl Options<ProceduresOptions> for ProceduresOptions {
    fn as_string(&self) -> String {
        match self {
            ProceduresOptions::UpdatePosts => String::from("Update Posts"),
            ProceduresOptions::AddSource => String::from("Add Source"),
            ProceduresOptions::DumpSessionData => String::from("Save"),
            ProceduresOptions::Home => String::from("Home"),
        }
    }

    fn from_string(text: &str) -> ProceduresOptions {
        match text {
            "Update Posts" => ProceduresOptions::UpdatePosts,
            "Add Source" => ProceduresOptions::AddSource,
            "Save" => ProceduresOptions::DumpSessionData,
            "Home" => ProceduresOptions::Home,
            _ => panic!("This isn't an option!"),
        }
    }
    fn as_vec_of_strings() -> Vec<String> {
        vec![
            ProceduresOptions::UpdatePosts.as_string(),
            ProceduresOptions::AddSource.as_string(),
            ProceduresOptions::DumpSessionData.as_string(),
            ProceduresOptions::Home.as_string(),
        ]
    }
}

pub enum FeedsOptions {
    ViewFeeds,
    AddFeed,
    Home,
}

pub trait Options<T> {
    fn as_string(&self) -> String;
    fn from_string(text: &str) -> T;
    fn as_vec_of_strings() -> Vec<String>;
}

impl Options<FeedsOptions> for FeedsOptions {
    fn as_string(&self) -> String {
        match self {
            FeedsOptions::Home => String::from("Home"),
            FeedsOptions::AddFeed => String::from("Add Feed"),
            FeedsOptions::ViewFeeds => String::from("View Feeds"),
        }
    }

    fn from_string(text: &str) -> FeedsOptions {
        match text {
            "Home" => FeedsOptions::Home,
            "Add Feed" => FeedsOptions::AddFeed,
            "View Feeds" => FeedsOptions::ViewFeeds,
            _ => panic!("This isn't an option!"),
        }
    }
    fn as_vec_of_strings() -> Vec<String> {
        vec![
            FeedsOptions::Home.as_string(),
            FeedsOptions::AddFeed.as_string(),
            FeedsOptions::ViewFeeds.as_string(),
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

impl Options<PostsOptions> for PostsOptions {
    fn as_string(&self) -> String {
        match self {
            PostsOptions::Browse => String::from("Browse Posts"),
            PostsOptions::Search => String::from("Search Posts"),
            PostsOptions::Categories => String::from("Browse Categories"),
            PostsOptions::Authors => String::from("Change Session"),
            PostsOptions::Home => String::from("Home"),
        }
    }

    fn from_string(text: &str) -> PostsOptions {
        match text {
            "Browse Posts" => PostsOptions::Browse,
            "Search Posts" => PostsOptions::Search,
            "Browse Categories" => PostsOptions::Categories,
            "Change Session" => PostsOptions::Authors,
            "Home" => PostsOptions::Home,
            _ => panic!("This isn't an option!"),
        }
    }
    fn as_vec_of_strings() -> Vec<String> {
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

impl Options<HomeScreenOptions> for HomeScreenOptions {
    fn as_string(&self) -> String {
        match self {
            HomeScreenOptions::ViewPosts => String::from("View Posts"),
            HomeScreenOptions::CreateSession => String::from("Create New Session"),
            HomeScreenOptions::ChangeSession => String::from("Change Session"),
            HomeScreenOptions::AddRemoveSources => String::from("Add or Remove Sources"),
            HomeScreenOptions::Procedures => String::from("Procedures"),
        }
    }

    fn from_string(text: &str) -> HomeScreenOptions {
        match text {
            "View Posts" => HomeScreenOptions::ViewPosts,
            "Create New Session" => HomeScreenOptions::CreateSession,
            "Change Session" => HomeScreenOptions::ChangeSession,
            "Add or Remove Sources" => HomeScreenOptions::AddRemoveSources,
            "Procedures" => HomeScreenOptions::Procedures,
            _ => panic!("This isn't an option!"),
        }
    }

    fn as_vec_of_strings() -> Vec<String> {
        vec![
            HomeScreenOptions::ViewPosts.as_string(),
            HomeScreenOptions::CreateSession.as_string(),
            HomeScreenOptions::ChangeSession.as_string(),
            HomeScreenOptions::AddRemoveSources.as_string(),
            HomeScreenOptions::Procedures.as_string(),
        ]
    }
}
