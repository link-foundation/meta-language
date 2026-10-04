// Parses each file the way formal-ai's grammar_projection_corpus_ratchet does
// (LinkNetwork::parse with the default configuration) and prints, per file, its
// size, the link count, the time, and the process's resident and peak memory.
//
//   node experiments/run-rust-experiment.mjs experiments/formal-ai-corpus-memory/main.rs <label> <file...>
use std::time::Instant;

fn status(field: &str) -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    status
        .lines()
        .find(|line| line.starts_with(field))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|kb| kb.parse().ok())
        .unwrap_or(0)
        / 1024
}

fn main() {
    let mut args = std::env::args().skip(1);
    let label = args.next().expect("label");
    for file in args {
        let text = std::fs::read_to_string(&file).expect("read");
        let start = Instant::now();
        let network =
            meta_language::LinkNetwork::parse(&text, &label, meta_language::ParseConfiguration::default());
        let links = network.links().count();
        let elapsed = start.elapsed();
        let rss_with = status("VmRSS:");
        drop(network);
        println!(
            "{file}: {} bytes, {links} links, {:.2}s, rss {} MiB holding / {} MiB after drop, peak {} MiB",
            text.len(),
            elapsed.as_secs_f64(),
            rss_with,
            status("VmRSS:"),
            status("VmHWM:")
        );
    }
}
