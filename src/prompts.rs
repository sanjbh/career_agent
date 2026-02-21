pub enum Role {
    Agent,
    Evaluator,
}

pub fn build_prompt(
    name: &str,
    summary: &str,
    linkedin: &str,
    role: Role,
) -> anyhow::Result<String> {
    match role {
        Role::Agent => todo!(),
        Role::Evaluator => todo!(),
    }
}
