mod core;

use core::search::{PdfSearchResult, handle};

/// entry point
pub fn search(
    keyword: &str,
    filepath: &str,
) -> Result<Vec<PdfSearchResult>, Box<dyn std::error::Error>> {
    handle(keyword, filepath)
}
