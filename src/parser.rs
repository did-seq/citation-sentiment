use crate::models::Citation;
use anyhow::{Context, Result};
use pdf_extract::extract_text;
use regex::Regex;
use std::fs::File;
use std::io::Read;

pub fn parse_manuscript(input_path: &str) -> Result<Vec<Citation>> {
    let content = if input_path.ends_with(".pdf") {
        extract_text(input_path).context("Failed to extract text from PDF")?
    } else {
        let mut file =
            File::open(input_path).context(format!("Failed to open file: {}", input_path))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context("Failed to read file content")?;
        content
    };

    if content.trim().is_empty() {
        return Err(anyhow::anyhow!("Input manuscript is empty"));
    }

    println!("Input content length: {} characters", content.len());

    // Regex to match:
    // - (Author, Year)
    // - [Number]
    // - Author Year
    let re = Regex::new(
        r"\(([^,]+),\s*(\d{4})\)|\[\d+\]|(?:\b[A-Z][a-zA-Z\.]+(?:\s+[A-Z][a-zA-Z]+)*\s+\d{4}\b)",
    )
    .context("Invalid regex pattern")?;
    let mut citations = Vec::new();

    let sentences: Vec<&str> = content.split('.').map(|s| s.trim()).collect();
    println!("Found {} sentences", sentences.len());

    for (i, sentence) in sentences.iter().enumerate() {
        if sentence.is_empty() {
            println!("Skipping empty sentence at index {}", i);
            continue;
        }

        let matches: Vec<_> = re.captures_iter(sentence).collect();
        if matches.is_empty() {
            println!("No citations found in sentence: '{}'", sentence);
        }

        for cap in matches {
            let citation_id = if let Some(author) = cap.get(1) {
                format!(
                    "{} {}",
                    author.as_str(),
                    cap.get(2).map_or("", |m| m.as_str())
                )
            } else if cap[0].starts_with('[') {
                cap[0].to_string()
            } else {
                cap[0].to_string()
            };

            let context: Vec<String> = vec![
                sentences.get(i.wrapping_sub(1)).unwrap_or(&"").to_string(),
                sentence.to_string(),
                sentences.get(i + 1).unwrap_or(&"").to_string(),
            ];

            let mut context = context.join(". ").trim().to_string();
            context = context.replace(|c: char| c.is_control() || c == '\n' || c == '\r', " ");
            context = Regex::new(r"\s+")
                .unwrap()
                .replace_all(&context, " ")
                .to_string();

            if context.is_empty() || context.len() < 5 {
                eprintln!(
                    "Skipping citation '{}' with invalid context: '{}'",
                    citation_id, context
                );
                continue;
            }

            println!(
                "Extracted citation '{}': context = '{}'",
                citation_id, context
            );
            citations.push(Citation {
                id: citation_id,
                context,
                sentiment: None,
            });
        }
    }

    if citations.is_empty() {
        return Err(anyhow::anyhow!(
            "No valid citations found in the manuscript. Check input format or regex pattern."
        ));
    }

    println!("Extracted {} citations", citations.len());
    Ok(citations)
}
