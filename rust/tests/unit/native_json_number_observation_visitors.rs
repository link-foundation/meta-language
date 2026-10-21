// PRIVATE GENERATED PROPOSAL; no concrete sink or native execution.
// One ordered borrowed visitor drives CountSink(Node=()) and DirectSink(Node=LinkId).
// Recursive control flow below is NOT an arbitrary-depth resource proof.
trait OutcomeWireSink: Sized {
    type Node;
    type Object;
    type Array;
    fn begin_object(&mut self, depth: usize) -> Result<Self::Object, String>;
    fn object_field<F>(
        &mut self,
        object: &mut Self::Object,
        key: &str,
        visit: F,
    ) -> Result<(), String>
    where
        F: FnOnce(&mut Self, usize) -> Result<Self::Node, String>;
    fn end_object(&mut self, object: Self::Object) -> Result<Self::Node, String>;
    fn begin_array(&mut self, count: usize, depth: usize) -> Result<Self::Array, String>;
    fn array_item<F>(&mut self, array: &mut Self::Array, visit: F) -> Result<(), String>
    where
        F: FnOnce(&mut Self, usize) -> Result<Self::Node, String>;
    fn end_array(&mut self, array: Self::Array) -> Result<Self::Node, String>;
    fn null(&mut self, depth: usize) -> Result<Self::Node, String>;
    fn boolean(&mut self, value: bool, depth: usize) -> Result<Self::Node, String>;
    fn string(&mut self, value: &str, depth: usize) -> Result<Self::Node, String>;
    fn usize(&mut self, value: usize, depth: usize) -> Result<Self::Node, String>;
    fn i64(&mut self, value: i64, depth: usize) -> Result<Self::Node, String>;
    fn json_number(
        &mut self,
        value: &serde_json::Number,
        depth: usize,
    ) -> Result<Self::Node, String>;
    fn native_json_number(
        &mut self,
        value: &serde_json::Number,
        depth: usize,
    ) -> Result<Self::Node, String> {
        self.json_number(value, depth)
    }
}

fn wire_parse_outcome<S: OutcomeWireSink>(
    value: &meta_language::ParseOutcome,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    let meta_language::ParseOutcome {
        ok,
        tree,
        ambiguities,
        rejection,
        source_planes,
    } = value;
    let mut object = sink.begin_object(depth)?;
    sink.object_field(&mut object, "ok", |sink, depth| sink.boolean(*ok, depth))?;
    sink.object_field(&mut object, "tree", |sink, depth| match tree {
        Some(item) => wire_syntax_tree(item, sink, depth),
        None => sink.null(depth),
    })?;
    sink.object_field(&mut object, "ambiguities", |sink, depth| {
        let mut array = sink.begin_array(ambiguities.len(), depth)?;
        for item in ambiguities {
            sink.array_item(&mut array, |sink, depth| wire_ambiguity(item, sink, depth))?;
        }
        sink.end_array(array)
    })?;
    sink.object_field(&mut object, "rejection", |sink, depth| match rejection {
        Some(item) => wire_parse_rejection(item, sink, depth),
        None => sink.null(depth),
    })?;
    if let Some(item) = source_planes {
        sink.object_field(&mut object, "sourcePlanes", |sink, depth| {
            let mut array = sink.begin_array(item.len(), depth)?;
            for item in item {
                sink.array_item(&mut array, |sink, depth| wire_json(item, sink, depth))?;
            }
            sink.end_array(array)
        })?;
    }
    sink.end_object(object)
}

fn wire_parse_rejection<S: OutcomeWireSink>(
    value: &meta_language::ParseRejection,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    let meta_language::ParseRejection {
        diagnostics,
        offset_semantics,
        reason,
        offset,
        line,
        column,
        expected,
        limit,
    } = value;
    let mut object = sink.begin_object(depth)?;
    sink.object_field(&mut object, "reason", |sink, depth| {
        sink.string(reason, depth)
    })?;
    if let Some(item) = offset {
        sink.object_field(&mut object, "offset", |sink, depth| {
            sink.usize(*item, depth)
        })?;
    }
    if let Some(item) = line {
        sink.object_field(&mut object, "line", |sink, depth| sink.usize(*item, depth))?;
    }
    if let Some(item) = column {
        sink.object_field(&mut object, "column", |sink, depth| {
            sink.usize(*item, depth)
        })?;
    }
    if let Some(item) = expected {
        sink.object_field(&mut object, "expected", |sink, depth| {
            let mut array = sink.begin_array(item.len(), depth)?;
            for item in item {
                sink.array_item(&mut array, |sink, depth| sink.string(item, depth))?;
            }
            sink.end_array(array)
        })?;
    }
    if let Some(item) = limit {
        sink.object_field(&mut object, "limit", |sink, depth| sink.usize(*item, depth))?;
    }
    if let Some(item) = diagnostics {
        sink.object_field(&mut object, "diagnostics", |sink, depth| {
            let mut array = sink.begin_array(item.len(), depth)?;
            for item in item {
                sink.array_item(&mut array, |sink, depth| wire_json(item, sink, depth))?;
            }
            sink.end_array(array)
        })?;
    }
    if let Some(item) = offset_semantics {
        sink.object_field(&mut object, "offsetSemantics", |sink, depth| {
            sink.string(item, depth)
        })?;
    }
    sink.end_object(object)
}

fn wire_ambiguity<S: OutcomeWireSink>(
    value: &meta_language::Ambiguity,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    let meta_language::Ambiguity {
        rule,
        start,
        end,
        languages,
        source,
        source_start,
        source_end,
        source_route,
        offset_semantics,
    } = value;
    let mut object = sink.begin_object(depth)?;
    sink.object_field(&mut object, "rule", |sink, depth| sink.string(rule, depth))?;
    sink.object_field(&mut object, "start", |sink, depth| {
        sink.usize(*start, depth)
    })?;
    sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
    if let Some(item) = languages {
        sink.object_field(&mut object, "languages", |sink, depth| {
            let mut array = sink.begin_array(item.len(), depth)?;
            for item in item {
                sink.array_item(&mut array, |sink, depth| sink.string(item, depth))?;
            }
            sink.end_array(array)
        })?;
    }
    if let Some(item) = source {
        sink.object_field(&mut object, "source", |sink, depth| {
            sink.usize(*item, depth)
        })?;
    }
    if let Some(item) = source_start {
        sink.object_field(&mut object, "sourceStart", |sink, depth| {
            sink.usize(*item, depth)
        })?;
    }
    if let Some(item) = source_end {
        sink.object_field(&mut object, "sourceEnd", |sink, depth| {
            sink.usize(*item, depth)
        })?;
    }
    if let Some(item) = offset_semantics {
        sink.object_field(&mut object, "offsetSemantics", |sink, depth| {
            sink.string(item, depth)
        })?;
    }
    if let Some(item) = source_route {
        sink.object_field(&mut object, "sourceRoute", |sink, depth| {
            let mut array = sink.begin_array(item.len(), depth)?;
            for item in item {
                sink.array_item(&mut array, |sink, depth| wire_json(item, sink, depth))?;
            }
            sink.end_array(array)
        })?;
    }
    sink.end_object(object)
}

fn wire_grammar_selection_alternative<S: OutcomeWireSink>(
    value: &meta_language::GrammarSelectionAlternative,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    let meta_language::GrammarSelectionAlternative {
        language,
        root,
        provenance,
    } = value;
    let mut object = sink.begin_object(depth)?;
    sink.object_field(&mut object, "language", |sink, depth| {
        sink.string(language, depth)
    })?;
    sink.object_field(&mut object, "root", |sink, depth| {
        wire_syntax_tree(root.as_ref(), sink, depth)
    })?;
    sink.object_field(&mut object, "provenance", |sink, depth| {
        wire_json(provenance, sink, depth)
    })?;
    sink.end_object(object)
}

fn wire_grammar_selection<S: OutcomeWireSink>(
    value: &meta_language::GrammarSelection,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    let meta_language::GrammarSelection {
        mode,
        mapping,
        start,
        end,
        candidates,
        provenance,
        alternatives,
        ambiguous,
        source_mapping,
    } = value;
    let mut object = sink.begin_object(depth)?;
    sink.object_field(&mut object, "mode", |sink, depth| sink.string(mode, depth))?;
    sink.object_field(&mut object, "mapping", |sink, depth| {
        sink.string(mapping, depth)
    })?;
    sink.object_field(&mut object, "start", |sink, depth| {
        sink.usize(*start, depth)
    })?;
    sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
    sink.object_field(&mut object, "candidates", |sink, depth| {
        let mut array = sink.begin_array(candidates.len(), depth)?;
        for item in candidates {
            sink.array_item(&mut array, |sink, depth| wire_json(item, sink, depth))?;
        }
        sink.end_array(array)
    })?;
    sink.object_field(&mut object, "provenance", |sink, depth| {
        wire_json(provenance, sink, depth)
    })?;
    sink.object_field(&mut object, "alternatives", |sink, depth| {
        let mut array = sink.begin_array(alternatives.len(), depth)?;
        for item in alternatives {
            sink.array_item(&mut array, |sink, depth| {
                wire_grammar_selection_alternative(item, sink, depth)
            })?;
        }
        sink.end_array(array)
    })?;
    if let Some(item) = source_mapping {
        sink.object_field(&mut object, "sourceMapping", |sink, depth| {
            wire_json(item, sink, depth)
        })?;
    }
    sink.object_field(&mut object, "ambiguous", |sink, depth| {
        sink.boolean(*ambiguous, depth)
    })?;
    sink.end_object(object)
}

fn wire_syntax_tree<S: OutcomeWireSink>(
    value: &meta_language::SyntaxTree,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    match value {
        meta_language::SyntaxTree::Node {
            kind,
            field,
            trivia,
            start,
            end,
            children,
            attributes,
        } => {
            let mut object = sink.begin_object(depth)?;
            sink.object_field(&mut object, "type", |sink, depth| {
                sink.string("node", depth)
            })?;
            sink.object_field(&mut object, "kind", |sink, depth| sink.string(kind, depth))?;
            if let Some(item) = field {
                sink.object_field(&mut object, "field", |sink, depth| sink.string(item, depth))?;
            }
            if *trivia {
                sink.object_field(&mut object, "trivia", |sink, depth| {
                    sink.boolean(*trivia, depth)
                })?;
            }
            sink.object_field(&mut object, "start", |sink, depth| {
                sink.usize(*start, depth)
            })?;
            sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
            sink.object_field(&mut object, "children", |sink, depth| {
                let mut array = sink.begin_array(children.len(), depth)?;
                for item in children {
                    sink.array_item(&mut array, |sink, depth| {
                        wire_syntax_tree(item, sink, depth)
                    })?;
                }
                sink.end_array(array)
            })?;
            if let Some(item) = attributes {
                sink.object_field(&mut object, "attributes", |sink, depth| {
                    wire_syntax_attributes(item, sink, depth)
                })?;
            }
            sink.end_object(object)
        }
        meta_language::SyntaxTree::Token {
            kind,
            field,
            trivia,
            start,
            end,
            attributes,
            text,
            lexical,
        } => {
            let mut object = sink.begin_object(depth)?;
            sink.object_field(&mut object, "type", |sink, depth| {
                sink.string("token", depth)
            })?;
            sink.object_field(&mut object, "kind", |sink, depth| match kind {
                Some(item) => sink.string(item, depth),
                None => sink.null(depth),
            })?;
            if let Some(item) = field {
                sink.object_field(&mut object, "field", |sink, depth| sink.string(item, depth))?;
            }
            if *trivia {
                sink.object_field(&mut object, "trivia", |sink, depth| {
                    sink.boolean(*trivia, depth)
                })?;
            }
            sink.object_field(&mut object, "start", |sink, depth| {
                sink.usize(*start, depth)
            })?;
            sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
            if let Some(item) = attributes {
                sink.object_field(&mut object, "attributes", |sink, depth| {
                    wire_syntax_attributes(item, sink, depth)
                })?;
            }
            if let Some(item) = lexical {
                sink.object_field(&mut object, "lexical", |sink, depth| {
                    wire_json(item, sink, depth)
                })?;
            }
            wire_leaf_text(text, sink, &mut object)?;
            sink.end_object(object)
        }
        meta_language::SyntaxTree::Error {
            start,
            end,
            text,
            reason,
        } => {
            let mut object = sink.begin_object(depth)?;
            sink.object_field(&mut object, "type", |sink, depth| {
                sink.string("error", depth)
            })?;
            sink.object_field(&mut object, "start", |sink, depth| {
                sink.usize(*start, depth)
            })?;
            sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
            wire_leaf_text(text, sink, &mut object)?;
            if let Some(item) = reason {
                sink.object_field(&mut object, "reason", |sink, depth| {
                    sink.string(item, depth)
                })?;
            }
            sink.end_object(object)
        }
        meta_language::SyntaxTree::Missing {
            kind,
            start,
            end,
            literal,
        } => {
            let mut object = sink.begin_object(depth)?;
            sink.object_field(&mut object, "type", |sink, depth| {
                sink.string("missing", depth)
            })?;
            sink.object_field(&mut object, "kind", |sink, depth| match kind {
                Some(item) => sink.string(item, depth),
                None => sink.null(depth),
            })?;
            sink.object_field(&mut object, "start", |sink, depth| {
                sink.usize(*start, depth)
            })?;
            sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
            if *literal {
                sink.object_field(&mut object, "literal", |sink, depth| {
                    sink.boolean(*literal, depth)
                })?;
            }
            sink.end_object(object)
        }
        meta_language::SyntaxTree::Embed {
            language,
            start,
            end,
            root,
            selection,
            source_children,
        } => {
            let mut object = sink.begin_object(depth)?;
            sink.object_field(&mut object, "type", |sink, depth| {
                sink.string("embed", depth)
            })?;
            sink.object_field(&mut object, "language", |sink, depth| {
                sink.string(language, depth)
            })?;
            sink.object_field(&mut object, "start", |sink, depth| {
                sink.usize(*start, depth)
            })?;
            sink.object_field(&mut object, "end", |sink, depth| sink.usize(*end, depth))?;
            sink.object_field(&mut object, "root", |sink, depth| {
                wire_syntax_tree(root.as_ref(), sink, depth)
            })?;
            if let Some(item) = source_children {
                sink.object_field(&mut object, "sourceChildren", |sink, depth| {
                    let mut array = sink.begin_array(item.len(), depth)?;
                    for item in item {
                        sink.array_item(&mut array, |sink, depth| {
                            wire_syntax_tree(item, sink, depth)
                        })?;
                    }
                    sink.end_array(array)
                })?;
            }
            if let Some(item) = selection {
                sink.object_field(&mut object, "selection", |sink, depth| {
                    wire_grammar_selection(item.as_ref(), sink, depth)
                })?;
            }
            sink.end_object(object)
        }
    }
}

fn wire_leaf_text<S: OutcomeWireSink>(
    value: &meta_language::LeafText,
    sink: &mut S,
    object: &mut S::Object,
) -> Result<(), String> {
    match value {
        meta_language::LeafText::Text(payload) => {
            sink.object_field(object, "text", |sink, depth| sink.string(payload, depth))?;
        }
        meta_language::LeafText::Hex(payload) => {
            sink.object_field(object, "text", OutcomeWireSink::null)?;
            sink.object_field(object, "hex", |sink, depth| sink.string(payload, depth))?;
        }
    }
    Ok(())
}

fn wire_operation_value<S: OutcomeWireSink>(
    value: &meta_language::OperationValue,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    match value {
        meta_language::OperationValue::Integer(payload) => sink.i64(*payload, depth),
        meta_language::OperationValue::Text(payload) => sink.string(payload, depth),
    }
}

fn wire_syntax_attributes<S: OutcomeWireSink>(
    value: &meta_language::SyntaxAttributes,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    let mut object = sink.begin_object(depth)?;
    for (key, item) in value {
        sink.object_field(&mut object, key, |sink, depth| {
            wire_operation_value(item, sink, depth)
        })?;
    }
    sink.end_object(object)
}

fn wire_json<S: OutcomeWireSink>(
    value: &serde_json::Value,
    sink: &mut S,
    depth: usize,
) -> Result<S::Node, String> {
    match value {
        serde_json::Value::Null => sink.null(depth),
        serde_json::Value::Bool(value) => sink.boolean(*value, depth),
        serde_json::Value::Number(value) => sink.native_json_number(value, depth),
        serde_json::Value::String(value) => sink.string(value, depth),
        serde_json::Value::Array(values) => {
            let mut array = sink.begin_array(values.len(), depth)?;
            for item in values {
                sink.array_item(&mut array, |sink, depth| wire_json(item, sink, depth))?;
            }
            sink.end_array(array)
        }
        serde_json::Value::Object(values) => {
            let mut object = sink.begin_object(depth)?;
            for (key, item) in values {
                sink.object_field(&mut object, key, |sink, depth| wire_json(item, sink, depth))?;
            }
            sink.end_object(object)
        }
    }
}
