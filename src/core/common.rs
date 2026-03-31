use crate::model::{Annotation, Config};
use std::collections::HashMap;
use std::fs::canonicalize;
use std::path::Path;

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

/// プロジェクト名をルートディレクトリから推測する
pub fn project_name_from_root(root: &Path) -> String {
    root.file_name()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
        .map(ToString::to_string)
        .or_else(|| {
            canonicalize(root)
                .ok()
                .and_then(|p| p.file_name().map(|name| name.to_owned()))
                .and_then(|s| s.to_str().map(ToString::to_string))
        })
        .unwrap_or_else(|| "Project".to_string())
}
