//! An acceptance-criterion id names exactly one row of its requirement file.
//! Two rows with one id make every citation of it ambiguous, so the repository
//! carries none.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn markdown_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).expect("readable directory") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

/// The ids of the criterion rows (`| FR-1-AC-2 | ...`) in one file's text that
/// appear more than once.
fn duplicates(text: &str) -> Vec<String> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("| ") else {
            continue;
        };
        let Some((id, _)) = rest.split_once(" |") else {
            continue;
        };
        let mut parts = id.split("-AC-");
        let (Some(head), Some(number), None) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        if head.contains(' ') || number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        *seen.entry(id).or_default() += 1;
    }
    seen.into_iter()
        .filter(|(_, n)| *n > 1)
        .map(|(id, _)| id.to_owned())
        .collect()
}

#[test]
fn no_requirement_file_repeats_an_acceptance_criterion_id() {
    let mut files = Vec::new();
    markdown_files(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("spec"),
        &mut files,
    );
    assert!(files.len() > 100, "the spec tree was found");
    let mut repeated = Vec::new();
    for file in files {
        let text = fs::read_to_string(&file).expect("utf-8 markdown");
        for id in duplicates(&text) {
            repeated.push(format!("{}: {id}", file.display()));
        }
    }
    assert_eq!(repeated, Vec::<String>::new());
}

#[test]
fn the_check_sees_a_repeated_id() {
    let text = "| FR-1-AC-1 | a | Test |\n| FR-1-AC-2 | b | Test |\n| FR-1-AC-1 | c | Test |\n| Member | x |\n";
    assert_eq!(duplicates(text), vec!["FR-1-AC-1".to_owned()]);
    assert!(duplicates("| FR-1-AC-1 | a |\n| FR-1-AC-2 | b |\n").is_empty());
}
