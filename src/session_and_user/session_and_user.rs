use crate::feeds_and_entry::feeds_and_entry::{BlogFeed, Entry};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> User {
        return User {
            name: String::from(name),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub user: User,
    pub name: String,
    pub blog_feeds: Vec<BlogFeed>,
}

impl Session {
    pub fn new(user: User, blog_feeds: Vec<BlogFeed>, name: &str) -> Session {
        return Session {
            name: String::from(name),
            user,
            blog_feeds,
        };
    }

    pub fn create_blog_feed(&mut self, name: &str, url: &str) {
        self.blog_feeds.push(BlogFeed::new(name, url));
    }

    fn to_json(&self) -> serde_json::Value {
        let the_json = json!(&self);
        the_json
    }

    pub fn dump_to_json(&self) {
        let the_json = self.to_json();

        std::fs::write(
            ".session.json",
            serde_json::to_string_pretty(&the_json).unwrap(),
        )
        .unwrap();
    }

    pub fn load_from_json() -> Session {
        let text = std::fs::read_to_string(".session.json").unwrap();
        serde_json::from_str::<Session>(&text).unwrap()
    }

    pub fn from_json(the_json: serde_json::Value) -> Session {
        let session: Session =
            serde_json::from_str(the_json.to_string().as_str()).expect("json was fucked");
        return session;
    }

    pub fn test_json_translation(&self) {
        let the_json = &self.to_json();
        let transformed_back = Session::from_json(the_json.clone());
        assert!(self == &transformed_back);
        println!("Great success")
    }

    // pub fn get_all_blog_entries(&self) -> Vec<Entry> {
    //     let mut all_entries = vec![];
    //     for feed in &self.blog_feeds {
    //         all_entries.append(&mut feed.entries.as_mut().unwrap());
    //     }
    //     return all_entries;
    // }

    pub fn get_all_blog_entry_titles(&self) -> Vec<String> {
        let mut all_titles: Vec<String> = vec![];
        for feed in &self.blog_feeds {
            for entry in &feed.entries {
                for e in entry.iter() {
                    all_titles.push(e.to_string());
                }
            }

            // all_entries.append(&mut feed.entries.unwrap());
        }
        return all_titles;
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}'s session {:?}\n\n{:?}",
            self.user.name, self.name, self.blog_feeds
        )
    }
}
