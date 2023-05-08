use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    pub title: String,
    pub authors: String,
    pub blurb: String,
    pub url: String,
    pub content: Option<String>,
    // updated_at: Option<DateTime<Utc>>,
    pub updated_at: Option<String>,
}

#[allow(unused)]
impl Entry {
    pub fn new(
        title: &str,
        authors: &str,
        blurb: &str,
        url: &str,
        content: Option<String>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Entry {
        let updated_at_str: String;
        match updated_at {
            Some(x) => updated_at_str = x.to_string(),
            None => updated_at_str = String::from(""),
        }
        Entry {
            title: String::from(title),
            authors: String::from(authors),
            blurb: String::from(blurb),
            url: String::from(url),
            content,
            updated_at: Some(updated_at_str),
        }
    }
    pub fn get_feed_content(&self) -> String {
        match &self.content {
            Some(x) => return x.to_string(),
            None => {
                if self.blurb.len() > 0 {
                    return self.blurb.to_string();
                } else {
                    return self.title.to_string();
                }
            }
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} - {:?}",
            &self.updated_at.as_ref().unwrap().trim_matches('"'),
            &self.title.trim_matches('"'),
        )
    }
}
