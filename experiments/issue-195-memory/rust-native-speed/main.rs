// Parses each file given on the command line with `LinkNetwork::parse` and
// the default configuration (so the native grammar of the language and the
// default memory budget apply), the way formal-ai's corpus inventory does,
// and prints the bytes, the seconds, the link count and whether the root is
// an ERROR node. Run it bounded on a handful of files:
//
//   (ulimit -v 6000000; timeout 900 node experiments/run-rust-experiment.mjs \
//     experiments/issue-195-memory/rust-native-speed/main.rs rust FILE...)
use std::time::Instant;

use meta_language::{LinkNetwork, ParseConfiguration};

fn main() {
    let mut args = std::env::args().skip(1);
    let language = args.next().expect("language");
    for file in args {
        let text = std::fs::read_to_string(&file).expect("read");
        let start = Instant::now();
        let network = LinkNetwork::parse(&text, &language, ParseConfiguration::default());
        let seconds = start.elapsed().as_secs_f64();
        println!(
            "{{\"file\":{file:?},\"bytes\":{},\"seconds\":{seconds:.2},\"kBps\":{:.1},\"links\":{}}}",
            text.len(),
            text.len() as f64 / 1024.0 / seconds,
            network.links().count()
        );
    }
}
