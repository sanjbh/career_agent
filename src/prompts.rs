use async_openai::types::chat::{
    ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessageContent,
};

pub enum Role {
    Agent,
    Evaluator,
}

const AGENT_TEMPLATE: &str = include_str!("../prompts/agent_prompt.txt");
const EVALUATOR_TEMPLATE: &str = include_str!("../prompts/evaluator_prompt.txt");
const RERUN_SUFFIX: &str = include_str!("../prompts/rerun_suffix.txt");
const EVALUATOR_USER_TEMPLATE: &str = include_str!("../prompts/evaluator_user.txt");

pub fn build_prompt(name: &str, summary: &str, linkedin: &str, role: Role) -> String {
    let prompt_contents = match role {
        Role::Agent => AGENT_TEMPLATE,
        Role::Evaluator => EVALUATOR_TEMPLATE,
    };

    prompt_contents
        .replace("{name}", name)
        .replace("{summary}", summary)
        .replace("{linkedin}", linkedin)
}

pub fn build_rerun_prompt(system_prompt: &str, rejected_reply: &str, feedback: &str) -> String {
    let suffix = RERUN_SUFFIX
        .replace("{rejected_reply}", rejected_reply)
        .replace("{feedback}", feedback);

    format!("{system_prompt}\n\n{suffix}")
}

pub fn build_evaluator_user_prompt(
    history: &[ChatCompletionRequestMessage],
    message: &str,
    reply: &str,
) -> String {
    EVALUATOR_USER_TEMPLATE
        .replace("{history}", &format_history(history))
        .replace("{message}", message)
        .replace("{reply}", reply)
}

fn format_history(history: &[ChatCompletionRequestMessage]) -> String {
    history
        .iter()
        .map(|msg| match msg {
            ChatCompletionRequestMessage::System(m) => {
                let text = match &m.content {
                    ChatCompletionRequestSystemMessageContent::Text(t) => t.as_str(),
                    _ => "[non-text content]",
                };
                format!("system: {}", text)
            }
            ChatCompletionRequestMessage::User(m) => {
                let text = match &m.content {
                    ChatCompletionRequestUserMessageContent::Text(t) => t.as_str(),
                    _ => "[non-text content]",
                };
                format!("user: {}", text)
            }
            ChatCompletionRequestMessage::Assistant(m) => {
                let text = match &m.content {
                    Some(ChatCompletionRequestAssistantMessageContent::Text(t)) => t.as_str(),
                    _ => "[non-text content]",
                };
                format!("assistant: {}", text)
            }
            _ => String::new(),
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join("\n")
}
