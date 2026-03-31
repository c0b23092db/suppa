use std::collections::HashMap;
use crate::model::{Config,Annotation};

/// アノテーションのリストを、HashMap<ラベル, Vec<アノテーション>>形式にする
pub fn hashmap_annotations<'a>(
    config: &Config,
    annotations: &'a [Annotation],
) -> HashMap<String, Vec<&'a Annotation>> {
    let alias_to_label = config
        .labels
        .iter()
        .flat_map(|label| {
            label
                .alias
                .iter()
                .cloned()
                .map(move |alias| (alias, label.label.clone()))
        })
        .collect::<HashMap<_, _>>();

    let mut map: HashMap<String, Vec<&Annotation>> = HashMap::new();
    for a in annotations {
        let canonical = alias_to_label
            .get(&a.label)
            .cloned()
            .unwrap_or_else(|| a.label.clone());
        map.entry(canonical).or_default().push(a);
    }
    map
}