use anyhow::{Context, Result};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

pub struct NLPModel {
    model: rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel,
}

impl NLPModel {
    pub fn new() -> Result<Self> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .context("Failed to create sentence embeddings model")?;
        Ok(NLPModel { model })
    }

    pub fn get_embeddings(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // Filter out invalid texts
        let valid_texts: Vec<String> = texts
            .iter()
            .filter(|text| {
                let trimmed = text.trim();
                if trimmed.is_empty() || trimmed.len() < 5 {
                    eprintln!("Skipping invalid text for embedding: '{}'", trimmed);
                    false
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        if valid_texts.is_empty() {
            return Err(anyhow::anyhow!("No valid texts provided for embedding"));
        }

        // Log input texts for debugging
        for (i, text) in valid_texts.iter().enumerate() {
            println!("Processing text {}: '{}'", i, text);
        }

        self.model
            .encode(&valid_texts)
            .context("Failed to generate sentence embeddings")
    }
}
