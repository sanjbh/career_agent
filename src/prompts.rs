pub enum Role {
    Agent,
    Evaluator,
}

const AGENT_TEMPLATE: &str = include_str!("../prompts/agent_prompt.txt");
const EVALUATOR_TEMPLATE: &str = include_str!("../prompts/evaluator_prompt.txt");

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
