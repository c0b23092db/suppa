#[cfg(test)]
mod tests {
    use super::{build_updated_markdown, archive_header_for_label};
    use crate::model::{Annotation, Config, LabelDefinition};
    use std::path::{Path, PathBuf};

    fn test_config() -> Config {
        Config {
            comments: vec!["//".to_string()],
            exclude: Vec::new(),
            labels: vec![LabelDefinition {
                label: "TODO".to_string(),
                enabled: true,
                mark: Some("✅".to_string()),
                checkbox: true,
                alias: Vec::new(),
            }],
        }
    }

    fn annotation(content: &str, line: u64) -> Annotation {
        Annotation {
            file: PathBuf::from("src/main.rs"),
            line,
            label: "TODO".to_string(),
            content: content.to_string(),
        }
    }

    #[test]
    fn update_moves_checked_items_to_archive() {
        let existing = "# suppa\n\n## ✅ TODO\n- [ ] AAA (src/main.rs:1)\n- [x] BBB (src/main.rs:2)\n## Archive:TODO\n- [x] CCC (src/old.rs:9)\n";
        let annotations = vec![annotation("AAA", 1), annotation("BBB", 20)];

        let markdown =
            build_updated_markdown(existing, Path::new("suppa"), &test_config(), &annotations);

        let expected = format!(
            "# suppa\n\n## ✅ TODO\n- [ ] AAA (src/main.rs:1)\n{}\n- [x] BBB (src/main.rs:20)\n- [x] CCC (src/old.rs:9)\n",
            archive_header_for_label("TODO")
        );
        assert_eq!(markdown, expected);
    }

    #[test]
    fn update_preserves_archived_items_without_source_annotations() {
        let existing = "# suppa\n\n## ✅ TODO\n- [x] BBB (src/main.rs:2)\n";

        let markdown =
            build_updated_markdown(existing, Path::new("suppa"), &test_config(), &[]);

        let expected = format!(
            "# suppa\n\n## ✅ TODO\n{}\n- [x] BBB (src/main.rs:2)\n",
            archive_header_for_label("TODO")
        );
        assert_eq!(markdown, expected);
    }
}
