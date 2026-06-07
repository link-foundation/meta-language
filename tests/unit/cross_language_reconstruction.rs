use meta_language::{
    FormalizationLevel, LinkNetwork, LinkType, NaturalizationDirection, ParseConfiguration,
};

#[test]
fn parsed_hawaii_sentence_reconstructs_between_english_and_russian() {
    let configuration = ParseConfiguration::default();
    let english = "Hawaii is a state.\n";
    let russian = "Гавайи это штат.\n";

    let english_network = LinkNetwork::parse(english, "English", configuration);
    assert_eq!(
        english_network.reconstruct_text_as("English", configuration),
        english
    );
    assert_eq!(
        english_network.reconstruct_text_as("Russian", configuration),
        russian
    );

    let russian_network = LinkNetwork::parse(russian, "Russian", configuration);
    assert_eq!(
        russian_network.reconstruct_text_as("Russian", configuration),
        russian
    );
    assert_eq!(
        russian_network.reconstruct_text_as("English", configuration),
        english
    );

    assert!(english_network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Semantic)
            && link.metadata().term() == Some("proposition:statehood")
    }));
}

#[test]
fn formalization_level_changes_cross_language_reconstruction_output() {
    let network = LinkNetwork::parse(
        "Hawaii is a state.\n",
        "English",
        ParseConfiguration::default(),
    );

    assert_eq!(
        network.reconstruct_text_as("Russian", ParseConfiguration::default()),
        "Гавайи это штат.\n"
    );
    assert_eq!(
        network.reconstruct_text_as(
            "Russian",
            ParseConfiguration::default()
                .with_naturalization_direction(NaturalizationDirection::Formalize)
        ),
        "statehood(Гавайи, штат)\n"
    );
    assert_eq!(
        network.reconstruct_text_as(
            "Russian",
            ParseConfiguration::default().with_formalization_level(FormalizationLevel::Lexical)
        ),
        "statehood(Гавайи, штат)\n"
    );
    assert_eq!(
        network.reconstruct_text_as(
            "Russian",
            ParseConfiguration::default().with_formalization_level(FormalizationLevel::Concept)
        ),
        "statehood(Q782, Q35657)\n"
    );
    assert_eq!(
        network.reconstruct_text_as(
            "Russian",
            ParseConfiguration::default().with_formalization_level(FormalizationLevel::Logical)
        ),
        "(proposition: statehood (subject: Q782) (object: Q35657) (truth: true))\n"
    );
}

#[test]
fn parse_configuration_exposes_deformalization_knobs() {
    let configuration = ParseConfiguration::default()
        .with_formalization_level(FormalizationLevel::Concept)
        .with_naturalization_direction(NaturalizationDirection::Formalize);

    assert_eq!(
        configuration.formalization_level(),
        FormalizationLevel::Concept
    );
    assert_eq!(
        configuration.naturalization_direction(),
        NaturalizationDirection::Formalize
    );
}
