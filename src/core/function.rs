use crate::core::hashmap_annotations;
use crate::model::{Annotation, Config};
use anyhow::Result;
use std::collections::HashMap;

pub fn print_summary(config: &Config, annotations: &[Annotation]) -> Result<()> {
    let label_marks: HashMap<String, String> = config
        .labels
        .iter()
        .filter_map(|label| {
            label
                .mark
                .as_ref()
                .map(|mark| (label.label.clone(), mark.clone()))
        })
        .collect();
    let summary = hashmap_annotations(config, annotations)
        .into_iter()
        .map(|(label, anns)| (label, anns.len()))
        .collect::<Vec<_>>();
    println!("Annotation Summary:");
    for (annotation_type, count) in summary {
        let display_label = match label_marks.get(&annotation_type) {
            Some(mark) => format!("{} {}", mark, annotation_type),
            None => annotation_type,
        };
        println!("  {}: {}", display_label, count);
    }
    Ok(())
}
