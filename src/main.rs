use career_agent::{agent, config, document, evaluator, prompts, server};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::AppConfig::load()?;
    let pdf_content = document::read_pdf("me/linkedin.pdf")?;
    let summary_text = document::read_text("me/summary.txt")?;

    let agent_prompt = prompts::build_prompt(&config.name, &summary_text, &pdf_content, prompts::Role::Agent);
    let evaluator_prompt = prompts::build_prompt(&config.name, &summary_text, &pdf_content, prompts::Role::Evaluator);

    let agent = agent::Agent::new(&config.agent, agent_prompt);
    let evaluator = evaluator::Evaluator::new(&config.evaluator, evaluator_prompt);

    let router = server::router(agent, evaluator);

    let addr = "127.0.0.1:8080";
    tracing::info!("Starting server at {addr}");

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
