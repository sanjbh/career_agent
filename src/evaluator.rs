use anyhow::Context;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};
use serde::Deserialize;

use crate::{config::LlmConfig, prompts};

#[derive(Debug, Deserialize)]
pub struct Evaluation {
    pub is_acceptable: bool,
    pub feedback: String,
}

pub struct Evaluator {
    client: Client<OpenAIConfig>,
    model: String,
    system_prompt: String,
}

impl Evaluator {
    pub fn new(config: &LlmConfig, system_prompt: String) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_base(&config.base_url)
            .with_api_key(&config.api_key);

        Self {
            client: Client::with_config(openai_config),
            model: config.model.clone(),
            system_prompt,
        }
    }
    pub async fn evaluate(
        &self,
        message: &str,
        reply: &str,
        history: &[ChatCompletionRequestMessage],
    ) -> anyhow::Result<Evaluation> {
        let evaluator_user_prompt = prompts::build_evaluator_user_prompt(history, message, reply);
        let mut messages: Vec<ChatCompletionRequestMessage> = Vec::with_capacity(2);

        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(self.system_prompt.as_str())
                .build()?
                .into(),
        );

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(evaluator_user_prompt)
                .build()?
                .into(),
        );
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .build()?;

        let response = self.client.chat().create(request).await?;

        let content = response.choices[0]
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow::anyhow!("empty response"))?;

        serde_json::from_str::<Evaluation>(&content).context("failed to parse evaluation JSON")
    }
}
