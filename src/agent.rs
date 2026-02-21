use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};

use crate::{config::LlmConfig, prompts};

pub struct Agent {
    client: Client<OpenAIConfig>,
    system_prompt: String,
    model: String,
}

impl Agent {
    pub fn new(config: &LlmConfig, system_prompt: String) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_base(&config.base_url)
            .with_api_key(&config.api_key);
        Self {
            client: Client::with_config(openai_config),
            system_prompt,
            model: config.model.clone(),
        }
    }

    pub async fn chat(
        &self,
        message: &str,
        history: &[ChatCompletionRequestMessage],
    ) -> anyhow::Result<String> {
        self.complete(&self.system_prompt, history, message).await
    }

    pub async fn rerun(
        &self,
        message: &str,
        history: &[ChatCompletionRequestMessage],
        rejected_reply: &str,
        feedback: &str,
    ) -> anyhow::Result<String> {
        let augmented_system_prompt =
            prompts::build_rerun_prompt(&self.system_prompt, rejected_reply, feedback);

        self.complete(&augmented_system_prompt, history, message)
            .await
    }

    async fn complete(
        &self,
        system_message: &str,
        history: &[ChatCompletionRequestMessage],
        user_message: &str,
    ) -> anyhow::Result<String> {
        let mut messages: Vec<ChatCompletionRequestMessage> = Vec::with_capacity(history.len() + 2);

        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_message)
                .build()?
                .into(),
        );

        messages.extend_from_slice(history);

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_message)
                .build()?
                .into(),
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .build()?;

        let response = self.client.chat().create(request).await?;

        response.choices[0]
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow::anyhow!("empty response"))
    }
}
