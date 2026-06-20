use meta_language::{LinkNetwork, ParseConfiguration, VerificationIssueKind};
fn report(language: &str, source: &str) {
    let n = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let r = n.verify_full_match(None);
    let has_diag = r.issues().iter().any(|i| i.kind()==VerificationIssueKind::ErrorLink || i.kind()==VerificationIssueKind::MissingLink);
    let flagged = n.links().any(|l| l.metadata().flags().has_error() || l.metadata().flags().is_missing());
    println!("{language:>8}: round_trip={} clean={} has_diag={has_diag} flagged={flagged}", n.reconstruct_text()==source, r.is_clean());
}
fn main() {
    report("php", "<?php\nfunction greet($name {\n    return $name;\n");
    report("swift", "func greet(_ name: String -> String {\n    return name\n");
    report("kotlin", "fun greet(name: String {\n    return name\n");
    report("scala", "object Demo {\n  def greet(name: String = s\"$name\"\n");
    report("lua", "local function greet(name\n  return name\n");
}
