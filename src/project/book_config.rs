/// Book's configuration.
/// Maps directly to the crowbook book.toml format

use std::path::Path;
use std::fs;

use egui::epaint::tessellator::path;

#[derive(Debug, Clone)]
pub struct BookConfig {
    pub title: String,
    pub author: String,
    pub lang: String,
    pub description: Option<String>,
    pub cover: Option<String>, // path to the cover image, usually "assets/cover.webp"
    pub date: Option<String>, // publication date
    pub output: OutputFormats, // at least one should be enabled
    pub rendering: RenderingOptions,
    pub pdf: PdfOptions,
}

#[derive(Debug,Clone)]
pub struct OutputFormats{
    pub epub: bool,
    pub html: bool,
    pub pdf: bool,
}

/// Rendering options that apply to all output formats
#[derive(Debug, Clone)]
pub struct RenderingOptions {
    pub initials: bool, // display drop cap at the start of each chapter?
    pub inline_toc: bool, // include an inline table of contents?
    pub chapter_label: Option<String>,
    pub chapter_roman_numerals: bool,
    pub part_label: Option<String>,
    pub part_roman_numerals: bool,
}

/// PDF page layout options
#[derive(Debug, Clone)]
pub struct PdfOptions {
    pub page_size:String,
    pub orientation: String,
    pub margins: PageMargins,
}

/// Page margins in millimeters
#[derive(Debug, Clone)]
pub struct PageMargins {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl BookConfig {
    pub fn new(title: String, author: String, lang: String) -> Self {
        Self { 
            title, 
            author, 
            lang, 
            description: None, 
            cover: None,
            date: None, 
            output: OutputFormats { epub: true, html: false, pdf: true }, 
            rendering: RenderingOptions {
                initials: false,
                inline_toc: false,
                chapter_label: None,
                chapter_roman_numerals: false,
                part_label: None,
                part_roman_numerals: true,
            },
            pdf: PdfOptions {
                page_size: String::from("a5"),
                orientation: String::from("portrait"),
                margins: PageMargins { top: 20, bottom: 20, left: 25, right: 20 },
            },
        }
    }

    // Serializes the book configuration into crowbook's book.toml
    pub fn write_crowbook_toml(&self, path: &Path, chapter_entries: &[String]) -> std::io::Result<()> {
        let mut content = String::new();

        // Metadata
        content.push_str(&format!("title: {}\n", self.title));
        content.push_str(&format!("author: {}\n", self.author));
        content.push_str(&format!("lang: {}\n", self.lang));

        if let Some(desc) = &self.description {
            content.push_str(&format!("description: {}\n", desc));
        }

        if let Some(cover) = &self.cover {
            content.push_str(&format!("cover: {}\n", cover));
        }

        if let Some(date) = &self.date {
            content.push_str(&format!("dat: {}\n", date));
        }

        // Output formats
        let mut outputs = Vec::new();
        if self.output.epub { outputs.push("epub"); }
        if self.output.html { outputs.push("html"); }

        if !outputs.is_empty() {
            content.push_str(&format!("output: [{}]\n", outputs.join(", ")));
        }

        // Rendering options
        content.push_str("\n# Rendering options\n");
        content.push_str(&format!(
            "rendering.initials: {}\n",
            self.rendering.initials
        ));
        content.push_str(&format!(
            "rendering.inline_toc: {}\n",
            self.rendering.inline_toc
        ));
        content.push_str(&format!(
            "rendering.chapter.roman_numerals: {}\n",
            self.rendering.chapter_roman_numerals
        ));

        if let Some(label) = &self.rendering.chapter_label {
            content.push_str(&format!("rendering.chapter: {}\n", label));
        }

        if let Some(label) = &self.rendering.part_label {
            content.push_str(&format!("rendering.part: {}\n", label));
        }

        // Chapter list
        content.push_str("\n# Chapters\n");
        for entry in chapter_entries {
            content.push_str(&format!("{}\n", entry));
        }

        fs::write(path, content)
    }

    // Serializes the PDF styling options into markdown2pdf's TOML format.
    // This file is generated at export time
    pub fn write_pdf_config(&self, path: &Path) -> std::io::Result<()> {
        let mut content = String::new();

        content.push_str("[page]\n");
        content.push_str(&format!("size = \"{}\"\n", self.pdf.page_size));
        content.push_str(&format!("orientation =  \"{}\"\n", self.pdf.orientation));
        content.push_str(&format!(
            "margins = {{ top = {}, bottom= {}, left = {}, right = {} }}\n",
            self.pdf.margins.top,
            self.pdf.margins.bottom,
            self.pdf.margins.left,
            self.pdf.margins.right,
        ));

        fs::write(path, content)
    }
}