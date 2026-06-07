//! ```cargo
//! [dependencies]
//! meta-language = { path = ".." }
//! ```

use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn main() {
    let samples = [
        ("open parameter list", "Module Program\n    Sub Main(\nEnd Module\n"),
        (
            "bad statement and open parameter list",
            "Module Program\n    Sub Main(\n        If Then\nEnd Module\n",
        ),
        (
            "separate missing and error methods",
            "Module Program\n    Sub First(\n    End Sub\n    Sub Main()\n        If Then\n    End Sub\nEnd Module\n",
        ),
        (
            "missing end if",
            "Module Program\n    Sub Main()\n        If True Then\n    End Sub\nEnd Module\n",
        ),
        (
            "bad statement",
            "Module Program\n    Sub Main()\n        If Then\n    End Sub\nEnd Module\n",
        ),
        (
            "missing expression",
            "Module Program\n    Sub Main()\n        Dim value As Integer =\n    End Sub\nEnd Module\n",
        ),
        (
            "unterminated string",
            "Module Program\n    Sub Main()\n        Dim value = \"open\n    End Sub\nEnd Module\n",
        ),
    ];

    for (name, source) in samples {
        let network = LinkNetwork::parse(source, "Visual Basic", ParseConfiguration::default());
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
