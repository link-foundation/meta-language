//! ```cargo
//! [dependencies]
//! meta-language = { path = ".." }
//! ```

use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn main() {
    let source = "<script>const x = 1;</script><style>.x { color: red; }</style><p style=\"color: blue\">text</p>\n";
    let network = LinkNetwork::parse(source, "HTML", ParseConfiguration::default());
    let report = network.verify_full_match(None);

    println!("clean: {}", report.is_clean());
    for issue in report.issues() {
        let link = network.link(issue.link_id()).expect("issue link exists");
        println!(
            "issue {:?} id={} type={:?} language={:?} term={:?} span={:?} flags={:?} refs={:?}",
            issue.kind(),
            issue.link_id(),
            link.metadata().link_type(),
            link.metadata().language(),
            link.metadata().term(),
            link.metadata().span().map(|span| span.byte_range()),
            link.metadata().flags(),
            link.references(),
        );
    }

    for link in network.links() {
        if link.metadata().link_type() == Some(LinkType::Region)
            || link.metadata().flags().is_error()
            || link.metadata().flags().has_error()
            || link.metadata().flags().is_missing()
        {
            println!(
                "link id={} type={:?} language={:?} term={:?} span={:?} flags={:?} refs={:?}",
                link.id(),
                link.metadata().link_type(),
                link.metadata().language(),
                link.metadata().term(),
                link.metadata().span().map(|span| span.byte_range()),
                link.metadata().flags(),
                link.references(),
            );
        }
    }
}
