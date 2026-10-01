// Reproduces the generated-parser failure after a rustc upgrade: a restored
// target cache keeps libpest_derive built by the old compiler next to the new
// one. Lexicographic selection can pick the stale artifact; newest-mtime cannot.
//   rustc --edition 2024 experiments/issue-195-stale-dependency-artifact.rs -o /tmp/stale && /tmp/stale
use std::fs::{self, File};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn main() {
    let dir = std::env::temp_dir().join(format!("stale-artifacts-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let stale = dir.join("libpest_derive-0000.so");
    let current = dir.join("libpest_derive-ffff.so");
    for (path, age) in [(&stale, 3600), (&current, 0)] {
        File::create(path)
            .unwrap()
            .set_modified(SystemTime::now() - Duration::from_secs(age))
            .unwrap();
    }
    let mut candidates: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    candidates.sort();
    let lexicographic = candidates.first().cloned().unwrap();
    let newest = candidates
        .iter()
        .max_by_key(|p| {
            fs::metadata(p)
                .and_then(|metadata| metadata.modified())
                .ok()
        })
        .cloned()
        .unwrap();
    fs::remove_dir_all(&dir).unwrap();
    println!(
        "lexicographic (before): {}",
        lexicographic.file_name().unwrap().to_string_lossy()
    );
    println!(
        "newest mtime (after):   {}",
        newest.file_name().unwrap().to_string_lossy()
    );
    assert_eq!(
        lexicographic, stale,
        "before the fix the stale artifact is chosen"
    );
    assert_eq!(
        newest, current,
        "after the fix the current artifact is chosen"
    );
}
