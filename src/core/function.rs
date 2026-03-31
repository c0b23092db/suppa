use crate::core::hashmap_annotations;
use crate::model::{Annotation, Config};
use anyhow::Result;
use std::collections::HashMap;

pub fn print_summary(config: &Config, annotations: &[Annotation]) -> Result<()> {
    let grouped = hashmap_annotations(config, annotations)
        .into_iter()
        .map(|(label, anns)| (label, anns.len()))
        .collect::<HashMap<_, _>>();

    let mut remaining = grouped;
    println!("Annotation Summary:");
    for label in &config.labels {
        if let Some(count) = remaining.remove(&label.label) {
            let display_label = match &label.mark {
                Some(mark) => format!("{} {}", mark, label.label),
                None => label.label.clone(),
            };
            println!("  {}: {}", display_label, count);
        }
    }

    for (annotation_type, count) in remaining {
        println!("  {}: {}", annotation_type, count);
    }
    Ok(())
}
