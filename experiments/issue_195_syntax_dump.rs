// Temporary: dumps syntax mappings (term, byte start, byte end) for files.
use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn main() {
    let mut args = std::env::args().skip(1);
    let language = args.next().expect("language");
    for path in args {
        let source = std::fs::read_to_string(&path).expect("file");
        let network = LinkNetwork::parse(&source, &language, ParseConfiguration::default());
        println!("== {path} clean={}", network.verify_full_match(None).is_clean());
        for link in network.links() {
            if link.metadata().link_type() != Some(LinkType::Syntax) { continue; }
            let Some(span) = link.metadata().span() else { continue };
            let Some(term) = link.metadata().term() else { continue };
            println!("{term} {} {}", span.byte_range().start(), span.byte_range().end());
        }
    }
}
