use std::error::Error;

use career_agent::{config, document, prompts};

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let config = config::AppConfig::load()?;
    let pdf_content = document::read_pdf("me/linkedin.pdf")?;
    let summary_text = document::read_text("me/summary.txt")?;

    // let agent_system_prompt = prompts::build_prompt(name, summary, linkedin, role)
    unimplemented!()
}
