//! Parses files with a native Links Notation grammar file and prints the
//! s-expression js/experiments/tree-sitter-native-compare.mjs prints.
//!   cargo run --release --example native_grammar_experiment -- GRAMMAR.lino FILE...
use meta_language::{
    FeatureParseOptions, SyntaxTree, compile_feature_grammar, parse_grammar_links,
};

fn trivia(node: &SyntaxTree) -> bool {
    matches!(node, SyntaxTree::Token { trivia: true, .. })
}

fn hoist(node: &SyntaxTree) -> Vec<SyntaxTree> {
    let SyntaxTree::Node {
        kind,
        field,
        start,
        end,
        children,
        attributes,
    } = node
    else {
        return vec![node.clone()];
    };
    let mut children: Vec<SyntaxTree> = children.iter().flat_map(hoist).collect();
    let first = children.iter().take_while(|child| trivia(child)).count();
    let rest = children.split_off(first);
    children.push(SyntaxTree::Node {
        kind: kind.clone(),
        field: field.clone(),
        start: *start,
        end: *end,
        children: rest,
        attributes: attributes.clone(),
    });
    children
}

fn hidden(kind: Option<&str>) -> bool {
    kind.is_none_or(|kind| kind.starts_with('_') || kind.starts_with('\''))
}

fn visit(node: &SyntaxTree, out: &mut Vec<String>) {
    let (kind, field, children): (Option<&str>, Option<&str>, &[SyntaxTree]) = match node {
        SyntaxTree::Error { .. } => {
            out.push("(ERROR)".into());
            return;
        }
        SyntaxTree::Missing { kind, .. } => {
            out.push(format!("(MISSING {})", kind.as_deref().unwrap_or("")));
            return;
        }
        SyntaxTree::Node {
            kind,
            field,
            children,
            ..
        } => (Some(kind.as_str()), field.as_deref(), children.as_slice()),
        SyntaxTree::Token { kind, field, .. } => (kind.as_deref(), field.as_deref(), &[]),
        SyntaxTree::Embed { .. } => return,
    };
    let mut inner = Vec::new();
    for child in children {
        visit(child, &mut inner);
    }
    if hidden(kind) {
        out.extend(inner);
        return;
    }
    let field = field.map(|field| format!("{field}: ")).unwrap_or_default();
    let tail = if inner.is_empty() {
        String::new()
    } else {
        format!(" {}", inner.join(" "))
    };
    out.push(format!("{field}({}{tail})", kind.unwrap_or("")));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let text = std::fs::read_to_string(&args[0]).unwrap();
    let started = std::time::Instant::now();
    let grammar = parse_grammar_links(&text).unwrap();
    let parser = compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).unwrap();
    eprintln!("compiled in {} ms", started.elapsed().as_millis());
    let options = FeatureParseOptions {
        error_recovery: Some(true),
        accept_recovery: Some(true),
        ..FeatureParseOptions::default()
    };
    for path in &args[1..] {
        let source = std::fs::read_to_string(path).unwrap();
        let started = std::time::Instant::now();
        let outcome = parser.parse_tree(source.as_bytes(), &options).unwrap();
        let elapsed = started.elapsed().as_millis();
        let mut out = Vec::new();
        if let Some(SyntaxTree::Node {
            kind,
            field,
            start,
            end,
            children,
            attributes,
        }) = outcome.tree
        {
            let root = SyntaxTree::Node {
                kind,
                field,
                start,
                end,
                children: children.iter().flat_map(hoist).collect(),
                attributes,
            };
            visit(&root, &mut out);
        }
        println!("{}", out.join(" "));
        eprintln!("{path}: {elapsed} ms");
    }
}
