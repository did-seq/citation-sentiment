use crate::models::{Citation, Sentiment};
use crate::nlp::NLPModel;
use anyhow::{Context, Result};

pub struct Classifier {
    nlp: NLPModel,
}

impl Classifier {
    pub fn new() -> Result<Self> {
        Ok(Classifier {
            nlp: NLPModel::new()?,
        })
    }

    pub fn classify(&self, citations: &mut [Citation]) -> Result<()> {
        let contexts: Vec<String> = citations.iter().map(|c| c.context.clone()).collect();
        let embeddings = self.nlp.get_embeddings(&contexts).unwrap_or_else(|e| {
            eprintln!(
                "Embedding failed: {}. Falling back to rule-based classification.",
                e
            );
            vec![vec![]; contexts.len()]
        });

        for (citation, _embedding) in citations.iter_mut().zip(embeddings.iter()) {
            let context = citation.context.to_lowercase();
            println!(
                "Classifying citation '{}': context = '{}'",
                citation.id, context
            );

            // Prioritize disagreement keywords
            citation.sentiment = Some(
                if context.contains("contrary to")
                    || context.contains("disagree")
                    || context.contains("contradicts")
                {
                    Sentiment::Disagreement
                } else if context.contains("consistent with")
                    || context.contains("supports")
                    || context.contains("in line with")
                {
                    Sentiment::Agreement
                } else {
                    Sentiment::Neutral
                },
            );

            println!("Assigned sentiment: {:?}", citation.sentiment);
        }
        Ok(())
    }
}
