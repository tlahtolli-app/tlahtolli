use uuid::Uuid;
use rust_i18n::t;

/// A chapter represents a single writing unit within the book. 
/// A .md file inside the projectś temporary folder.
/// 
/// The title is optional. If there's no title, the UI displays
/// an excerpt of the first line instead.

#[derive(Debug, Clone)]

pub struct Chapter {
    pub id: Uuid,
    pub title: Option<String>, // None if the chapter has no name
    pub order: usize,
    // Whether this chapter is numbered in te exported output.
    // true -> "Chapter 3"
    // false -> included wihtout a number (preface, epilogue, etc)
    pub numbered: bool,
    pub part: Option<String>, // None if the book has no parts
    pub excerpt: Option<String>,
}

impl Chapter {

    // New chapter with default values
    pub fn new(order: usize) -> Self {
        Self {
            id: Uuid::new_v4(), // generate a random UUID
            title: None,
            order,
            numbered: true, // chapters are numbered by default
            part: None,
            excerpt: None
        }
    }

    // Return the filename used to store this chapter on disk.
    pub fn filename(&self) -> String {
        format!("{}.md", self.id)
    }

    // Returns the text to display in the sidebar
    pub fn display_name(&self) -> String {
        if let Some(title) = &self.title {
            title.clone()
        } else if let Some(excerpt) = &self.excerpt {
            excerpt.clone()
        } else {
            // Placeholder text for chapters with no title and no content yet
            t!("chapter.empty").to_string()
        }
    }

    pub fn crowbook_entry(&self) -> String {
        let prefix = if self.numbered { "+" } else { "-" };
        format!("{} chapters/{}", prefix, self.filename())
    }
}