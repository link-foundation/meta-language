//! The feature union forms of the native listing and the links form, rendered
//! and read from the tables of [`crate::grammar::feature`]. It mirrors the
//! codecs of `js/src/grammar-feature-forms.js`.

use super::super::feature::{
    ByteClassItem, FeatureExpr, FeatureForm, FieldType, FieldValue, FormFields, Operation,
    OperationCategory, check_integer, feature_expression_fields, operation_form,
};
use super::super::{GrammarExpr, GrammarImportError};
use super::native::Cursor;

/// How one serialization spells names, texts, expressions and calls.
pub(super) trait FormCodec {
    fn name(&self, value: &str) -> String;
    fn text(&self, value: &str) -> String;
    fn expression(&self, expr: &GrammarExpr) -> String;
    fn call(&self, head: &str, parts: Vec<String>) -> String;
    fn block(&self, word: &str, parts: Vec<String>) -> String;
    fn byte_class(&self, negated: bool, items: Vec<String>) -> String;
}

fn render_fields(fields: FormFields, values: &[FieldValue], codec: &impl FormCodec) -> Vec<String> {
    let mut parts = Vec::new();
    for ((_, kind), value) in fields.iter().zip(values) {
        match (kind, value) {
            (_, FieldValue::Word(word)) => parts.push(word.clone()),
            (_, FieldValue::Integer(value)) => parts.push(value.to_string()),
            (FieldType::Text, FieldValue::Text(text) | FieldValue::Name(text)) => {
                parts.push(codec.text(text));
            }
            (_, FieldValue::Name(name) | FieldValue::Text(name)) => parts.push(codec.name(name)),
            (_, FieldValue::Expression(expr)) => parts.push(codec.expression(expr)),
            (_, FieldValue::Expressions(items)) => {
                parts.extend(items.iter().map(|item| codec.expression(item)));
            }
            (_, FieldValue::Operation(operation)) => parts.push(render_operation(operation, codec)),
            (_, FieldValue::Operations(items)) => {
                parts.extend(items.iter().map(|item| render_operation(item, codec)));
            }
            (FieldType::Block(word, _), FieldValue::Block(Some(items))) => parts.push(
                codec.block(
                    word,
                    items
                        .iter()
                        .map(|item| render_operation(item, codec))
                        .collect(),
                ),
            ),
            (_, FieldValue::Block(_)) => {}
        }
    }
    parts
}

/// Renders one operation of the scanner and action language.
pub(super) fn render_operation(operation: &Operation, codec: &impl FormCodec) -> String {
    let fields = operation_form(&operation.head).map_or(&[][..], |(_, fields)| fields);
    codec.call(
        &operation.head,
        render_fields(fields, &operation.fields, codec),
    )
}

/// Renders a feature form or a byte class; the codec spells Unicode classes
/// and calls itself.
pub(super) fn render_feature_form(feature: &FeatureExpr, codec: &impl FormCodec) -> Option<String> {
    match feature {
        FeatureExpr::ByteClass { negated, items } => {
            let items = items
                .iter()
                .map(|item| match item {
                    ByteClassItem::Byte(value) => codec.call("byte", vec![value.to_string()]),
                    ByteClassItem::Range(start, end) => {
                        codec.call("byteRange", vec![start.to_string(), end.to_string()])
                    }
                })
                .collect();
            Some(codec.byte_class(*negated, items))
        }
        FeatureExpr::Form(form) => {
            let fields = feature_expression_fields(&form.head)?;
            Some(codec.call(&form.head, render_fields(fields, &form.fields, codec)))
        }
        FeatureExpr::UnicodeClass { .. } | FeatureExpr::Call { .. } => None,
    }
}

fn check_byte(value: i64) -> Result<u8, String> {
    u8::try_from(value).map_err(|_| format!("byte {value} is outside 0..255"))
}

impl Cursor {
    fn integer(&mut self) -> Result<i64, GrammarImportError> {
        let begin = self.position;
        if self.peek() == Some('-') {
            self.position += 1;
        }
        while self.peek().is_some_and(|value| value.is_ascii_digit()) {
            self.position += 1;
        }
        check_integer(&self.text(begin)).map_err(|detail| self.fail(detail))
    }

    fn byte(&mut self) -> Result<u8, GrammarImportError> {
        let value = self.integer()?;
        check_byte(value).map_err(|detail| self.fail(detail))
    }

    /// Reads one operation of `category` at the cursor.
    pub(super) fn operation(
        &mut self,
        category: OperationCategory,
    ) -> Result<Operation, GrammarImportError> {
        let head = self.word()?;
        let Some((found, fields)) = operation_form(&head) else {
            return Err(self.fail(format!("unknown operation {head}")));
        };
        if found != category {
            return Err(self.fail(format!(
                "{head} is a {}, not a {}",
                found.as_str(),
                category.as_str()
            )));
        }
        let fields = self.fields(fields)?;
        Ok(FeatureForm::new(head, fields))
    }

    /// Reads a `(OPERATION, ...)` statement list.
    pub(super) fn statements(&mut self) -> Result<Vec<Operation>, GrammarImportError> {
        self.list(|cursor| cursor.operation(OperationCategory::Statement))
    }

    fn fields(&mut self, fields: FormFields) -> Result<Vec<FieldValue>, GrammarImportError> {
        let mut values = Vec::new();
        if fields.is_empty() {
            return Ok(values);
        }
        self.open()?;
        let mut first = true;
        for (_, kind) in fields {
            self.skip_spaces();
            if let FieldType::Block(_, true) = kind
                && self.peek() == Some(')')
            {
                values.push(FieldValue::Block(None));
                continue;
            }
            if matches!(kind, FieldType::Expressions | FieldType::Conditions) {
                let mut items = Vec::new();
                let mut conditions = Vec::new();
                self.skip_spaces();
                while self.peek() != Some(')') {
                    if !first {
                        self.separator()?;
                    }
                    first = false;
                    if *kind == FieldType::Expressions {
                        items.push(self.expression()?);
                    } else {
                        conditions.push(self.operation(OperationCategory::Condition)?);
                    }
                    self.skip_spaces();
                    if self.done() {
                        return Err(self.fail("expected )"));
                    }
                }
                values.push(if *kind == FieldType::Expressions {
                    FieldValue::Expressions(items)
                } else {
                    FieldValue::Operations(conditions)
                });
                continue;
            }
            if !first {
                self.separator()?;
            }
            first = false;
            values.push(match kind {
                FieldType::Choice(words) => {
                    let word = self.word()?;
                    if !words.contains(&word.as_str()) {
                        return Err(
                            self.fail(format!("expected {}, not {word}", words.join(" or ")))
                        );
                    }
                    FieldValue::Word(word)
                }
                FieldType::Integer => FieldValue::Integer(self.integer()?),
                FieldType::Name => FieldValue::Name(self.name()?),
                FieldType::Text => FieldValue::Text(self.string()?),
                FieldType::Expression => FieldValue::Expression(self.expression()?),
                FieldType::Condition => {
                    FieldValue::Operation(self.operation(OperationCategory::Condition)?)
                }
                FieldType::Value => {
                    FieldValue::Operation(self.operation(OperationCategory::Value)?)
                }
                FieldType::Block(word, _) => {
                    if self.word()? != *word {
                        return Err(self.fail(format!("expected {word}(...)")));
                    }
                    FieldValue::Block(Some(self.statements()?))
                }
                FieldType::Expressions | FieldType::Conditions => {
                    unreachable!("lists are read above")
                }
            });
        }
        self.close()?;
        Ok(values)
    }

    /// Reads a feature form or a byte class headed by `head`, or `None`.
    pub(super) fn feature(
        &mut self,
        head: &str,
    ) -> Result<Option<GrammarExpr>, GrammarImportError> {
        if head == "byteClass" || head == "notByteClass" {
            let items = self.list(Self::byte_item)?;
            return Ok(Some(GrammarExpr::feature(FeatureExpr::ByteClass {
                negated: head == "notByteClass",
                items,
            })));
        }
        let Some(fields) = feature_expression_fields(head) else {
            return Ok(None);
        };
        let fields = self.fields(fields)?;
        if head == "longest"
            && matches!(fields.first(), Some(FieldValue::Expressions(items)) if items.is_empty())
        {
            return Err(self.fail("longest needs an alternative"));
        }
        Ok(Some(GrammarExpr::feature(FeatureExpr::Form(
            FeatureForm::new(head, fields),
        ))))
    }

    fn byte_item(&mut self) -> Result<ByteClassItem, GrammarImportError> {
        let head = self.word()?;
        match head.as_str() {
            "byte" => {
                self.open()?;
                let value = self.byte()?;
                self.close()?;
                Ok(ByteClassItem::Byte(value))
            }
            "byteRange" => {
                self.open()?;
                let start = self.byte()?;
                self.separator()?;
                let end = self.byte()?;
                self.close()?;
                if end < start {
                    return Err(self.fail(format!("byte range {start}, {end} is reversed")));
                }
                Ok(ByteClassItem::Range(start, end))
            }
            other => Err(self.fail(format!("unknown byte class item {other}"))),
        }
    }
}

/// The links reader helpers, implemented by the links codec.
pub(super) trait LinksReader {
    type Node;
    fn parts<'a>(
        &self,
        node: &'a Self::Node,
    ) -> Result<(&'a str, &'a [Self::Node]), GrammarImportError>;
    fn word<'a>(&self, node: &'a Self::Node, what: &str) -> Result<&'a str, GrammarImportError>;
    fn decoded_word(&self, node: &Self::Node, what: &str) -> Result<String, GrammarImportError>;
    fn expression(&self, node: &Self::Node) -> Result<GrammarExpr, GrammarImportError>;
    fn fail(&self, detail: String) -> GrammarImportError;
}

fn take<'a, R: LinksReader>(
    reader: &R,
    args: &'a [R::Node],
    index: &mut usize,
    head: &str,
    key: &str,
) -> Result<&'a R::Node, GrammarImportError> {
    let value = args
        .get(*index)
        .ok_or_else(|| reader.fail(format!("{head} needs {key}")));
    *index += 1;
    value
}

fn read_links_fields<R: LinksReader>(
    reader: &R,
    fields: FormFields,
    args: &[R::Node],
    head: &str,
) -> Result<Vec<FieldValue>, GrammarImportError> {
    let mut values = Vec::new();
    let mut index = 0;
    for (key, kind) in fields {
        values.push(match kind {
            FieldType::Choice(words) => {
                let word = reader.word(
                    take(reader, args, &mut index, head, key)?,
                    &words.join(" or "),
                )?;
                if !words.contains(&word) {
                    return Err(reader.fail(format!("expected {}, not {word}", words.join(" or "))));
                }
                FieldValue::Word(word.to_owned())
            }
            FieldType::Integer => {
                let text = reader.word(take(reader, args, &mut index, head, key)?, "an integer")?;
                FieldValue::Integer(check_integer(text).map_err(|detail| reader.fail(detail))?)
            }
            FieldType::Name => FieldValue::Name(reader.decoded_word(
                take(reader, args, &mut index, head, key)?,
                &format!("a {key}"),
            )?),
            FieldType::Text => FieldValue::Text(
                reader.decoded_word(take(reader, args, &mut index, head, key)?, "a text")?,
            ),
            FieldType::Expression => FieldValue::Expression(
                reader.expression(take(reader, args, &mut index, head, key)?)?,
            ),
            FieldType::Condition => FieldValue::Operation(read_links_operation(
                reader,
                take(reader, args, &mut index, head, key)?,
                OperationCategory::Condition,
            )?),
            FieldType::Value => FieldValue::Operation(read_links_operation(
                reader,
                take(reader, args, &mut index, head, key)?,
                OperationCategory::Value,
            )?),
            FieldType::Expressions => {
                let rest = args.get(index..).unwrap_or_default();
                index = args.len();
                FieldValue::Expressions(
                    rest.iter()
                        .map(|item| reader.expression(item))
                        .collect::<Result<_, _>>()?,
                )
            }
            FieldType::Conditions => {
                let rest = args.get(index..).unwrap_or_default();
                index = args.len();
                FieldValue::Operations(
                    rest.iter()
                        .map(|item| {
                            read_links_operation(reader, item, OperationCategory::Condition)
                        })
                        .collect::<Result<_, _>>()?,
                )
            }
            FieldType::Block(word, optional) => {
                if *optional && index >= args.len() {
                    values.push(FieldValue::Block(None));
                    continue;
                }
                let (block_head, block_args) =
                    reader.parts(take(reader, args, &mut index, head, key)?)?;
                if block_head != *word {
                    return Err(reader.fail(format!("expected ({word} ...), not {block_head}")));
                }
                FieldValue::Block(Some(
                    block_args
                        .iter()
                        .map(|item| {
                            read_links_operation(reader, item, OperationCategory::Statement)
                        })
                        .collect::<Result<_, _>>()?,
                ))
            }
        });
    }
    if index != args.len() {
        return Err(reader.fail(format!("{head} takes fewer values")));
    }
    Ok(values)
}

/// Reads one operation of `category` from the links form.
pub(super) fn read_links_operation<R: LinksReader>(
    reader: &R,
    node: &R::Node,
    category: OperationCategory,
) -> Result<Operation, GrammarImportError> {
    let (head, args) = reader.parts(node)?;
    let Some((found, fields)) = operation_form(head) else {
        return Err(reader.fail(format!("unknown operation {head}")));
    };
    if found != category {
        return Err(reader.fail(format!(
            "{head} is a {}, not a {}",
            found.as_str(),
            category.as_str()
        )));
    }
    Ok(FeatureForm::new(
        head,
        read_links_fields(reader, fields, args, head)?,
    ))
}

/// Reads a feature form or a byte class from the links form, or `None`.
pub(super) fn read_links_feature<R: LinksReader>(
    reader: &R,
    head: &str,
    args: &[R::Node],
) -> Result<Option<GrammarExpr>, GrammarImportError> {
    if head == "byteClass" {
        let Some((first, rest)) = args.split_first() else {
            return Err(reader.fail("byteClass needs plain or negated".to_owned()));
        };
        let flag = reader.word(first, "plain or negated")?;
        if flag != "plain" && flag != "negated" {
            return Err(reader.fail(format!("expected plain or negated, not {flag}")));
        }
        let byte = |node: &R::Node| -> Result<u8, GrammarImportError> {
            let text = reader.word(node, "a byte")?;
            check_integer(text)
                .and_then(check_byte)
                .map_err(|detail| reader.fail(detail))
        };
        let items = rest
            .iter()
            .map(|item| {
                let (item_head, item_args) = reader.parts(item)?;
                match (item_head, item_args) {
                    ("byte", [value]) => Ok(ByteClassItem::Byte(byte(value)?)),
                    ("byteRange", [start, end]) => {
                        let (start, end) = (byte(start)?, byte(end)?);
                        if end < start {
                            return Err(
                                reader.fail(format!("byte range {start}, {end} is reversed"))
                            );
                        }
                        Ok(ByteClassItem::Range(start, end))
                    }
                    _ => Err(reader.fail(format!("unknown byte class item {item_head}"))),
                }
            })
            .collect::<Result<_, _>>()?;
        return Ok(Some(GrammarExpr::feature(FeatureExpr::ByteClass {
            negated: flag == "negated",
            items,
        })));
    }
    let Some(fields) = feature_expression_fields(head) else {
        return Ok(None);
    };
    let fields = read_links_fields(reader, fields, args, head)?;
    if head == "longest"
        && matches!(fields.first(), Some(FieldValue::Expressions(items)) if items.is_empty())
    {
        return Err(reader.fail("longest needs an alternative".to_owned()));
    }
    Ok(Some(GrammarExpr::feature(FeatureExpr::Form(
        FeatureForm::new(head, fields),
    ))))
}
