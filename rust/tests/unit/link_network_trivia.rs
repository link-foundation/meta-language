use meta_language::{LinkId, LinkNetwork, LinkType, ParseConfiguration, TriviaAttachmentPolicy};

#[test]
fn extra_tokens_get_trivia_links_owned_by_the_syntax_link_above_them_per_policy() {
    let trivia_of = |source: &str, language: &str, policy: TriviaAttachmentPolicy| {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::new(policy));
        let describe = |id: &LinkId| {
            let metadata = network
                .link(*id)
                .expect("trivia reference exists")
                .metadata();
            format!(
                "{:?}:{}",
                metadata.link_type().expect("typed reference"),
                metadata.term().unwrap_or_default()
            )
        };
        network
            .links()
            .filter(|link| {
                link.metadata().link_type() == Some(LinkType::Trivia)
                    && link.metadata().span().is_some()
            })
            .map(|link| {
                std::iter::once(link.metadata().term().unwrap_or_default().to_owned())
                    .chain(link.references().iter().map(describe))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    let containment = vec!["containment trivia", "Syntax:whitespace", "Token: "];
    let token = vec!["token trivia", "Token: "];
    assert_eq!(
        trivia_of("a b", "txt", TriviaAttachmentPolicy::Both),
        vec![containment.clone(), token.clone()]
    );
    assert_eq!(
        trivia_of("a b", "txt", TriviaAttachmentPolicy::ContainmentLink),
        vec![containment]
    );
    assert_eq!(
        trivia_of("a b", "txt", TriviaAttachmentPolicy::TokenLink),
        vec![token]
    );
    // A grammar extra such as a comment is owned by its own leaf Syntax link,
    // and whitespace between grammar nodes by the node enclosing it.
    let mut javascript = trivia_of(
        "x; // note\n",
        "JavaScript",
        TriviaAttachmentPolicy::ContainmentLink,
    );
    javascript.sort();
    assert_eq!(
        javascript,
        vec![
            vec!["containment trivia", "Syntax:comment", "Token:// note"],
            vec!["containment trivia", "Syntax:program", "Token:\n"],
            vec!["containment trivia", "Syntax:program", "Token: "],
        ]
    );
}
