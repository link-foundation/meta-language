use crate::link_network::LinkType;

pub struct SelfDescriptionRoot {
    pub term: &'static str,
    pub link_type: LinkType,
    pub references: &'static [&'static str],
}

pub const SELF_DESCRIPTION_ROOTS: &[SelfDescriptionRoot] = &[
    SelfDescriptionRoot {
        term: "link",
        link_type: LinkType::Link,
        references: &["reference", "reference"],
    },
    SelfDescriptionRoot {
        term: "reference",
        link_type: LinkType::Reference,
        references: &["link", "link"],
    },
    SelfDescriptionRoot {
        term: "relation link",
        link_type: LinkType::Relation,
        references: &["link", "reference"],
    },
    SelfDescriptionRoot {
        term: "language",
        link_type: LinkType::Language,
        references: &["grammar", "concept"],
    },
    SelfDescriptionRoot {
        term: "grammar",
        link_type: LinkType::Grammar,
        references: &["language", "relation link"],
    },
    SelfDescriptionRoot {
        term: "type",
        link_type: LinkType::Type,
        references: &["Type", "concept"],
    },
    SelfDescriptionRoot {
        term: "Type",
        link_type: LinkType::Type,
        references: &["Type", "Type"],
    },
    SelfDescriptionRoot {
        term: "concept",
        link_type: LinkType::Concept,
        references: &["link", "language"],
    },
    SelfDescriptionRoot {
        term: "point",
        link_type: LinkType::Concept,
        references: &["point", "point"],
    },
    SelfDescriptionRoot {
        term: "field",
        link_type: LinkType::Field,
        references: &["relation link", "reference"],
    },
    SelfDescriptionRoot {
        term: "trivia",
        link_type: LinkType::Trivia,
        references: &["region", "link"],
    },
    SelfDescriptionRoot {
        term: "region",
        link_type: LinkType::Region,
        references: &["language", "link"],
    },
    SelfDescriptionRoot {
        term: "object",
        link_type: LinkType::Object,
        references: &["link", "reference"],
    },
];

pub fn definition_expression(term: &str, references: &[&str]) -> String {
    let mut expression = String::from("(");
    expression.push_str(term);
    expression.push(':');
    for reference in references {
        expression.push(' ');
        expression.push_str(reference);
    }
    expression.push(')');
    expression
}
