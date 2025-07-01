use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Sentiment {
    Agreement,
    Disagreement,
    Neutral,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Citation {
    pub id: String,
    // Surrounding text
    pub context: String,
    pub sentiment: Option<Sentiment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub citations: Vec<Citation>,
}
