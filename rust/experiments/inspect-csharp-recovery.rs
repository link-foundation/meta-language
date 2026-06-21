//! ```cargo
//! [dependencies]
//! meta-language = { path = ".." }
//! ```

use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn main() {
    let samples = [
        ("old fixture", "class C { void M() { if ( }"),
        ("missing if paren", "class C { void M() { if (true { } } }"),
        ("missing method paren", "class C { void M( { } }"),
        ("missing expression", "class C { void M() { int value = ; } }"),
        (
            "separate missing and error methods",
            "class C { void First( { } void M() { int value = ; } }",
        ),
    ];

    for (name, source) in samples {
        let network = LinkNetwork::parse(source, "C#", ParseConfiguration::default());
        let report = network.verify_full_match(None);
        println!("{name}: clean={}", report.is_clean());
        for issue in report.issues() {
            let link = network.link(issue.link_id()).expect("issue link exists");
            println!(
                "  issue {:?} id={} type={:?} term={:?} span={:?} flags={:?}",
                issue.kind(),
                issue.link_id(),
                link.metadata().link_type(),
                link.metadata().term(),
                link.metadata().span().map(|span| span.byte_range()),
                link.metadata().flags(),
            );
        }
        for link in network.links() {
            if link.metadata().link_type() == Some(LinkType::Syntax)
                && (link.metadata().flags().is_error()
                    || link.metadata().flags().has_error()
                    || link.metadata().flags().is_missing())
            {
                println!(
                    "  syntax id={} term={:?} span={:?} flags={:?}",
                    link.id(),
                    link.metadata().term(),
                    link.metadata().span().map(|span| span.byte_range()),
                    link.metadata().flags(),
                );
            }
        }
    }
}
