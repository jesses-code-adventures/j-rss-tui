use chrono::{DateTime, Utc};
use feed_rs::{self, model::{Person, Content}};
use std::fmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::feeds_and_entry::entry::Entry;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogFeed {
    pub name: String,
    pub url: String,
    pub entries: Option<Vec<Entry>>,
}

#[allow(unused)]
impl BlogFeed {
    pub fn new(url: &str, name: &str) -> BlogFeed {
        BlogFeed {
            name: String::from(name),
            url: String::from(url),
            entries: None,
        }
    }

    pub async fn get_rss_feed(&self) -> Result<feed_rs::model::Feed> {
        let content = reqwest::get(&self.url).await?.bytes().await?;
        let content_str = String::from_utf8_lossy(&content);
        let feed = feed_rs::parser::parse(content_str.as_bytes());
        Ok(feed.unwrap())
    }

    pub fn populate_entries(&mut self, feed: &feed_rs::model::Feed)  -> Option<Vec<Entry>> {
        let mut entries: Vec<Entry> = vec![];
        for entry in feed.entries.iter() {
            // Deleted markdown conversion as i dont think it was doing anything
            // Left the code for now as it could be retested
            // I just wanted to remove the dependency ok

            // let markdown_title = &html2md::parse_html(entry.title.as_ref().unwrap().content.as_str());
            let markdown_title = &entry.title.as_ref().unwrap().content;

            let unprocessed_authors: Vec<Person> = entry.authors.to_owned();
            let mut html_authors: Vec<String> = vec![];
            for author in unprocessed_authors {
                html_authors.push(author.name.to_string());
            }
            // let markdown_authors = &html2md::parse_html(html_authors.join(", ").as_str());
            let markdown_authors = html_authors.join(", ");

            // let markdown_content = &html2md::parse_html(
            //     entry
            //     .content
            //     .to_owned()
            //     .unwrap_or(Content::default())
            //     .body
            //     .to_owned()
            //     .unwrap_or(String::from("")).as_str());
            //
            let markdown_content = entry
                .content
                .to_owned()
                .unwrap_or(Content::default())
                .body
                .to_owned()
                .unwrap_or(String::from(""));

            let unprocessed_url: String = entry.links.to_owned()[0].to_owned().href;

            let unprocessed_date: Option<DateTime<Utc>> = entry.updated.to_owned();

            entries.push(
                Entry::new(
                    markdown_title.as_str(),
                    markdown_authors.as_str(),
                    markdown_content.as_str(),
                    unprocessed_url.as_str(),
                    None,
                    unprocessed_date
                )
                );

        }
        self.entries = Some(entries);
        self.entries.clone()
    }

    pub fn format_feed_entries(&self) -> String {
        let mut resp = String::from("");
        let default_entries: Vec<Entry> = vec![];
        for entry in self.entries.as_ref().unwrap_or(&default_entries).iter() {
            resp.push('[');
            let title = format!("{:?}", entry.title);
            resp.push_str(&title[1..title.len()-1]);
            resp.push(']');
            resp.push('(');
            let url = format!("{:?}", entry.url);
            resp.push_str(&url[1..url.len()-1]);
            resp.push(')');
            if let Some(update_time) = &entry.updated_at {
                resp.push('\n');
                let update_str = format!("{:?}", update_time.to_string());
                resp.push_str(&update_str[1..update_str.len()-1]);
                resp.push_str("\n\n");
            } else {
                resp.push_str("\n\n");
            }
            let blurb = format!("{:?}", entry.blurb);
            resp.push_str(&blurb[1..blurb.len()-1]);
            resp.push_str("\n\n");
        }
        resp
    }

    pub fn get_feed_content(&self) -> Vec<String> {
        let default_entries: Vec<Entry> = vec![];
        let mut blurbs: Vec<String> = vec![];
        self.entries.as_ref().unwrap_or(&default_entries).iter().map(|entry| {
            println!("{:?}", entry);
            match &entry.content {
                Some(x) => blurbs.push(x.to_string()),
                None => {
                    if !entry.blurb.is_empty() {
                        blurbs.push(entry.blurb.to_string())
                    } else {
                        blurbs.push(entry.title.to_string())
                    }
                }
            }

        });
        blurbs
    }
}

impl fmt::Display for BlogFeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.name)
    }
}


