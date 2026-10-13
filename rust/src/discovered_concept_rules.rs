// Generated automatically from js/src/discovered-concept-rules.js.
// Emitter styles are recorded in parity/self-translation/discovered-concept-rules-decorators.lino.
// Do not edit by hand.
/// Read a shared discovered concept graph decision.
#[must_use]
pub fn accept_discovered_concept_shape(term: &str, arity: &str, has_definition: bool) -> bool {
    if term == "discovered concept graph" {
        arity == "4" && has_definition
    } else if term == "discovered concept sequence" {
        !has_definition
    } else if term == "discovered concept candidate" {
        arity == "4" && has_definition
    } else if term == "discovered concept identity" {
        has_definition
    } else if term == "discovered formal definition" {
        arity == "3" && has_definition
    } else {
        false
    }
}
