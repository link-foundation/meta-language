//! The Rust emitter: writes a checked program as Rust with its translation contract.
//!
//! Naturals and integers are `ml::Big`, an unbounded integer the translation
//! carries in its own prelude, so no value is ever narrowed; machine integers
//! stay Rust machine integers with checked arithmetic, which panics exactly
//! where the source aborts. Data types are enums whose data-typed fields are
//! boxed. Every value is owned: a variable is cloned where it is consumed.
//! Theorems cannot be proved in Rust: each becomes an executable property,
//! checked over a bounded domain by `--ml-check-theorems`, while the proof
//! stays discharged by the source kernel.
//!
//! Mirrors `js/src/translation/emit-rust.js`.

use std::fmt::Write as _;

use super::decimal::Decimal;
use super::diagnostics::{unsupported, Result};
use super::emit_common::{CtorStyle, EmitOptions, EmitState, Emitted};
use super::ir::{
    rename_function, rename_main, rename_theorem, Binder, ByZero, Decl, Effect, Expr, LitValue,
    Node, Pattern, Program, Prop, Semantics,
};
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::Type;
use super::Language;

const KEYWORDS: &[&str] = &[
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "gen",
    "union",
    "raw",
    // Names the emitted code relies on.
    "ml",
    "main",
    "std",
    "core",
    "alloc",
    "String",
    "Vec",
    "Box",
    "Option",
    "Some",
    "None",
    "Ok",
    "Err",
    "Result",
    "bool",
    "str",
    "char",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "f32",
    "f64",
    "Big",
    "Clone",
    "Debug",
    "PartialEq",
    "Eq",
    "ToString",
    "ml_main",
    "ml_check_theorems",
];

const PRELUDE: &str = r#"/// Unbounded integers for the portable core's naturals and integers.
pub mod ml {
    use std::cmp::Ordering;
    use std::fmt;

    /// Sign and magnitude; the magnitude is little-endian base 2^32 without
    /// leading zero limbs, and zero is never negative.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Big {
        negative: bool,
        magnitude: Vec<u32>,
    }

    fn trim(mut magnitude: Vec<u32>) -> Vec<u32> {
        while magnitude.last() == Some(&0) {
            magnitude.pop();
        }
        magnitude
    }

    fn compare_magnitude(a: &[u32], b: &[u32]) -> Ordering {
        a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    fn add_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for index in 0..a.len().max(b.len()) {
            let sum = u64::from(*a.get(index).unwrap_or(&0)) + u64::from(*b.get(index).unwrap_or(&0)) + carry;
            out.push(sum as u32);
            carry = sum >> 32;
        }
        if carry > 0 {
            out.push(carry as u32);
        }
        out
    }

    /// |a| - |b| where |a| >= |b|.
    fn sub_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len());
        let mut borrow = 0i64;
        for (index, limb) in a.iter().enumerate() {
            let mut difference = i64::from(*limb) - i64::from(*b.get(index).unwrap_or(&0)) - borrow;
            borrow = 0;
            if difference < 0 {
                difference += 1 << 32;
                borrow = 1;
            }
            out.push(difference as u32);
        }
        trim(out)
    }

    fn mul_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = vec![0u32; a.len() + b.len()];
        for (i, x) in a.iter().enumerate() {
            let mut carry = 0u64;
            for (j, y) in b.iter().enumerate() {
                let product = u64::from(out[i + j]) + u64::from(*x) * u64::from(*y) + carry;
                out[i + j] = product as u32;
                carry = product >> 32;
            }
            let mut index = i + b.len();
            while carry > 0 {
                let sum = u64::from(out[index]) + carry;
                out[index] = sum as u32;
                carry = sum >> 32;
                index += 1;
            }
        }
        trim(out)
    }

    /// Quotient and remainder of magnitudes; the divisor is not zero.
    fn divmod_magnitude(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut quotient = vec![0u32; a.len()];
        if b.len() == 1 {
            let divisor = u64::from(b[0]);
            let mut remainder = 0u64;
            for index in (0..a.len()).rev() {
                let current = (remainder << 32) | u64::from(a[index]);
                quotient[index] = (current / divisor) as u32;
                remainder = current % divisor;
            }
            return (trim(quotient), trim(vec![remainder as u32]));
        }
        let mut remainder: Vec<u32> = Vec::new();
        for bit in (0..a.len() * 32).rev() {
            let mut carry = (a[bit / 32] >> (bit % 32)) & 1;
            for limb in remainder.iter_mut() {
                let next = *limb >> 31;
                *limb = (*limb << 1) | carry;
                carry = next;
            }
            if carry != 0 {
                remainder.push(carry);
            }
            if compare_magnitude(&remainder, b) != Ordering::Less {
                remainder = sub_magnitude(&remainder, b);
                quotient[bit / 32] |= 1 << (bit % 32);
            }
        }
        (trim(quotient), remainder)
    }

    impl Big {
        fn from_parts(negative: bool, magnitude: Vec<u32>) -> Self {
            let magnitude = trim(magnitude);
            Self { negative: negative && !magnitude.is_empty(), magnitude }
        }

        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = match text.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, text),
            };
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        pub fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        pub fn is_negative(&self) -> bool {
            self.negative
        }

        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
        }

        /// Division rounding toward zero; the divisor is not zero.
        pub fn divmod_trunc(&self, other: &Self) -> (Self, Self) {
            let (quotient, remainder) = divmod_magnitude(&self.magnitude, &other.magnitude);
            (
                Self::from_parts(self.negative != other.negative, quotient),
                Self::from_parts(self.negative, remainder),
            )
        }
    }

    impl Ord for Big {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.negative, other.negative) {
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => compare_magnitude(&self.magnitude, &other.magnitude),
                (true, true) => compare_magnitude(&other.magnitude, &self.magnitude),
            }
        }
    }

    impl PartialOrd for Big {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl fmt::Display for Big {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_zero() {
                return formatter.write_str("0");
            }
            let billion = [1_000_000_000u32];
            let mut chunks = Vec::new();
            let mut rest = self.magnitude.clone();
            while !rest.is_empty() {
                let (quotient, remainder) = divmod_magnitude(&rest, &billion);
                chunks.push(remainder.first().copied().unwrap_or(0));
                rest = quotient;
            }
            let mut text = String::new();
            if self.negative {
                text.push('-');
            }
            text.push_str(&chunks.pop().unwrap_or(0).to_string());
            for chunk in chunks.iter().rev() {
                text.push_str(&format!("{chunk:09}"));
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts or is total (x / 0 = 0, x % 0 = x).
    pub fn divide(a: &Big, b: &Big, rounding: &str, abort_on_zero: bool, remainder: bool) -> Big {
        if b.is_zero() {
            if abort_on_zero {
                panic!("division by zero");
            }
            return if remainder { a.clone() } else { Big::zero() };
        }
        let one = Big::from_i128(1);
        let (mut quotient, rest) = a.divmod_trunc(b);
        if !rest.is_zero() {
            if rounding == "floor" && rest.is_negative() != b.is_negative() {
                quotient = quotient.sub(&one);
            }
            if rounding == "euclid" && rest.is_negative() {
                quotient = if b.is_negative() { quotient.add(&one) } else { quotient.sub(&one) };
            }
        }
        if remainder { a.sub(&quotient.mul(b)) } else { quotient }
    }

    pub fn to_nat_checked(value: Big) -> Big {
        if value.is_negative() {
            panic!("{value} is not a natural number");
        }
        value
    }

    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
    }

    /// Bounded domains for executable theorem checks.
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}"#;

/// A legal Rust identifier for a source name.
fn ident(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() || char == '_' {
                char
            } else {
                '_'
            }
        })
        .collect();
    if result.starts_with(|char: char| char.is_ascii_digit())
        || result.starts_with("__")
        || result == "_"
        || result.is_empty()
    {
        result = format!("x{result}");
    }
    if KEYWORDS.contains(&result.as_str()) {
        result.push('_');
    }
    result
}

/// `camelCase` and `HTTPServer` split into `camel_case` and `http_server`.
fn snake(name: &str) -> String {
    // `/([a-z0-9])([A-Z])/g` inserts `_` between a lower-case letter or digit and a capital.
    let chars: Vec<char> = name.chars().collect();
    let mut first = Vec::with_capacity(chars.len());
    let mut index = 0;
    while index < chars.len() {
        let char = chars[index];
        let next = chars.get(index + 1).copied();
        if (char.is_ascii_lowercase() || char.is_ascii_digit())
            && next.is_some_and(|next| next.is_ascii_uppercase())
        {
            first.extend([char, '_', next.unwrap_or(char)]);
            index += 2;
        } else {
            first.push(char);
            index += 1;
        }
    }
    // `/([A-Z]+)([A-Z][a-z])/g` splits a run of capitals before its last one
    // when a lower-case letter follows.
    let mut second = String::with_capacity(first.len() + 4);
    let mut index = 0;
    while index < first.len() {
        let run = first[index..]
            .iter()
            .take_while(|char| char.is_ascii_uppercase())
            .count();
        let after = first.get(index + run).copied();
        if run >= 2 && after.is_some_and(|after| after.is_ascii_lowercase()) {
            second.extend(&first[index..index + run - 1]);
            second.push('_');
            second.extend(&first[index + run - 1..=index + run]);
            index += run + 1;
        } else {
            second.push(first[index]);
            index += 1;
        }
    }
    ident(&second.to_lowercase())
}

/// `my_type` and `my-type` become `MyType`.
fn camel(name: &str) -> String {
    let joined: String = name
        .split(|char: char| !char.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                let mut part = first.to_uppercase().to_string();
                part.extend(chars);
                part
            })
        })
        .collect();
    ident(if joined.is_empty() { "T" } else { &joined })
}

/// Emits a checked program as Rust.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_rust(program: &Program) -> Result<Emitted> {
    let state = EmitState::new(
        program,
        Language::Rust,
        snake,
        KEYWORDS,
        EmitOptions {
            ctor_style: CtorStyle::Data,
            type_name: Some(camel),
            value_name: Some(snake),
            ctor_name: Some(camel),
            module_segment: Some(snake),
            type_space: true,
            modules_share_type_space: true,
            ..EmitOptions::default()
        },
    );
    RustEmitter {
        program,
        state,
        temporaries: 0,
        theorem_checks: Vec::new(),
        uses_big: false,
    }
    .file()
}

/// Declarations grouped by source module, in declaration order.
#[derive(Default)]
struct ModuleTree<'p> {
    items: Vec<&'p Decl>,
    modules: Vec<(String, Self)>,
}

/// A theorem the `--ml-check-theorems` runner evaluates.
#[derive(Clone)]
struct TheoremCheck {
    reference: String,
    binders: Vec<Binder>,
    source: String,
}

struct RustEmitter<'p> {
    program: &'p Program,
    state: EmitState<'p>,
    temporaries: usize,
    theorem_checks: Vec<TheoremCheck>,
    uses_big: bool,
}

const fn is_copy(ty: &Type) -> bool {
    matches!(ty, Type::Bool | Type::Fixed { .. } | Type::Unit)
}

const fn comparison_operator(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        _ => ">=",
    }
}

/// The rounding as the JavaScript runtime interpolates it.
const fn rounding_text(rounding: Option<Rounding>) -> &'static str {
    match rounding {
        Some(Rounding::Trunc) => "trunc",
        Some(Rounding::Euclid) => "euclid",
        Some(Rounding::Floor) => "floor",
        None => "undefined",
    }
}

impl<'p> RustEmitter<'p> {
    fn file(mut self) -> Result<Emitted> {
        let mut tree = ModuleTree::default();
        let program = self.program;
        for entry in &program.declarations {
            let mut node = &mut tree;
            for segment in entry.module_path() {
                let position = if let Some(position) =
                    node.modules.iter().position(|(name, _)| name == segment)
                {
                    position
                } else {
                    node.modules.push((segment.clone(), ModuleTree::default()));
                    node.modules.len() - 1
                };
                node = &mut node.modules[position].1;
            }
            node.items.push(entry);
        }
        let body = self.module_body(&tree, &[])?;
        let main = match &program.main {
            Some(main) => Some(self.main(main)?),
            None => None,
        };
        let runner = self.theorem_runner();
        let entry = self.entry(main.is_some());
        let mut lines = vec![
            format!(
                "// Translated from {} by meta-language: portable core, Rust target.",
                program.source_language.as_str()
            ),
            "#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types)]"
                .to_owned(),
            String::new(),
        ];
        if self.uses_big {
            lines.push(PRELUDE.to_owned());
            lines.push(String::new());
        }
        let has_main = main.is_some();
        for block in body.into_iter().chain(main).chain(runner).chain([entry]) {
            lines.push(block);
            lines.push(String::new());
        }
        Ok(self
            .state
            .finish(lines.join("\n"), has_main.then(|| "main".to_owned())))
    }

    fn module_body(&mut self, node: &ModuleTree<'p>, path: &[String]) -> Result<Vec<String>> {
        let mut blocks = Vec::new();
        for entry in &node.items {
            blocks.push(self.declaration(entry)?);
        }
        for (segment, child) in &node.modules {
            let mut inner_path = path.to_vec();
            inner_path.push(segment.clone());
            let inner = self.module_body(child, &inner_path)?;
            let name = self.state.module_name(&inner_path).unwrap_or_default();
            blocks.push(format!(
                "pub mod {name} {{\n{}\n}}",
                indent(&inner.join("\n\n"), 1)
            ));
        }
        Ok(blocks)
    }

    fn declaration(&mut self, entry: &'p Decl) -> Result<String> {
        match entry {
            Decl::Data(_) => Ok(self.data(entry)),
            Decl::Fn(_) => self.function(entry),
            Decl::Theorem(_) => self.theorem(entry),
        }
    }

    fn ty(&mut self, ty: &Type) -> String {
        match ty {
            Type::Nat | Type::Int => {
                self.uses_big = true;
                self.state.encode(
                    "unbounded-integers",
                    "naturals and integers are ml::Big, an arbitrary-precision integer defined in the translated program; naturals stay non-negative because natural subtraction truncates and conversions to naturals are checked",
                );
                "crate::ml::Big".to_owned()
            }
            Type::Fixed { .. } => ty.key(),
            Type::Bool => "bool".to_owned(),
            Type::String => "String".to_owned(),
            Type::Unit => "()".to_owned(),
            Type::Data { name } => format!("crate::{}", self.state.reference(name, "::")),
            other => unreachable!("no Rust type for {}", other.kind()),
        }
    }

    fn field_type(&mut self, ty: &Type) -> String {
        if matches!(ty, Type::Data { .. }) {
            format!("Box<{}>", self.ty(ty))
        } else {
            self.ty(ty)
        }
    }

    fn data(&mut self, entry: &'p Decl) -> String {
        let Decl::Data(data) = entry else {
            unreachable!("a data declaration")
        };
        let name = self.state.local_name(&data.full_name).to_owned();
        self.state.map(entry, &name);
        self.state.encode(
            "data",
            "a data type is an enum with one tuple variant per constructor; data-typed fields are boxed, and values are compared structurally",
        );
        let mut lines = vec![
            "#[derive(Clone, Debug, PartialEq, Eq)]".to_owned(),
            format!("pub enum {name} {{"),
        ];
        for ctor in &data.ctors {
            let local = self
                .state
                .ctor_local(&data.full_name, &ctor.name)
                .to_owned();
            if ctor.fields.is_empty() {
                lines.push(format!("    {local},"));
                continue;
            }
            let fields: Vec<String> = ctor
                .fields
                .iter()
                .map(|field| self.field_type(&field.ty))
                .collect();
            lines.push(format!("    {local}({}),", fields.join(", ")));
        }
        lines.push("}".to_owned());
        lines.join("\n")
    }

    fn function(&mut self, entry: &'p Decl) -> Result<String> {
        let Decl::Fn(function) = entry else {
            unreachable!("a function declaration")
        };
        let (params, body) = rename_function(function, &snake, &self.state.local_reserved());
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        let binders: Vec<String> = params
            .iter()
            .map(|param| format!("{}: {}", param.name, self.ty(&param.ty)))
            .collect();
        let ret = self.ty(&function.ret);
        let body = self.expr(&body)?;
        Ok(format!(
            "pub fn {name}({}) -> {ret} {{\n{}\n}}",
            binders.join(", "),
            indent(&body, 1)
        ))
    }

    fn theorem(&mut self, entry: &'p Decl) -> Result<String> {
        let Decl::Theorem(theorem) = entry else {
            unreachable!("a theorem declaration")
        };
        let (binders, prop, _) = rename_theorem(theorem, &snake, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), true);
        self.theorem_checks.push(TheoremCheck {
            reference: format!("crate::{}", self.state.reference(&theorem.full_name, "::")),
            binders: binders.clone(),
            source: theorem.full_name.clone(),
        });
        let params: Vec<String> = binders
            .iter()
            .map(|binder| format!("{}: {}", binder.name, self.ty(&binder.ty)))
            .collect();
        let prop = self.prop(&prop)?;
        Ok(format!(
            "/// Executable form of theorem {}.\npub fn {name}({}) -> bool {{\n{}\n}}",
            theorem.full_name,
            params.join(", "),
            indent(&prop, 1)
        ))
    }

    /// The property checked on every input of the binders' bounded domains.
    fn all_of(&mut self, binders: &[Binder], body: String) -> String {
        binders.iter().rev().fold(body, |inner, binder| {
            format!(
                "{}.into_iter().all(|{}| {inner})",
                self.domain(&binder.ty, 3),
                binder.name
            )
        })
    }

    fn theorem_runner(&mut self) -> Option<String> {
        if self.theorem_checks.is_empty() {
            return None;
        }
        self.state.encode(
            "theorem-properties",
            "each theorem is an executable property; --ml-check-theorems evaluates it on every input of a bounded domain, and its proof remains checked by the source kernel",
        );
        let checks: Vec<String> = self
            .theorem_checks
            .clone()
            .iter()
            .map(|check| {
                let args: Vec<String> = check
                    .binders
                    .iter()
                    .map(|binder| own(&binder.name, &binder.ty))
                    .collect();
                let call = format!("{}({})", check.reference, args.join(", "));
                let holds = self.all_of(&check.binders, call);
                let source = format_escape(&check.source);
                [
                    format!("    if !({holds}) {{"),
                    format!("        panic!(\"theorem {source} fails on a bounded input\");"),
                    "    }".to_owned(),
                    format!("    println!(\"theorem {source}: holds on the bounded domain\");"),
                ]
                .join("\n")
            })
            .collect();
        Some(format!(
            "fn ml_check_theorems() {{\n{}\n}}",
            checks.join("\n")
        ))
    }

    fn entry(&self, has_main: bool) -> String {
        let run = match (self.theorem_checks.is_empty(), has_main) {
            (false, true) => "if check { ml_check_theorems() } else { ml_main() }",
            (false, false) => "if check { ml_check_theorems() }",
            (true, true) => "ml_main()",
            (true, false) => "",
        };
        [
            "// Deep recursion in the source is not bounded by a small native stack.",
            "fn main() {",
            "    let check = std::env::args().any(|argument| argument == \"--ml-check-theorems\");",
            "    let worker = std::thread::Builder::new()",
            "        .stack_size(1 << 28)",
            &format!("        .spawn(move || {{ {run} }})"),
            "        .expect(\"spawn the program thread\");",
            "    if worker.join().is_err() {",
            "        std::process::exit(101);",
            "    }",
            "}",
        ]
        .join("\n")
    }

    /// Values of a type up to a constructor depth, as a Rust `Vec`.
    fn domain(&mut self, ty: &Type, depth: i32) -> String {
        match ty {
            Type::Nat => {
                self.uses_big = true;
                if depth >= 3 {
                    "crate::ml::range(0, 6)"
                } else {
                    "crate::ml::range(0, 2)"
                }
                .to_owned()
            }
            Type::Int => {
                self.uses_big = true;
                if depth >= 3 {
                    "crate::ml::range(-4, 4)"
                } else {
                    "crate::ml::range(-1, 1)"
                }
                .to_owned()
            }
            Type::Fixed { signed, .. } => {
                let key = ty.key();
                let values = if *signed { [-1, 0, 1] } else { [0, 1, 2] };
                let values: Vec<String> =
                    values.iter().map(|value| format!("{value}{key}")).collect();
                format!("vec![{}]", values.join(", "))
            }
            Type::Bool => "vec![false, true]".to_owned(),
            Type::String => {
                "vec![String::new(), String::from(\"a\"), String::from(\"ab\")]".to_owned()
            }
            Type::Unit => "vec![()]".to_owned(),
            Type::Data { name } => {
                let program = self.program;
                let entry = program.data(name);
                let mut lines = vec![format!(
                    "let mut values: Vec<{}> = Vec::new();",
                    self.ty(ty)
                )];
                for ctor in &entry.ctors {
                    let recursive = ctor
                        .fields
                        .iter()
                        .any(|field| matches!(field.ty, Type::Data { .. }));
                    if recursive && depth <= 1 {
                        continue;
                    }
                    let names: Vec<String> = (0..ctor.fields.len())
                        .map(|index| format!("f{index}"))
                        .collect();
                    let args: Vec<String> = ctor
                        .fields
                        .iter()
                        .zip(&names)
                        .map(|(field, name)| {
                            if matches!(field.ty, Type::Data { .. }) {
                                format!("Box::new({name}.clone())")
                            } else {
                                format!("{name}.clone()")
                            }
                        })
                        .collect();
                    let head = format!(
                        "crate::{}",
                        self.state.ctor_ref(&entry.full_name, &ctor.name, "::")
                    );
                    let mut statement = if args.is_empty() {
                        format!("values.push({head});")
                    } else {
                        format!("values.push({head}({}));", args.join(", "))
                    };
                    for (field, name) in ctor.fields.iter().zip(&names).rev() {
                        let inner_depth = if matches!(field.ty, Type::Data { .. }) {
                            depth - 1
                        } else {
                            1
                        };
                        statement = format!(
                            "for {name} in {} {{\n{}\n}}",
                            self.domain(&field.ty, inner_depth),
                            indent(&statement, 1)
                        );
                    }
                    lines.push(statement);
                }
                lines.push("values".to_owned());
                block(&lines.join("\n"))
            }
            other => unreachable!("no Rust domain for {}", other.kind()),
        }
    }

    fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let body = self.prop(body)?;
                self.all_of(binders, body)
            }
            Prop::And { left, right } => {
                format!("({} && {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Or { left, right } => {
                format!("({} || {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Implies { left, right } => {
                format!("(!{} || {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Not { arg } => format!("!{}", self.prop(arg)?),
            Prop::Bool { expr } => self.expr(expr)?,
            other => {
                let (op, comparison) = other.comparison().expect("a comparison");
                format!(
                    "({} {} {})",
                    self.borrow(&comparison.left)?,
                    comparison_operator(op),
                    self.borrow(&comparison.right)?
                )
            }
        })
    }

    /// An expression whose value is only read: variables are borrowed, not cloned.
    fn borrow(&mut self, expr: &Expr) -> Result<String> {
        match expr.var_name() {
            Some(name) => Ok(format!("&{name}")),
            None => Ok(format!("&{}", self.expr(expr)?)),
        }
    }

    /// A method receiver: `&self` methods borrow a variable in place.
    fn receiver(&mut self, expr: &Expr) -> Result<String> {
        match expr.var_name() {
            Some(name) => Ok(name.to_owned()),
            None => Ok(format!("({})", self.expr(expr)?)),
        }
    }

    fn expr(&mut self, expr: &Expr) -> Result<String> {
        match &expr.node {
            Node::Lit { value } => Ok(self.literal(expr, value)),
            Node::Unit => Ok("()".to_owned()),
            Node::Var { name } => Ok(own(name, &expr.ty)),
            Node::Call { func, args } => {
                let head = format!("crate::{}", self.state.reference(func, "::"));
                let mut texts = Vec::with_capacity(args.len());
                for arg in args {
                    texts.push(self.expr(arg)?);
                }
                Ok(format!("{head}({})", texts.join(", ")))
            }
            Node::Ctor { data, ctor, args } => {
                let program = self.program;
                let entry = program.data(data);
                let declared = entry
                    .ctors
                    .iter()
                    .find(|candidate| candidate.name == *ctor)
                    .expect("the checker resolves every constructor");
                let head = format!("crate::{}", self.state.ctor_ref(data, ctor, "::"));
                if args.is_empty() {
                    return Ok(head);
                }
                let mut texts = Vec::with_capacity(args.len());
                for (arg, field) in args.iter().zip(&declared.fields) {
                    let text = self.expr(arg)?;
                    texts.push(if matches!(field.ty, Type::Data { .. }) {
                        format!("Box::new({text})")
                    } else {
                        text
                    });
                }
                Ok(format!("{head}({})", texts.join(", ")))
            }
            Node::Unary { op, arg, .. } => {
                if *op == UnaryOp::Not {
                    return Ok(format!("!{}", self.receiver(arg)?));
                }
                if matches!(expr.ty, Type::Fixed { .. }) {
                    let text = format!("{}.checked_neg()", self.receiver(arg)?);
                    return Ok(self.checked(&text, &expr.ty, "negation"));
                }
                Ok(format!("{}.neg()", self.receiver(arg)?))
            }
            Node::Binary { .. } => self.binary(expr),
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond)?;
                let then = self.expr(then)?;
                let otherwise = self.expr(otherwise)?;
                Ok(format!(
                    "if {cond} {{\n{}\n}} else {{\n{}\n}}",
                    indent(&then, 1),
                    indent(&otherwise, 1)
                ))
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value)?;
                let body = self.expr(body)?;
                Ok(block(&format!("let {name} = {value};\n{body}")))
            }
            Node::Match { .. } => self.match_expr(expr),
            Node::ToString { arg } => {
                match arg.ty {
                    Type::String => return self.expr(arg),
                    Type::Data { .. } | Type::Unit => {
                        return Err(unsupported(
                            "output of structured values",
                            &format!("a {} value has no portable textual form", arg.ty.kind()),
                            arg.span,
                        ));
                    }
                    _ => {}
                }
                Ok(format!("{}.to_string()", self.receiver(arg)?))
            }
            Node::Cast { .. } => self.cast(expr),
            Node::Abort { message } => Ok(format!("panic!(\"{{}}\", {})", rust_string(message))),
        }
    }

    fn literal(&mut self, expr: &Expr, value: &LitValue) -> String {
        let text = value.text();
        match &expr.ty {
            Type::Nat | Type::Int => {
                self.uses_big = true;
                let bound = 1i128 << 126;
                let small = Decimal::parse(&text)
                    .and_then(|value| value.to_i128())
                    .is_some_and(|value| value >= -bound && value < bound);
                if small {
                    format!("crate::ml::Big::from_i128({text})")
                } else {
                    format!("crate::ml::Big::parse(\"{text}\")")
                }
            }
            Type::Fixed { .. } => {
                if text.starts_with('-') {
                    format!("({text}{})", expr.ty.key())
                } else {
                    format!("{text}{}", expr.ty.key())
                }
            }
            Type::Bool => text,
            Type::String => format!("String::from({})", rust_string(&text)),
            other => unreachable!("no Rust literal for {}", other.kind()),
        }
    }

    fn checked(&mut self, text: &str, ty: &Type, what: &str) -> String {
        let key = ty.key();
        self.state.encode(
            &format!("machine-integer:{key}"),
            &format!("{key} stays a Rust {key}; its arithmetic is checked and panics where the source aborts"),
        );
        format!("{text}.expect(\"{key} {what} overflowed\")")
    }

    fn binary(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op, left, right, ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        match op {
            BinaryOp::And => Ok(format!("({} && {})", self.expr(left)?, self.expr(right)?)),
            BinaryOp::Or => Ok(format!("({} || {})", self.expr(left)?, self.expr(right)?)),
            BinaryOp::Concat => Ok(format!(
                "format!(\"{{}}{{}}\", {}, {})",
                self.receiver(left)?,
                self.receiver(right)?
            )),
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => Ok(format!(
                "({} {} {})",
                self.borrow(left)?,
                comparison_operator(*op),
                self.borrow(right)?
            )),
            _ if matches!(expr.ty, Type::Fixed { .. }) => self.fixed_arithmetic(expr),
            _ => self.big_arithmetic(expr),
        }
    }

    fn fixed_arithmetic(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            rounding,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let left = self.receiver(left)?;
        let right = self.expr(right)?;
        let (method, what) = match op {
            BinaryOp::Add => ("checked_add", "addition"),
            BinaryOp::Sub => ("checked_sub", "subtraction"),
            BinaryOp::Mul => ("checked_mul", "multiplication"),
            BinaryOp::Div => ("checked_div", "division"),
            BinaryOp::Rem => ("checked_rem", "remainder"),
            other => unreachable!("no Rust machine-integer operator {other:?}"),
        };
        let division = matches!(op, BinaryOp::Div | BinaryOp::Rem);
        let signed = matches!(expr.ty, Type::Fixed { signed: true, .. });
        let mut method = method.to_owned();
        if division && *rounding == Some(Rounding::Euclid) {
            method.push_str("_euclid");
        } else if division && *rounding != Some(Rounding::Trunc) && signed {
            return Err(unsupported(
                &format!("{} division on {}", rounding_text(*rounding), expr.ty.key()),
                "Rust machine integers divide with truncating or Euclidean rounding only",
                expr.span,
            ));
        }
        if division {
            self.state.abort_to_total("division by zero");
        }
        Ok(self.checked(&format!("{left}.{method}({right})"), &expr.ty, what))
    }

    fn big_arithmetic(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left: left_expr,
            right: right_expr,
            semantics,
            rounding,
            by_zero,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let left = self.receiver(left_expr)?;
        let right = self.borrow(right_expr)?;
        match op {
            BinaryOp::Add => Ok(format!("{left}.add({right})")),
            BinaryOp::Mul => Ok(format!("{left}.mul({right})")),
            BinaryOp::Sub => {
                if *semantics == Some(Semantics::Truncated) {
                    return Ok(format!(
                        "crate::ml::nat_sub({}, {right})",
                        self.borrow(left_expr)?
                    ));
                }
                Ok(format!("{left}.sub({right})"))
            }
            BinaryOp::Div | BinaryOp::Rem => {
                let abort = *by_zero == Some(ByZero::Abort);
                if abort {
                    self.state.abort_to_total("division by zero");
                }
                Ok(format!(
                    "crate::ml::divide({}, {right}, \"{}\", {abort}, {})",
                    self.borrow(left_expr)?,
                    rounding_text(*rounding),
                    *op == BinaryOp::Rem
                ))
            }
            other => unreachable!("no Rust operator {other:?}"),
        }
    }

    fn match_expr(&mut self, expr: &Expr) -> Result<String> {
        let Node::Match { scrutinee, cases } = &expr.node else {
            unreachable!("a match expression")
        };
        let fallback = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::Wild | Pattern::Bind { .. }));
        if let Type::Data { name } = &scrutinee.ty {
            let program = self.program;
            let entry = program.data(name);
            let mut arms = Vec::with_capacity(cases.len());
            for kase in cases {
                let arm = match &kase.pattern {
                    Pattern::Wild => format!("_ => {}", block(&self.expr(&kase.body)?)),
                    Pattern::Bind { name } => {
                        format!("{name} => {}", block(&self.expr(&kase.body)?))
                    }
                    Pattern::Ctor { data, ctor, binds } => {
                        let declared = entry
                            .ctors
                            .iter()
                            .find(|candidate| candidate.name == *ctor)
                            .expect("the checker resolves every constructor");
                        let head = format!("crate::{}", self.state.ctor_ref(data, ctor, "::"));
                        let names: Vec<&str> = binds
                            .iter()
                            .map(|bind| bind.as_deref().unwrap_or("_"))
                            .collect();
                        let mut lines: Vec<String> = binds
                            .iter()
                            .zip(&declared.fields)
                            .filter_map(|(bind, field)| match bind {
                                Some(bind) if matches!(field.ty, Type::Data { .. }) => {
                                    Some(format!("let {bind} = *{bind};"))
                                }
                                _ => None,
                            })
                            .collect();
                        lines.push(self.expr(&kase.body)?);
                        let text = if names.is_empty() {
                            head
                        } else {
                            format!("{head}({})", names.join(", "))
                        };
                        format!("{text} => {}", block(&lines.join("\n")))
                    }
                    Pattern::NatZero | Pattern::NatSucc { .. } => {
                        unreachable!("a natural pattern on a data type")
                    }
                };
                arms.push(arm);
            }
            return Ok(format!(
                "match {} {{\n{}\n}}",
                self.expr(scrutinee)?,
                indent(&arms.join("\n"), 1)
            ));
        }
        let natural = cases
            .iter()
            .any(|kase| matches!(kase.pattern, Pattern::NatZero | Pattern::NatSucc { .. }));
        if scrutinee.ty != Type::Nat && !natural {
            let kase = fallback.expect("a match without constructors has a fallback case");
            let pattern = match &kase.pattern {
                Pattern::Bind { name } => name.as_str(),
                _ => "_",
            };
            let subject = self.expr(scrutinee)?;
            let body = block(&self.expr(&kase.body)?);
            return Ok(format!(
                "match {subject} {{\n{}\n}}",
                indent(&format!("{pattern} => {body}"), 1)
            ));
        }
        let mut lines = Vec::new();
        let subject = if let Some(name) = scrutinee.var_name() {
            name.to_owned()
        } else {
            self.temporaries += 1;
            let subject = format!("__subject{}", self.temporaries);
            let value = self.expr(scrutinee)?;
            lines.push(format!("let {subject} = {value};"));
            subject
        };
        let zero = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::NatZero));
        let succ = cases.iter().find_map(|kase| match &kase.pattern {
            Pattern::NatSucc { name } => Some((name, &kase.body)),
            _ => None,
        });
        let zero_text = match zero {
            Some(zero) => self.expr(&zero.body)?,
            None => self.fallback_body(fallback, &subject)?,
        };
        let machine = matches!(scrutinee.ty, Type::Fixed { .. });
        // On an unsigned machine integer the successor case holds a value of at least 1, so `- 1` cannot wrap.
        let predecessor = if machine {
            format!("{subject} - 1")
        } else {
            format!("crate::ml::pred(&{subject})")
        };
        let succ_text = match succ {
            Some((name, body)) => format!("let {name} = {predecessor};\n{}", self.expr(body)?),
            None => self.fallback_body(fallback, &subject)?,
        };
        let test = if machine {
            format!("{subject} == 0")
        } else {
            format!("{subject}.is_zero()")
        };
        lines.push(format!(
            "if {test} {{\n{}\n}} else {{\n{}\n}}",
            indent(&zero_text, 1),
            indent(&succ_text, 1)
        ));
        if lines.len() == 1 {
            return Ok(lines.remove(0));
        }
        Ok(block(&lines.join("\n")))
    }

    fn fallback_body(
        &mut self,
        fallback: Option<&super::ir::Case>,
        subject: &str,
    ) -> Result<String> {
        let kase = fallback.expect("a match missing a natural case has a fallback case");
        let mut lines = Vec::new();
        if let Pattern::Bind { name } = &kase.pattern {
            lines.push(format!("let {name} = {subject}.clone();"));
        }
        lines.push(self.expr(&kase.body)?);
        Ok(lines.join("\n"))
    }

    fn cast(&mut self, expr: &Expr) -> Result<String> {
        let Node::Cast {
            arg,
            from,
            to,
            flavor,
        } = &expr.node
        else {
            unreachable!("a cast")
        };
        let arg = self.expr(arg)?;
        if matches!(from, Type::Fixed { .. }) && matches!(to, Type::Fixed { .. }) {
            return Ok(format!("{}::from({arg})", to.key()));
        }
        match flavor {
            Flavor::Exact => Ok(arg),
            Flavor::Clamp => Ok(format!("crate::ml::clamp_nat({arg})")),
            Flavor::Checked => {
                self.state.checked_to_total("conversion to a natural");
                Ok(format!("crate::ml::to_nat_checked({arg})"))
            }
        }
    }

    fn main(&mut self, main: &super::ir::Main) -> Result<String> {
        let effects = rename_main(main, &snake, &self.state.local_reserved());
        let mut lines = Vec::with_capacity(effects.len());
        let mut assertion = 0;
        for effect in &effects {
            match effect {
                Effect::Print { expr, .. } => {
                    lines.push(format!("println!(\"{{}}\", {});", self.expr(expr)?));
                }
                Effect::Let { name, value, .. } => {
                    lines.push(format!("let {name} = {};", self.expr(value)?));
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    lines.push(format!(
                        "assert!({}, \"assertion {assertion}\");",
                        self.prop(prop)?
                    ));
                    self.state
                        .assertion_theorem(&format!("assertion {assertion}"), effect);
                }
            }
        }
        self.state.encode(
            "program-output",
            "main prints the lines the source program prints, in order, with println!",
        );
        Ok(format!(
            "fn ml_main() {{\n{}\n}}",
            indent(&lines.join("\n"), 1)
        ))
    }
}

/// An owned copy of a variable.
fn own(name: &str, ty: &Type) -> String {
    if is_copy(ty) {
        name.to_owned()
    } else {
        format!("{name}.clone()")
    }
}

fn block(text: &str) -> String {
    format!("{{\n{}\n}}", indent(text, 1))
}

fn rust_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for char in text.chars() {
        match char {
            '"' | '\\' => {
                out.push('\\');
                out.push(char);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ if (char as u32) < 0x20 || char as u32 == 0x7f => {
                let _ = write!(out, "\\u{{{:x}}}", char as u32);
            }
            _ => out.push(char),
        }
    }
    out
}

/// Text inside a `format!`-style literal, where braces are doubled.
fn format_escape(text: &str) -> String {
    rust_escape(text).replace('{', "{{").replace('}', "}}")
}

fn rust_string(text: &str) -> String {
    format!("\"{}\"", rust_escape(text))
}

fn indent(text: &str, depth: usize) -> String {
    let pad = "    ".repeat(depth);
    text.split('\n')
        .map(|line| {
            if line.is_empty() {
                line.to_owned()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
