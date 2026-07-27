use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{Link, LinkId, LinkNetwork, LinkType, ParseConfiguration, QueryMatch};

use super::{canonical_reconstruction_language, TranslationRule, TranslationRuleSet};

pub(super) fn render_roots(
    rule_set: &TranslationRuleSet,
    network: &LinkNetwork,
    target_language: &str,
    configuration: ParseConfiguration,
) -> Option<String> {
    let renderer = TranslationRenderer::new(rule_set, network);
    let roots = renderer.rendering_roots();
    if roots.is_empty()
        || !roots
            .iter()
            .any(|root| renderer.has_template(*root, target_language, configuration))
    {
        return None;
    }

    Some(
        roots
            .into_iter()
            .map(|root| renderer.render(root, target_language, configuration))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

pub(super) fn render_link(
    rule_set: &TranslationRuleSet,
    network: &LinkNetwork,
    link_id: LinkId,
    target_language: &str,
    configuration: ParseConfiguration,
) -> String {
    TranslationRenderer::new(rule_set, network).render(link_id, target_language, configuration)
}

struct ClaimedLink<'a> {
    rule: &'a TranslationRule,
    query_match: QueryMatch,
}

struct TranslationRenderer<'a> {
    rule_set: &'a TranslationRuleSet,
    network: &'a LinkNetwork,
    matches_by_link: BTreeMap<LinkId, ClaimedLink<'a>>,
}

impl<'a> TranslationRenderer<'a> {
    fn new(rule_set: &'a TranslationRuleSet, network: &'a LinkNetwork) -> Self {
        let mut matches_by_link = BTreeMap::new();
        for rule in rule_set.rules() {
            for query_match in network.query_matches(rule.query()) {
                matches_by_link
                    .entry(query_match.link_id())
                    .or_insert(ClaimedLink { rule, query_match });
            }
        }
        Self {
            rule_set,
            network,
            matches_by_link,
        }
    }

    fn rendering_roots(&self) -> Vec<LinkId> {
        let mut matched_children = BTreeSet::new();
        for root_id in self.matches_by_link.keys() {
            let mut pending = self
                .network
                .link(*root_id)
                .map_or_else(Vec::new, |link| link.references().to_vec());
            let mut visited = BTreeSet::new();
            while let Some(link_id) = pending.pop() {
                if link_id == *root_id || !visited.insert(link_id) {
                    continue;
                }
                if self.matches_by_link.contains_key(&link_id) {
                    matched_children.insert(link_id);
                }
                if let Some(link) = self.network.link(link_id) {
                    pending.extend_from_slice(link.references());
                }
            }
        }

        let roots = self
            .matches_by_link
            .keys()
            .filter(|link_id| !matched_children.contains(link_id))
            .copied()
            .collect::<Vec<_>>();
        if roots.is_empty() {
            self.matches_by_link.keys().copied().collect()
        } else {
            roots
        }
    }

    fn has_template(
        &self,
        link_id: LinkId,
        target_language: &str,
        configuration: ParseConfiguration,
    ) -> bool {
        self.matches_by_link.get(&link_id).is_some_and(|claimed| {
            self.template_for(claimed.rule, target_language, configuration)
                .is_some()
        })
    }

    fn render(
        &self,
        link_id: LinkId,
        target_language: &str,
        configuration: ParseConfiguration,
    ) -> String {
        self.render_while_visiting(
            link_id,
            target_language,
            configuration,
            &mut BTreeSet::new(),
        )
    }

    fn render_while_visiting(
        &self,
        link_id: LinkId,
        target_language: &str,
        configuration: ParseConfiguration,
        visiting: &mut BTreeSet<LinkId>,
    ) -> String {
        if !visiting.insert(link_id) {
            return self.render_unclaimed(link_id, target_language);
        }

        let rendered = self.matches_by_link.get(&link_id).map_or_else(
            || self.render_unclaimed(link_id, target_language),
            |claimed| {
                self.template_for(claimed.rule, target_language, configuration)
                    .map_or_else(
                        || self.render_unclaimed(link_id, target_language),
                        |template| {
                            self.expand(
                                template.source(),
                                claimed.rule,
                                &claimed.query_match,
                                target_language,
                                configuration,
                                visiting,
                            )
                        },
                    )
            },
        );
        visiting.remove(&link_id);
        rendered
    }

    fn template_for<'rule>(
        &self,
        rule: &'rule TranslationRule,
        target_language: &str,
        configuration: ParseConfiguration,
    ) -> Option<&'rule super::TranslationTemplate> {
        if configuration.formalization_level() != crate::FormalizationLevel::Natural
            || configuration.naturalization_direction() == crate::NaturalizationDirection::Formalize
        {
            return rule.template_for(target_language, configuration);
        }

        let mut pending = VecDeque::from([target_language]);
        let mut visited = BTreeSet::new();
        while let Some(candidate) = pending.pop_front() {
            if !visited.insert(candidate) {
                continue;
            }
            if let Some(template) = rule.template_for(candidate, configuration) {
                return Some(template);
            }
            if let Some(fallbacks) = self.rule_set.language_fallbacks().get(candidate) {
                pending.extend(fallbacks.iter().map(String::as_str));
            }
        }
        None
    }

    fn resolve(
        &self,
        rule: &TranslationRule,
        query_match: &QueryMatch,
        name: &str,
    ) -> Option<LinkId> {
        if name == "." {
            return Some(query_match.link_id());
        }
        if let Some(link_id) = query_match.captures().first(name) {
            return Some(link_id);
        }
        let reference_index = *rule.reference_captures().get(name)?;
        self.network
            .link(query_match.link_id())?
            .references()
            .get(reference_index)
            .copied()
    }

    fn render_mode(
        &self,
        link_id: LinkId,
        mode: Option<&str>,
        target_language: &str,
        configuration: ParseConfiguration,
        visiting: &mut BTreeSet<LinkId>,
    ) -> String {
        match mode {
            None => self.render_while_visiting(link_id, target_language, configuration, visiting),
            Some("text" | "source") => self.captured_text(link_id, &mut BTreeSet::new()),
            Some("term") => self
                .network
                .link(link_id)
                .and_then(|link| link.metadata().term())
                .map_or_else(|| link_id.to_string(), str::to_string),
            Some("concept") => self.network.link(link_id).map_or_else(String::new, |link| {
                concept_id_for_link(self.network, link)
                    .or_else(|| link.metadata().term())
                    .map_or_else(|| link_id.to_string(), str::to_string)
            }),
            Some("language") => self.render_unclaimed(link_id, target_language),
            Some(context) => self.render_while_visiting(
                link_id,
                &contextual_language(target_language, context),
                configuration,
                visiting,
            ),
        }
    }

    fn expand(
        &self,
        source: &str,
        rule: &TranslationRule,
        query_match: &QueryMatch,
        target_language: &str,
        configuration: ParseConfiguration,
        visiting: &mut BTreeSet<LinkId>,
    ) -> String {
        let mut output = String::new();
        let mut index = 0;
        while index < source.len() {
            let rest = &source[index..];
            if rest.starts_with("{{") {
                output.push('{');
                index += 2;
                continue;
            }
            if rest.starts_with("}}") {
                output.push('}');
                index += 2;
                continue;
            }

            if let Some((matched_len, name)) = parse_conditional(rest) {
                let close_token = format!("{{/{name}}}");
                let body_start = index + matched_len;
                let close = source[body_start..]
                    .find(&close_token)
                    .map(|offset| body_start + offset);
                let body_end = close.unwrap_or(source.len());
                if self.resolve(rule, query_match, name).is_some() {
                    output.push_str(&self.expand(
                        &source[body_start..body_end],
                        rule,
                        query_match,
                        target_language,
                        configuration,
                        visiting,
                    ));
                }
                index = close.map_or(source.len(), |position| position + close_token.len());
                continue;
            }

            if let Some(placeholder) = parse_placeholder(rest) {
                if let Some(target) = self.resolve(rule, query_match, placeholder.name) {
                    let value = if placeholder.variadic {
                        self.network.link(target).map_or_else(String::new, |link| {
                            link.references()
                                .iter()
                                .map(|child| {
                                    self.render_mode(
                                        *child,
                                        placeholder.mode,
                                        target_language,
                                        configuration,
                                        visiting,
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(&decode_separator(placeholder.separator))
                        })
                    } else {
                        self.render_mode(
                            target,
                            placeholder.mode,
                            target_language,
                            configuration,
                            visiting,
                        )
                    };
                    output.push_str(&indent_continuation(&value, current_indent(&output)));
                }
                index += placeholder.matched_len;
                continue;
            }

            let character = rest
                .chars()
                .next()
                .expect("non-empty template remainder must contain a character");
            output.push(character);
            index += character.len_utf8();
        }
        output
    }

    fn render_unclaimed(&self, link_id: LinkId, target_language: &str) -> String {
        let Some(link) = self.network.link(link_id) else {
            return String::new();
        };
        if let Some(rendered) = concept_id_for_link(self.network, link).and_then(|concept| {
            reconstruct_concept_for_language(self.network, concept, target_language)
        }) {
            return rendered.to_string();
        }
        if link.metadata().link_type() == Some(LinkType::Token)
            || link.metadata().link_type() == Some(LinkType::Syntax)
        {
            return self.captured_text(link_id, &mut BTreeSet::new());
        }
        link.metadata().term().map_or_else(
            || self.captured_text(link_id, &mut BTreeSet::new()),
            str::to_string,
        )
    }

    fn captured_text(&self, link_id: LinkId, visiting: &mut BTreeSet<LinkId>) -> String {
        if !visiting.insert(link_id) {
            return String::new();
        }
        let rendered = self.network.link(link_id).map_or_else(String::new, |link| {
            if link.metadata().link_type() == Some(LinkType::Token) {
                return link.metadata().term().unwrap_or_default().to_string();
            }
            link.references()
                .iter()
                .map(|reference| self.captured_text(*reference, visiting))
                .collect()
        });
        visiting.remove(&link_id);
        rendered
    }
}

struct Placeholder<'a> {
    matched_len: usize,
    variadic: bool,
    name: &'a str,
    mode: Option<&'a str>,
    separator: Option<&'a str>,
}

fn parse_placeholder(source: &str) -> Option<Placeholder<'_>> {
    if !source.starts_with('{') || source.starts_with("{{") || source.starts_with("{?") {
        return None;
    }
    let close = source.find('}')?;
    let content = &source[1..close];
    let (binding, separator) = content
        .split_once('|')
        .map_or((content, None), |(binding, separator)| {
            (binding, Some(separator))
        });
    let binding = binding.trim();
    let (variadic, binding) = binding
        .strip_prefix('*')
        .map_or((false, binding), |binding| (true, binding));
    let (name, mode) = binding
        .split_once(':')
        .map_or((binding, None), |(name, mode)| (name, Some(mode)));
    let name = name.trim();
    let mode = mode.map(str::trim);
    if !valid_binding_name(name) || mode.is_some_and(|value| !valid_identifier(value)) {
        return None;
    }
    Some(Placeholder {
        matched_len: close + 1,
        variadic,
        name,
        mode,
        separator,
    })
}

fn parse_conditional(source: &str) -> Option<(usize, &str)> {
    let rest = source.strip_prefix("{?")?;
    let close = rest.find('}')?;
    let name = &rest[..close];
    valid_identifier(name).then_some((close + 3, name))
}

fn valid_binding_name(value: &str) -> bool {
    value == "." || valid_identifier(value)
}

fn valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

fn contextual_language(target_language: &str, context: &str) -> String {
    format!(
        "{}:{context}",
        target_language.split(':').next().unwrap_or(target_language)
    )
}

fn decode_separator(separator: Option<&str>) -> String {
    let Some(separator) = separator else {
        return String::new();
    };
    let mut output = String::new();
    let mut characters = separator.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match characters.next() {
            Some('n') => output.push('\n'),
            Some('s') => output.push(' '),
            Some('t') => output.push('\t'),
            Some(other) => output.push(other),
            None => output.push('\\'),
        }
    }
    output
}

fn current_indent(output: &str) -> &str {
    let line = output.rsplit_once('\n').map_or(output, |(_, line)| line);
    if line
        .chars()
        .all(|character| matches!(character, ' ' | '\t'))
    {
        line
    } else {
        ""
    }
}

fn indent_continuation(value: &str, indent: &str) -> String {
    if indent.is_empty() || !value.contains('\n') {
        return value.to_string();
    }
    value
        .split('\n')
        .enumerate()
        .map(|(position, line)| {
            if position == 0 || line.is_empty() {
                line.to_string()
            } else {
                format!("{indent}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn concept_id_for_link<'a>(network: &'a LinkNetwork, link: &'a Link) -> Option<&'a str> {
    if link.metadata().link_type() == Some(LinkType::Concept) {
        return link.metadata().term();
    }
    let first_reference = link.references().first().copied()?;
    let concept = network.link(first_reference)?;
    (concept.metadata().link_type() == Some(LinkType::Concept))
        .then(|| concept.metadata().term())
        .flatten()
}

fn reconstruct_concept_for_language<'a>(
    network: &'a LinkNetwork,
    concept: &str,
    language: &str,
) -> Option<&'a str> {
    network.reconstruct_concept(concept, language).or_else(|| {
        canonical_reconstruction_language(language)
            .and_then(|canonical| network.reconstruct_concept(concept, canonical))
    })
}
