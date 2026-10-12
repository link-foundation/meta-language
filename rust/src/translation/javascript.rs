//! JavaScript frontend for the portable core.
//!
//! It reads the subset of modern JavaScript the core can represent
//! faithfully: Number and `BigInt` arithmetic, strings and booleans; top-level functions
//! and object-literal namespaces of methods whose types come from `JSDoc`
//! (`@param`, `@returns`, and `@typedef` unions of `{ $: 'tag', … }` object
//! types for data types) or, where `JSDoc` is silent, are inferred from how the
//! program uses them; bodies made of `const`, `if`, `return`, `throw` and
//! `switch` statements; and a top level of `console.log`, `const` and
//! `node:assert` statements, which are the program's effects. A function
//! whose leading statements throw on a negative argument takes a natural
//! number. Everything else is rejected with a precise obligation.
//!
//! Mirrors `js/src/translation/javascript.js`.

use std::collections::{HashMap, HashSet};

use super::diagnostics::{Result, TranslationError, type_error, unsupported};
use super::lexer::{
    Comment, Source, Token, TokenCursor, TokenKind, describe, is_js_space, tokenize,
};
use super::surface::{
    BinaryOp, External, Guard, SArrayItem, SComparison, SCtor, SData, SEffect, SExpr, SField, SFn,
    SImport, SImportName, SItem, SMain, SModule, SNode, SParam, SPattern, SPatternNode, SProgram,
    SProp, SPropNode, SRow, STagTest, ShowStyle, UnaryOp,
};
use super::types::{BOOL, FLOAT, INT, NAT, STRING, Type, array};
use super::{Language, Span};

mod array_methods;
mod console;
mod declarations;
mod expressions;
mod flow;
mod imperative;
mod imports;
mod infer;
mod loops;
mod lowering;
mod math;
mod object_properties;
mod product_source_rules;
mod products;
mod statements;
use self::flow::{imperative, statement_uses};
use self::imperative::lower_imperative;
use self::lowering::{assertion_kind, guarded_parameter, lower, prop_of};

const ROOT: &str = "crate";
const ASSIGNMENTS: [&str; 16] = [
    "=", "+=", "-=", "*=", "/=", "%=", "**=", "<<=", ">>=", ">>>=", "&=", "|=", "^=", "&&=", "||=",
    "??=",
];
/// `x op= e` is `x = x op e`; `+=` adds numbers or concatenates strings like `+`.
const COMPOUND: [(&str, BinaryOp); 7] = [
    ("+=", BinaryOp::Plus),
    ("-=", BinaryOp::Sub),
    ("*=", BinaryOp::Mul),
    ("/=", BinaryOp::Div),
    ("%=", BinaryOp::Rem),
    ("&&=", BinaryOp::And),
    ("||=", BinaryOp::Or),
];
const ERRORS: [&str; 3] = ["Error", "RangeError", "TypeError"];
const GLOBALS: [&str; 24] = [
    "Math",
    "Number",
    "BigInt",
    "parseInt",
    "parseFloat",
    "JSON",
    "Array",
    "Object",
    "Symbol",
    "Date",
    "Promise",
    "Reflect",
    "Proxy",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "globalThis",
    "process",
    "require",
    "console",
    "setTimeout",
    "isNaN",
    "isFinite",
];
/// The own properties of `Object.prototype`. The JavaScript frontend looks
/// assertion methods up in a plain object, so these names inherit a truthy
/// entry and read as `notDeepStrictEqual`.
const OBJECT_PROTOTYPE: [&str; 12] = [
    "constructor",
    "__defineGetter__",
    "__defineSetter__",
    "hasOwnProperty",
    "__lookupGetter__",
    "__lookupSetter__",
    "isPrototypeOf",
    "propertyIsEnumerable",
    "toString",
    "valueOf",
    "__proto__",
    "toLocaleString",
];

/// What an `assert.method(…)` call asserts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AssertionKind {
    Eq,
    Ne,
    Deep,
    NotDeep,
}

/// Reads a JavaScript ES module into the surface AST.
///
/// # Errors
///
/// On syntax errors, type errors in `JSDoc` annotations and constructs outside
/// the portable core.
pub fn parse_javascript(source: &str) -> Result<SProgram> {
    parse_javascript_in(source, &ModuleContext::default())
}

/// What self-translation tells the frontend about the module of the item it reads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleContext {
    /// The signatures of the module's other items and of the items it
    /// imports, which the item may call and read.
    pub externals: Vec<External>,
    /// The module's directory inside its crate, as path segments; with it,
    /// relative imports are items of the crate.
    pub module_directory: Option<Vec<String>>,
}

/// [`parse_javascript`] for one item of a module in `context`.
///
/// # Errors
///
/// As [`parse_javascript`].
pub fn parse_javascript_in(source: &str, context: &ModuleContext) -> Result<SProgram> {
    parse_javascript_bound_in(source, context, &HashMap::new(), false)
}

pub(crate) fn parse_javascript_bound_in(
    source: &str,
    context: &ModuleContext,
    literal_bindings: &HashMap<String, SExpr>,
    readonly_module_constants: bool,
) -> Result<SProgram> {
    let tokens = tokenize(source, Language::JavaScript)?;
    let mut parser = JavaScriptParser::new(source, tokens.tokens, &tokens.comments);
    parser.literal_bindings.clone_from(literal_bindings);
    parser.readonly_module_constants = readonly_module_constants;
    parser.externals.clone_from(&context.externals);
    parser
        .module_directory
        .clone_from(&context.module_directory);
    parser.file()
}

/// Locals in scope, the ones `let` or a parameter makes assignable, and
/// names of the current block still in their temporal dead zone.
#[derive(Clone, Default)]
struct Scope {
    locals: HashSet<String>,
    tdz: HashSet<String>,
    mutable: HashSet<String>,
}

/// Enclosing loops and switches, for `break` and `continue`.
#[derive(Clone, Copy, Default)]
struct Jumps {
    loops: usize,
    switches: usize,
}

/// The local name of the imported `node:assert` module.
struct Assertion {
    name: String,
    strict: bool,
}

/// A `JSDoc` block's `@param` types, in order, and its `@returns` type.
struct JsDoc {
    params: Vec<(String, String)>,
    returns: Option<String>,
    range: Span,
}

/// A `JSDoc` tag: `@tag {type} name`.
struct JsDocTag {
    tag: String,
    ty: Option<String>,
    name: Option<String>,
}

/// Function-body statements before lowering to one expression.
#[derive(Clone)]
enum Stmt {
    Const {
        name: String,
        value: SExpr,
        span: Span,
    },
    /// `let name = value`, an assignable local.
    Let {
        name: String,
        value: SExpr,
        span: Span,
    },
    /// `name = value`, and `x op= e`, `x++` and `x--` read as one.
    Assign {
        name: String,
        value: SExpr,
        span: Span,
    },
    While {
        cond: SExpr,
        body: Vec<Self>,
        span: Span,
    },
    DoWhile {
        body: Vec<Self>,
        cond: SExpr,
        span: Span,
    },
    /// `for (init; cond; update) body`, whose `init` are `let` declarations or assignments.
    For {
        init: Vec<Self>,
        cond: Option<SExpr>,
        update: Vec<Self>,
        body: Vec<Self>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Return {
        expr: SExpr,
        span: Span,
    },
    Throw {
        message: String,
        span: Span,
    },
    /// A braced block; `inline` for the bindings of a destructuring `const`.
    Block {
        body: Vec<Self>,
        inline: bool,
        span: Span,
    },
    Empty,
    If {
        cond: SExpr,
        then: Vec<Self>,
        otherwise: Option<Vec<Self>>,
        span: Span,
    },
    Switch(Switch),
    Expr {
        expr: SExpr,
        span: Span,
    },
    /// `console.log(expr)`.
    Print {
        expr: SExpr,
        style: ShowStyle,
        span: Span,
    },
}

#[derive(Clone)]
struct Switch {
    /// The switch subject: the discriminant, or the object of `x.$`.
    scrutinee: SExpr,
    /// The subject's name and data type in a tag switch.
    tag: Option<(String, SData)>,
    clauses: Vec<Clause>,
    span: Span,
}

#[derive(Clone)]
struct Clause {
    tests: Vec<CaseTest>,
    body: Vec<Stmt>,
    span: Span,
}

#[derive(Clone)]
enum CaseTest {
    Default {
        span: Span,
    },
    Tag {
        tag: String,
        span: Span,
    },
    NumLit {
        value: String,
        negative: bool,
        span: Span,
    },
    BoolLit {
        value: bool,
        span: Span,
    },
}

/// Where a block-declaration scan stops: the end of the file or the block's `}`.
#[derive(Clone, Copy)]
enum ScanEnd {
    File,
    Brace,
}

struct JavaScriptParser {
    source: Source,
    cursor: TokenCursor,
    docs: Vec<Comment>,
    /// Tags of every @typedef data type, for `switch (x.$)`.
    data_types: Vec<SData>,
    scope: Scope,
    literal_bindings: HashMap<String, SExpr>,
    assertion: Option<Assertion>,
    /// Async functions run sequentially: each call of one is awaited where it
    /// is made, so nothing else runs until its result is back. `await` is
    /// allowed at the top level of the module and in async functions.
    async_names: HashSet<String>,
    in_async: bool,
    unawaited: Vec<(String, Span)>,
    sequential_async: bool,
    jumps: Jumps,
    /// Whether a top-level statement is being read, where `return` is a syntax error.
    top_level: bool,
    readonly_module_constants: bool,
    readonly_constant_initializer: bool,
    /// Functions and data types that loops and joins of statements lower to, and their count.
    generated: Vec<SItem>,
    generated_count: usize,
    /// The other items of the module an item may call and read.
    externals: Vec<External>,
    /// The module's directory inside its crate, which makes relative imports items of the crate.
    module_directory: Option<Vec<String>>,
    imports: Vec<SImport>,
    exported_names: HashSet<String>,
    function_names: HashSet<String>,
    nested_product_bindings: HashSet<Span>,
    global_binding_names: HashSet<String>,
}

impl JavaScriptParser {
    fn new(source: &str, tokens: Vec<Token>, comments: &[Comment]) -> Self {
        Self {
            source: Source::new(source),
            cursor: TokenCursor::new(tokens, Language::JavaScript),
            docs: comments
                .iter()
                .filter(|comment| comment.text.starts_with("/**") && comment.text != "/**/")
                .cloned()
                .collect(),
            data_types: Vec::new(),
            scope: Scope::default(),
            literal_bindings: HashMap::new(),
            assertion: None,
            async_names: HashSet::new(),
            in_async: true,
            unawaited: Vec::new(),
            sequential_async: false,
            jumps: Jumps::default(),
            top_level: false,
            readonly_module_constants: false,
            readonly_constant_initializer: false,
            generated: Vec::new(),
            generated_count: 0,
            externals: Vec::new(),
            module_directory: None,
            imports: Vec::new(),
            exported_names: HashSet::new(),
            function_names: HashSet::new(),
            nested_product_bindings: HashSet::new(),
            global_binding_names: HashSet::new(),
        }
    }

    fn peek(&self) -> Token {
        self.cursor.peek().clone()
    }

    fn peek_at(&self, offset: usize) -> Token {
        self.cursor.peek_at(offset).clone()
    }

    /// The span from `from` to the next token.
    fn to_here(&self, from: &Token) -> Span {
        span(from, self.cursor.peek())
    }

    fn fail(message: &str, token: &Token) -> TranslationError {
        TranslationError::syntax(
            format!("{message} but found {}", describe(token)),
            Some(token.span()),
        )
    }

    fn fail_here(&self, message: &str) -> TranslationError {
        Self::fail(message, self.cursor.peek())
    }
}

/// `JSDoc` tags of a comment: `@tag {type} name`. Leading `*` of each line is
/// decoration.
fn jsdoc_tags(comment: &Comment) -> Result<Vec<JsDocTag>> {
    let units: Vec<u16> = comment.text.encode_utf16().collect();
    let inner = String::from_utf16_lossy(
        &units[3.min(units.len())..units.len().saturating_sub(2).max(3.min(units.len()))],
    );
    let text: Vec<char> = inner
        .split('\n')
        .map(|line| {
            let line = line
                .trim_start_matches(|ch: char| u16::try_from(u32::from(ch)).is_ok_and(is_js_space));
            line.strip_prefix('*').unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
        .chars()
        .collect();
    let mut tags = Vec::new();
    let mut at = 0;
    while let Some((tag_start, tag_end)) = next_tag(&text, at) {
        let tag: String = text[tag_start + 1..tag_end].iter().collect();
        let mut index = tag_end;
        while matches!(text.get(index), Some(' ' | '\t')) {
            index += 1;
        }
        let mut ty = None;
        if text.get(index) == Some(&'{') {
            let mut depth = 0;
            let mut end = index;
            while end < text.len() {
                if text[end] == '{' {
                    depth += 1;
                }
                if text[end] == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                end += 1;
            }
            if depth != 0 {
                return Err(TranslationError::syntax(
                    format!("unbalanced braces in the JSDoc type of @{tag}"),
                    Some(Span::new(comment.start, comment.end)),
                ));
            }
            ty = Some(
                text[index + 1..end]
                    .iter()
                    .map(|&ch| if ch == '\n' { ' ' } else { ch })
                    .collect(),
            );
            index = end + 1;
        }
        tags.push(JsDocTag {
            tag,
            ty,
            name: jsdoc_name(&text[index.min(text.len())..]),
        });
        at = index;
    }
    Ok(tags)
}

/// The next `@tag` at or after `from`: the positions of the `@` and after the letters.
fn next_tag(text: &[char], from: usize) -> Option<(usize, usize)> {
    (from..text.len()).find_map(|start| {
        if text[start] != '@' {
            return None;
        }
        let letters = text[start + 1..]
            .iter()
            .take_while(|ch| ch.is_ascii_alphabetic())
            .count();
        (letters > 0).then_some((start, start + 1 + letters))
    })
}

/// Read the shared `JSDoc` name range; convert its UTF-16 offsets at the host boundary.
fn jsdoc_name(text: &[char]) -> Option<String> {
    let tail: String = text.iter().collect();
    let units: Vec<u16> = tail.encode_utf16().collect();
    let numeric: Vec<f64> = units.iter().map(|&unit| f64::from(unit)).collect();
    let range = super::frontend_rules::find_documentation_parameter_range(&numeric);
    if range.is_empty() {
        return None;
    }
    let start = super::frontend_rules::ml_array::number_index(range[0])?;
    let end = super::frontend_rules::ml_array::number_index(range[1])?;
    Some(String::from_utf16_lossy(units.get(start..end)?))
}

#[cfg(test)]
mod product_tests {
    use super::{
        External, ModuleContext, SItem, SNode, Type, parse_javascript, parse_javascript_in,
    };
    use crate::translation::surface::ExternalParam;

    #[test]
    fn translation_javascript_products_use_closed_fresh_values_and_static_positions() {
        let source = "/** @returns {[number,boolean]} */ function make(){return Object.freeze([5,true]);}\n\
            /** @param {[number,boolean]} value @returns {number} */ function choose(value){const [number,flag]=value;return flag?number+2:number;}\n\
            const value=make();console.log(choose(value)===7);console.log(value[-0]===value[0]);";
        let surface = parse_javascript(source).expect("closed fresh tuple products parse");
        let SItem::Fn(make) = &surface.items[0] else {
            panic!("the factory is a function");
        };
        let SNode::Tuple {
            product_model,
            frozen,
            ..
        } = &make.body.node
        else {
            panic!("the factory retains its source product model");
        };
        assert!(*frozen);
        assert_eq!(product_model.as_deref(), Some("immutable value product"));
        super::super::check::check_program(&surface).expect("the closed product program checks");
        let value = serde_json::to_value(&surface).expect("source products serialize");
        assert_eq!(
            value["main"]["effects"][2]["expr"]["left"]["k"],
            "tupleIndex"
        );
        assert_eq!(value["main"]["effects"][2]["expr"]["left"]["index"], 0);
    }

    #[test]
    fn translation_javascript_product_destructuring_keeps_one_initializer_evaluation() {
        let source = "function make(){console.log('read');return Object.freeze([Object.freeze([5,true]),2]);}\n\
            const [[value,flag],increment]=make();console.log(flag?value+increment:value);";
        let surface = parse_javascript(source).expect("nested fresh products parse");
        super::super::check::check_program(&surface).expect("nested product positions check");
        let value = serde_json::to_value(&surface).expect("the source program serializes");
        let effects = &value["main"]["effects"];
        assert_eq!(effects[0]["value"]["path"][1], "make");
        assert_eq!(effects[1]["name"], "value");
        assert_eq!(effects[2]["name"], "flag");
        assert_eq!(effects[3]["name"], "increment");
        assert_eq!(effects[1]["value"]["k"], "tupleIndex");
        assert_eq!(effects[1]["value"]["object"]["k"], "tupleIndex");
    }

    #[test]
    fn translation_javascript_product_sources_refuse_unproved_ownership_and_uses() {
        for source in [
            "function make(){return Object.freeze([[5,true],2]);}",
            "function make(){return Object.freeze([]);}",
            "function Object(value){return value;}function make(){return Object.freeze([5,true]);}",
            "export function make(){return Object.freeze([5,true]);}",
            "const value=Object.freeze([5,true]);console.log(value===value);",
            "/** @returns {[number,boolean]} */ function make(){return [5,true];}console.log(make()[0]);",
            "/** @param {[number,boolean]} value */ function read(value){return value[0];}console.log(read([5,true]));",
            "function read(){const [[value]]=[[5]];return value;}console.log(read());",
        ] {
            assert!(
                parse_javascript(source)
                    .and_then(|surface| super::super::check::check_program(&surface))
                    .is_err(),
                "unproved products must refuse: {source}"
            );
        }
    }

    #[test]
    fn translation_javascript_products_preserve_legacy_arrays_and_require_actual_callers() {
        let source = "/** @param {[number,number]} pair */function first(pair){return pair[0];}console.log(first([2,3]));";
        let surface = parse_javascript(source).expect("legacy homogeneous positional arrays parse");
        super::super::check::check_program(&surface).expect("legacy positional reads check");
        for source in [
            "/** @returns {[string,number]} */function pair(){return ['name',5];}",
            "/** @param {string[]} values @returns {[string,number]} */function pair(values){return [values[0],1];}",
        ] {
            let surface = parse_javascript(source).expect("legacy reference-bearing tuples parse");
            super::super::check::check_program(&surface)
                .expect("legacy reference-bearing tuples check");
        }
        for source in [
            "function make(){return Object.freeze([1,2]);}",
            "function make(){return Object.freeze([5,true]);}",
        ] {
            let error =
                parse_javascript(source).expect_err("a product return needs actual callers");
            assert!(error.to_string().contains("Object.freeze operand"));
        }
    }

    #[test]
    fn translation_javascript_recursive_product_inference_has_a_finite_refusal() {
        let error = parse_javascript("function make(){return Object.freeze([make(),true]);}")
            .expect_err("a recursive inferred product needs a declared data type");
        assert_eq!(
            error.reason,
            "a recursive inferred type needs an explicit data type"
        );
    }

    #[test]
    fn translation_javascript_product_fields_resolve_shadowed_global_bindings_and_tdz() {
        for name in ["NaN", "Infinity", "undefined"] {
            let source = format!(
                "function make({name}){{return Object.freeze([{name},true]);}}const value=make(5);console.log(value[0]===5);"
            );
            let surface =
                parse_javascript(&source).expect("global binding names can be lexical parameters");
            super::super::check::check_program(&surface)
                .expect("the source product retains its actual parameter");
            let SItem::Fn(function) = &surface.items[0] else {
                panic!("the factory is a function");
            };
            let SNode::Tuple { items, .. } = &function.body.node else {
                panic!("the source is a frozen tuple");
            };
            assert!(matches!(&items[0].node, SNode::Name { path } if path == &[name.to_owned()]));
            let source = format!(
                "function make(){{const value=Object.freeze([{name},true]);const {name}=5;return value;}}console.log(make()[0]);"
            );
            assert!(
                parse_javascript(&source)
                    .expect_err("the later lexical binding remains in its temporal dead zone")
                    .reason
                    .contains("temporal dead zone")
            );
            let source = format!(
                "function make(){{return Object.freeze([{name},true]);}}const {name}=5;console.log(make()[0]);"
            );
            assert!(
                parse_javascript(&source)
                    .expect_err("an uncaptured module binding cannot become a built-in constant")
                    .construct
                    .as_deref()
                    .is_some_and(|construct| construct.contains("global binding"))
            );
            let source = format!(
                "function make(){{return Object.freeze([{name},true]);}}import {{value as {name}}} from './values.js';"
            );
            assert!(
                parse_javascript_in(
                    &source,
                    &ModuleContext {
                        module_directory: Some(vec!["example".to_owned()]),
                        ..ModuleContext::default()
                    }
                )
                .expect_err("an imported alias cannot become a built-in constant")
                .construct
                .as_deref()
                .is_some_and(|construct| construct.contains("global binding"))
            );
        }
    }

    #[test]
    fn translation_javascript_whole_product_conditions_keep_an_explicit_obligation() {
        for condition in [
            "value",
            "value?true:false",
            "!value",
            "value&&true",
            "value||false",
        ] {
            let source = format!(
                "function check(){{const value=Object.freeze([5,true]);if({condition})return true;return false;}}console.log(check());"
            );
            let error = parse_javascript(&source).expect_err("a whole product condition refuses");
            if !condition.contains("&&") && !condition.contains("||") {
                assert_eq!(
                    error.construct.as_deref(),
                    Some("immutable tuple product use")
                );
            }
        }
    }

    #[test]
    fn translation_javascript_parameter_defaults_share_product_caller_and_escape_proofs() {
        let source = "function read(value=Object.freeze([5,true])){return value[0];}console.log(read()===5);console.log(read(Object.freeze([7,true]))===7);";
        let surface =
            parse_javascript(source).expect("fresh default products stay in closed calls");
        super::super::check::check_program(&surface).expect("the default product positions check");
        let context = ModuleContext {
            externals: vec![External::Function {
                name: "escape".to_owned(),
                params: vec![ExternalParam {
                    name: "value".to_owned(),
                    ty: Type::Tuple {
                        elements: vec![Type::Float, Type::Bool],
                    },
                }],
                ret: Type::Bool,
            }],
            ..ModuleContext::default()
        };
        let error = parse_javascript_in(
            "function run(flag=escape(Object.freeze([5,true]))){return flag;}console.log(run());",
            &context,
        )
        .expect_err("a product cannot escape through a parameter default");
        assert!(error.reason.contains("unknown or external caller"));
    }

    #[test]
    fn translation_javascript_default_parameter_bindings_keep_temporal_dead_zones() {
        for name in ["NaN", "Infinity", "undefined"] {
            for parameters in [
                format!("{name}={name}"),
                format!("value={name},{name}=5"),
                format!("{name}=`${{{name}}}`"),
            ] {
                let source = format!(
                    "function make({parameters}){{return Object.freeze([5,true]);}}make();"
                );
                let error = parse_javascript(&source)
                    .expect_err("defaults cannot substitute constants for parameter bindings");
                assert!(
                    error.reason.contains("temporal dead zone")
                        || error.construct.as_deref().is_some_and(|construct| {
                            construct.contains("default reading parameter")
                        })
                );
            }
        }
    }
}

/// `/^[A-Za-z_$][\w$]*$/`.
fn is_identifier_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_' || first == '$')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$')
}

/// `String.prototype.trim`: JavaScript whitespace and line terminators.
fn js_trim(text: &str) -> &str {
    text.trim_matches(|ch: char| u16::try_from(u32::from(ch)).is_ok_and(is_js_space))
}

/// A `JSDoc` type, where an empty one counts as missing.
fn non_empty(text: Option<String>) -> Option<String> {
    text.filter(|text| !text.is_empty())
}

/// `5n` and `0x1fn` are `BigInt` literals, which the lexer keeps as decimal
/// digits; `42`, `1.5` and `1e21` are Numbers, IEEE-754 doubles, which the
/// lexer keeps as `String(value)`.
fn number_literal(token: &Token) -> SExpr {
    node(
        SNode::Num {
            value: token.value.clone(),
            ty: (token.suffix != "n").then_some(FLOAT),
            negative: false,
            unit: false,
        },
        span(token, token),
    )
}

fn has_ctor(data: &SData, tag: &str) -> bool {
    data.ctors.iter().any(|ctor| ctor.name == tag)
}

fn is_tag_field(expr: &SExpr) -> bool {
    matches!(&expr.node, SNode::Field { field, .. } if field == "$")
}

const fn node(node: SNode, place: Span) -> SExpr {
    SExpr::new(node, Some(place))
}

const fn wild(place: Span) -> SPattern {
    SPattern {
        node: SPatternNode::Wild,
        span: Some(place),
    }
}

fn binary(op: BinaryOp, left: SExpr, right: SExpr, token: &Token) -> SExpr {
    let place = joined(&left, &right, token);
    node(
        SNode::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            rounding: None,
        },
        place,
    )
}

fn joined(left: &SExpr, right: &SExpr, token: &Token) -> Span {
    Span::new(
        left.span.map_or(token.start, |place| place.start),
        right.span.map_or(token.end, |place| place.end),
    )
}

fn span(from: &Token, to: &Token) -> Span {
    let end = if to.kind == TokenKind::Eof {
        from.end
    } else {
        to.start
    };
    Span::new(from.start, from.end.max(end))
}
