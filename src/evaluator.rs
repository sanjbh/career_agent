use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Evaluation {
    pub is_acceptable: bool,
    pub feedback: String,
}
