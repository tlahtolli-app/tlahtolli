pub mod archive;
pub mod book_config;
pub mod chapter;
pub mod image;

use std::path::PathBuf;
use crate::project::book_config::BookConfig;
use crate::project::chapter::Chapter;

/// Represents a project open in memory
/// 
/// When a .tl file is opened, it's decompressed into a temporary directory.
/// All edits happen on those files. When the user saves, the directory is
/// recompressed into the .tl file.
///
#[derive(Debug)]
pub struct Project {
    pub path: Option<PathBuf>,
    pub temp_dir: PathBuf,
    pub config: BookConfig,
    pub chapters: Vec<Chapter>, // order in this Vec determines order in the exported book
    pub unsaved: bool,
}

impl Project {
    // Creates a new empty project with one blank chapter
    pub fn new(title: String, author: String, lang: String, temp_dir: PathBuf) -> Self {
        let first_chapter = Chapter::new(0);

        Self {
            path: None,
            temp_dir,
            config: BookConfig::new(title, author, lang),
            chapters: vec![first_chapter],
            unsaved: true, // new project always starts unsaved
        }
    }

    // Returns path to chapters dir in temp folder
    pub fn chapters_dir(&self) -> PathBuf {
        self.temp_dir.join("chapters")
    }

    // Returns path to assets dir
    pub fn assets_dir(&self) -> PathBuf {
        self.temp_dir.join("assets")
    }

    // Returns path to book.toml
    pub fn book_toml_path(&self) -> PathBuf {
        self.temp_dir.join("book.toml")
    }

    // Returns path for a given chapter's .md file
    pub fn chapter_path(&self, chapter: &Chapter) -> PathBuf {
        self.chapters_dir().join(chapter.filename())
    }

    // Returns the index of the active chapter
    pub fn chapter_by_id(&self, id: &uuid::Uuid) -> Option<&Chapter> {
        self.chapters.iter().find(|c| &c.id == id)
    }

    // Returns a mutable reference to a chapter by UUID
    pub fn chapter_by_id_mut(&mut self, id: &uuid::Uuid) -> Option<&mut Chapter> {
        self.chapters.iter_mut().find(|c| &c.id == id)
    }

    // Adds a new chapter at the end of the book and returns it's id
    pub fn add_chapter(&mut self) -> uuid::Uuid {
        let order = self.chapters.len();
        let chapter = Chapter::new(order);
        let id = chapter.id;
        self.chapters.push(chapter);
        self.unsaved = true;
        id
    }

    pub fn mark_unsaved(&mut self) {
        self.unsaved = true;
    }

    pub fn mark_saved(&mut self) {
        self.unsaved = false;
    }
}