use meta_language::{render_pdf_document, BlockNode, InlineNode};
use std::collections::BTreeMap;

fn strong(text: &str) -> InlineNode {
    InlineNode::Wrapped {
        concept: "strong".to_string(),
        attributes: BTreeMap::new(),
        children: vec![InlineNode::Text(text.to_string())],
    }
}

fn main() {
    let doc = meta_language::FormattingDocument {
        blocks: vec![
            BlockNode::Heading {
                level: 1,
                children: vec![InlineNode::Text("Status Report".to_string())],
            },
            BlockNode::Paragraph {
                children: vec![
                    InlineNode::Text("The system is ".to_string()),
                    strong("ready"),
                    InlineNode::Text(" for launch.".to_string()),
                ],
            },
        ],
    };
    let pdf = render_pdf_document(&doc);

    // Emit a Rust string literal for embedding into LANGUAGE_FIXTURES.
    let mut literal = String::new();
    for ch in pdf.chars() {
        match ch {
            '\n' => literal.push_str("\\n"),
            '"' => literal.push_str("\\\""),
            '\\' => literal.push_str("\\\\"),
            other => literal.push(other),
        }
    }
    println!("{literal}");
}
