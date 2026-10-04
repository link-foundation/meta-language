use meta_language::identify_language;

#[test]
fn identifies_supported_languages_like_the_javascript_runtime() {
    for (text, expected) in [
        ("Hawaii is a state.", Some("English")),
        ("Hawaii est un etat.\n", Some("French")),
        ("Hawaii e um estado.\n", Some("Portuguese")),
        ("Гавайи это штат.", Some("Russian")),
        ("مرحبا.\n", Some("Urdu")),
        ("سلام۔\n", Some("Modern Standard Arabic")),
        ("我喜欢学习。", Some("Mandarin Chinese")),
        ("আমি বাড়ি যাই।", Some("Bengali")),
        ("12345", None),
        ("ok", None),
        ("Ωμέγα", None),
        ("", None),
    ] {
        assert_eq!(identify_language(text), expected, "{text:?}");
    }
}
