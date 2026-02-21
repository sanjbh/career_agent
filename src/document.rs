use anyhow::Context;
use std::{fs, path::PathBuf};

pub fn read_pdf(file_name: &str) -> anyhow::Result<String> {
    let path = PathBuf::from(file_name);

    pdf_extract::extract_text(&path).context(format!("Failed to extract text from {:?}", path))
}

pub fn read_text(file_name: &str) -> anyhow::Result<String> {
    fs::read_to_string(file_name).context(format!("Failed to extract text from {file_name}"))
}
