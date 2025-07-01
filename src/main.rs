use crate::classifier::Classifier;
use crate::models::Report;
use crate::parser::parse_manuscript;
use anyhow::Result;
use serde_json;
use std::fs::File;
use std::io::Write;

mod classifier;
mod models;
mod nlp;
mod parser;

fn main() -> Result<()> {
    // Parse manuscript
    let mut citations = parse_manuscript("data/input_manuscript.txt")?;

    // Classify citations
    let classifier = Classifier::new()?;
    classifier.classify(&mut citations)?;

    // Generate report
    let report = Report { citations };
    let json = serde_json::to_string_pretty(&report)?;
    let mut file = File::create("data/output_report.json")?;
    file.write_all(json.as_bytes())?;

    println!("Sentiment analysis complete. Report saved to data/output_report.json");
    Ok(())
}
