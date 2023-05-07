use chrono::{DateTime, Utc};
use feed_rs::{self, model::{Person, Content}};
use std::fmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    title: String,
    authors: String,
    blurb: String,
    url: String,
    content: Option<String>,
    // updated_at: Option<DateTime<Utc>>,
    updated_at: Option<String>,
}

impl Entry {
    fn new(title: &str, authors: &str, blurb: &str, url: &str, content: Option<String>, updated_at: Option<DateTime<Utc>>) -> Entry {
        let updated_at_str: String;
        match updated_at {
            Some(x) => { updated_at_str = x.to_string() },
            None => { updated_at_str = String::from("") }
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
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {:?}\n\nAuthor(s): {:?}\n\nUpdated At: {:?}\n\nBlurb: {:?}", &self.title, &self.authors, &self.updated_at, &self.blurb)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogFeed {
    pub name: String,
    pub url: String,
    pub entries: Option<Vec<Entry>>,
}

impl BlogFeed {
    pub fn new(url: &str, name: &str) -> BlogFeed {
        return BlogFeed {
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

    pub fn populate_entries(&mut self, feed: &feed_rs::model::Feed) -> &Option<Vec<Entry>> {
        let mut entries: Vec<Entry> = vec![];
        for entry in feed.entries.iter() {
            let markdown_title = &html2md::parse_html(entry.title.as_ref().unwrap().content.as_str());

            let unprocessed_authors: Vec<Person> = entry.authors.to_owned();
            let mut html_authors: Vec<String> = vec![];
            for author in unprocessed_authors {
                html_authors.push(author.name.to_string());
            }
            let markdown_authors = &html2md::parse_html(html_authors.join(", ").as_str());

            let markdown_content = &html2md::parse_html(
                entry
                .content
                .to_owned()
                .unwrap_or(Content::default())
                .body
                .to_owned()
                .unwrap_or(String::from("")).as_str());

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
        return &self.entries
    }

    pub fn format_feed_entries(&self) -> String {
        let mut resp = String::from("");
        let default_entries: Vec<Entry> = vec![];
        for entry in self.entries.as_ref().unwrap_or(&default_entries).iter() {
            resp.push_str("[");
            let title = format!("{:?}", entry.title.clone());
            resp.push_str(&title[1..title.len()-1]);
            resp.push_str("]");
            resp.push_str("(");
            let url = format!("{:?}", entry.url.clone());
            resp.push_str(&url[1..url.len()-1]);
            resp.push_str(")");
            if let Some(update_time) = &entry.updated_at {
                resp.push_str("\n");
                let update_str = format!("{:?}", update_time.to_string());
                resp.push_str(&update_str[1..update_str.len()-1]);
                resp.push_str("\n\n");
            } else {
                resp.push_str("\n\n");
            }
            let blurb = format!("{:?}", entry.blurb.clone());
            resp.push_str(&blurb[1..blurb.len()-1]);
            resp.push_str("\n\n");
        }
        return resp
    }
}

impl fmt::Display for BlogFeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.name)
    }
}


