/// Book's configuration.
/// Maps directly to the crowbook book.toml format

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
}