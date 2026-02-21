use anyhow::Context;
use validator::Validate;

#[derive(Validate)]
pub struct LlmConfig {
    #[validate(url)]
    pub base_url: String,

    #[validate(length(min = 1))]
    pub api_key: String,

    #[validate(length(min = 1))]
    pub model: String,
}

#[derive(Validate)]
pub struct AppConfig {
    #[validate(nested)]
    pub agent: LlmConfig,

    #[validate(nested)]
    pub evaluator: LlmConfig,

    pub name: String,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let agent_config = LlmConfig {
            base_url: std::env::var("AGENT_BASE_URL").context("AGENT_BASE_URL must be set")?,
            api_key: std::env::var("AGENT_API_KEY").context("AGENT_API_KEY must be set")?,
            model: std::env::var("AGENT_MODEL").context("AGENT_MODEL must be set")?,
        };

        let evaluator_config = LlmConfig {
            api_key: std::env::var("EVALUATOR_API_KEY").context("EVALUATOR_API_KEY must be set")?,
            base_url: std::env::var("EVALUATOR_BASE_URL")
                .context("EVALUATOR_BASE_URL must be set")?,
            model: std::env::var("EVALUATOR_MODEL").context("EVALUATOR_MODEL must be set")?,
        };

        let app_config = AppConfig {
            agent: agent_config,
            evaluator: evaluator_config,
            name: std::env::var("PERSON_NAME").context("PERSON_NAME must be set")?,
        };

        app_config.validate()?;

        Ok(app_config)
    }
}
