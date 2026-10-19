//! Actual source and lexical facts for the shared tag-owner decisions.

use super::{External, JavaScriptParser, ROOT, SExpr, SNode, Token, Type};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::javascript_tag_context_rules::{
    accept_java_script_tag_header_fact, read_java_script_tag_declared_owner,
    read_java_script_tag_receiver_form,
};

#[derive(Clone)]
pub(super) enum TagReceiverFact {
    Declared(Option<Type>),
    Header { token: Token, asynchronous: bool },
}

impl JavaScriptParser {
    fn remember_tag_function(&mut self, name: String, token: Token, asynchronous: bool) {
        use std::collections::hash_map::Entry;
        match self.tag_function_results.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(None);
            }
            Entry::Vacant(entry) => {
                entry.insert(Some(TagReceiverFact::Header {
                    token,
                    asynchronous,
                }));
            }
        }
    }

    pub(super) fn read_tag_receiver_fact(&self, node: &SExpr) -> Option<TagReceiverFact> {
        let target = match &node.node {
            SNode::App { func, .. } => func.as_ref(),
            _ => node,
        };
        let path = match &target.node {
            SNode::Name { path } | SNode::FunctionValue { path } => path.as_slice(),
            _ => &[],
        };
        let function_kind = match &node.node {
            SNode::App { .. } => target.node.kind(),
            _ => "",
        };
        let form = read_java_script_tag_receiver_form(node.node.kind(), function_kind, path, ROOT);
        match number_index(form) {
            Some(1) => path
                .get(1)
                .and_then(|name| self.tag_function_results.get(name))
                .cloned()
                .flatten(),
            Some(2) => path.first().and_then(|name| self.lexical.tag_fact(name)),
            _ => None,
        }
    }

    pub(super) fn read_tag_receiver_owner(&self, node: &SExpr) -> String {
        let Some(fact) = self.read_tag_receiver_fact(node) else {
            return String::new();
        };
        let (mut ty, header, asynchronous) = match fact {
            TagReceiverFact::Declared(ty) => (ty, None, false),
            TagReceiverFact::Header {
                token,
                asynchronous,
            } => (None, Some(token), asynchronous),
        };
        // Only speculative annotation lookup maps TranslationError to unavailable.
        // Ordinary declaration parsing retains its existing error propagation.
        if accept_java_script_tag_header_fact(ty.is_some(), header.is_some(), asynchronous)
            && let Some(doc) = header
                .as_ref()
                .and_then(|token| self.jsdoc_for(token).ok().flatten())
            && let Some(returns) = doc.returns
        {
            ty = self.ty(&returns, doc.range).ok();
        }
        let kind = ty.as_ref().map_or("", Type::kind);
        let path = match ty.as_ref() {
            Some(Type::Named { path, .. }) => path.as_slice(),
            _ => &[],
        };
        let name = match ty.as_ref() {
            Some(Type::Data { name }) => name.as_str(),
            _ => "",
        };
        read_java_script_tag_declared_owner(kind, path, name, ROOT)
    }
}

impl JavaScriptParser {
    /// Read module bindings before function scopes are entered, as the JavaScript author does.
    pub(super) fn prescan_product_bindings(&mut self) {
        self.function_names
            .extend(self.externals.iter().filter_map(|item| match item {
                super::External::Function { name, .. } => Some(name.clone()),
                super::External::Constant { .. } => None,
            }));
        for item in &self.externals {
            if let External::Function { name, ret, .. } = item {
                self.tag_function_results.insert(
                    name.clone(),
                    Some(TagReceiverFact::Declared(Some(ret.clone()))),
                );
            }
        }
        let mut headers = Vec::new();
        let tokens = &self.cursor.tokens;
        let mut depth = 0usize;
        for index in self.cursor.index..tokens.len() {
            let token = &tokens[index];
            if depth == 0 && token.kind == super::TokenKind::Identifier {
                if token.value == "import" {
                    let mut braces = 0usize;
                    for at in index + 1..tokens.len() {
                        let imported = &tokens[at];
                        if braces == 0
                            && (imported.kind == super::TokenKind::String
                                || matches!(imported.value.as_str(), "from" | ";"))
                        {
                            break;
                        }
                        let previous = tokens[at - 1].value.as_str();
                        if imported.kind == super::TokenKind::Identifier
                            && (matches!(previous, "import" | "as")
                                || (matches!(previous, "{" | ",")
                                    && tokens.get(at + 1).is_none_or(|next| next.value != "as")))
                        {
                            self.global_binding_names.insert(imported.value.clone());
                        }
                        match imported.value.as_str() {
                            "{" => braces += 1,
                            "}" => braces = braces.saturating_sub(1),
                            _ => {}
                        }
                    }
                }
                let named_function = token.value == "function"
                    && tokens
                        .get(index + 1)
                        .is_some_and(|next| next.kind == super::TokenKind::Identifier);
                let constant_function = token.value == "const"
                    && tokens
                        .get(index + 1)
                        .is_some_and(|next| next.kind == super::TokenKind::Identifier)
                    && tokens.get(index + 2).is_some_and(|next| next.value == "=")
                    && self.starts_function(index + 3);
                if named_function || constant_function {
                    self.function_names.insert(tokens[index + 1].value.clone());
                }
                if named_function {
                    let asynchronous = index
                        .checked_sub(1)
                        .and_then(|at| tokens.get(at))
                        .is_some_and(|previous| previous.value == "async");
                    let mut head = index - usize::from(asynchronous);
                    if head
                        .checked_sub(1)
                        .and_then(|at| tokens.get(at))
                        .is_some_and(|previous| previous.value == "export")
                    {
                        head -= 1;
                    }
                    let expression = index
                        .checked_sub(1)
                        .and_then(|at| tokens.get(at))
                        .is_some_and(|previous| previous.value == "=")
                        || asynchronous
                            && index
                                .checked_sub(2)
                                .and_then(|at| tokens.get(at))
                                .is_some_and(|previous| previous.value == "=");
                    if !expression {
                        headers.push((
                            tokens[index + 1].value.clone(),
                            tokens[head].clone(),
                            asynchronous,
                        ));
                    }
                }
                if constant_function {
                    let asynchronous = tokens
                        .get(index + 3)
                        .is_some_and(|next| next.value == "async")
                        && tokens.get(index + 4).is_none_or(|next| next.value != "=>");
                    let head = if index
                        .checked_sub(1)
                        .and_then(|at| tokens.get(at))
                        .is_some_and(|previous| previous.value == "export")
                    {
                        index - 1
                    } else {
                        index
                    };
                    headers.push((
                        tokens[index + 1].value.clone(),
                        tokens[head].clone(),
                        asynchronous,
                    ));
                }
            }
            if token.kind == super::TokenKind::Punct {
                match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => depth = depth.saturating_sub(1),
                    _ => {}
                }
            }
        }
        for (name, token, asynchronous) in headers {
            self.remember_tag_function(name, token, asynchronous);
        }
        self.global_binding_names
            .extend(self.function_names.iter().cloned());
        self.global_binding_names
            .extend(self.externals.iter().map(|item| item.name().to_owned()));
        self.global_binding_names
            .extend(self.literal_bindings.keys().cloned());
    }
}
