use lopdf::Document;
use std::fmt;

const TRIM_PRESERVED_CHARS_LEN: usize = 15;

/// search result
#[derive(Debug)]
pub struct PdfSearchResult {
    pub page_num: u32,
    pub text: String,
}

impl fmt::Display for PdfSearchResult {
    /// format for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p.{}: {}", self.page_num, self.text)
    }
}

/// search main handler
pub fn handle(
    keyword: &str,
    filepath: &str,
) -> Result<Vec<PdfSearchResult>, Box<dyn std::error::Error>> {
    let mut ret: Vec<PdfSearchResult> = vec![];

    let doc = Document::load(filepath)?;

    for page_id in doc.get_pages().keys() {
        if let Ok(page_text) = doc.extract_text(&[*page_id]) {
            if page_text.contains(keyword) {
                let found = PdfSearchResult {
                    page_num: page_id.to_owned(),
                    text: trim_around_keyword(page_text.as_str(), keyword),
                };
                ret.push(found);
            }
        }
    }

    Ok(ret)
}

/// optimize search result by limiting page text into keyword and the surroundings only
fn trim_around_keyword(input: &str, keyword: &str) -> String {
    let pos = input.find(keyword).unwrap();

    let start = if TRIM_PRESERVED_CHARS_LEN <= pos {
        pos - TRIM_PRESERVED_CHARS_LEN
    } else {
        0
    };
    let end = if pos + keyword.len() + TRIM_PRESERVED_CHARS_LEN <= input.len() {
        pos + keyword.len() + TRIM_PRESERVED_CHARS_LEN
    } else {
        input.len()
    };

    input[start..end].to_string()
}
