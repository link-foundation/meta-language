//! Checked expression variants, reexported at their original public path.

use serde::{Deserialize, Serialize};

use super::{
    BinaryOp, ByZero, Case, Expr, Flavor, LitValue, RecordEntry, Rounding, Semantics, Type,
    UnaryOp, is_false,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Node {
    Lit {
        value: LitValue,
    },
    Unit,
    /// An empty optional value; its element and absence policy remain on the type.
    None {
        absence: String,
    },
    Some {
        value: Box<Expr>,
    },
    FlattenOption {
        value: Box<Expr>,
    },
    Coalesce {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    OptionalField {
        object: Box<Expr>,
        field: String,
        index: usize,
    },
    OptionalProject {
        object: Box<Expr>,
        field: String,
    },
    NullishNumber {
        arg: Box<Expr>,
    },
    OptionalText {
        arg: Box<Expr>,
        #[serde(default)]
        console: bool,
    },
    OptionalTypeOf {
        arg: Box<Expr>,
        present: String,
    },
    Truthy {
        arg: Box<Expr>,
    },
    StrictEqual {
        left: Box<Expr>,
        right: Box<Expr>,
        negate: bool,
    },
    UnwrapOption {
        arg: Box<Expr>,
    },
    OptionalPredicate {
        arg: Box<Expr>,
        op: String,
    },
    Var {
        name: String,
    },
    FunctionRef {
        #[serde(rename = "fn")]
        func: String,
    },
    Apply {
        #[serde(rename = "fn")]
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Lambda {
        params: Vec<super::LambdaParam>,
        body: Box<Expr>,
    },
    CellAllocate {
        value: Box<Expr>,
    },
    CellRead {
        cell: Box<Expr>,
    },
    CellWrite {
        cell: Box<Expr>,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    Call {
        #[serde(rename = "fn")]
        func: String,
        args: Vec<Expr>,
    },
    Ctor {
        data: String,
        ctor: String,
        args: Vec<Expr>,
        #[serde(default, skip_serializing_if = "is_false")]
        frozen: bool,
    },
    Unary {
        op: UnaryOp,
        arg: Box<Expr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantics: Option<Semantics>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        /// The operand type of comparisons and arithmetic.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        domain: Option<Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantics: Option<Semantics>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rounding: Option<Rounding>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        by_zero: Option<ByZero>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        #[serde(rename = "else")]
        otherwise: Box<Expr>,
    },
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    Match {
        scrutinee: Box<Expr>,
        cases: Vec<Case>,
    },
    StringMap {
        op: String,
        string: Box<Expr>,
    },
    Method {
        object: Box<Expr>,
        op: String,
        args: Vec<Option<Expr>>,
    },
    StringTest {
        op: String,
        string: Box<Expr>,
        search: Box<Expr>,
    },
    ToString {
        arg: Box<Expr>,
        /// `console.log` of a Number, which prints -0 as "-0" where `String(-0)` is "0".
        #[serde(default, skip_serializing_if = "is_false")]
        console: bool,
    },
    Cast {
        arg: Box<Expr>,
        from: Type,
        to: Type,
        flavor: Flavor,
        /// The message of the guard a checked conversion to a parameter is.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        /// The place of that guard among the callee's guards.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        order: Option<usize>,
    },
    Abort {
        message: String,
    },
    /// Prints `text`, then is `body`.
    Print {
        text: Box<Expr>,
        body: Box<Expr>,
    },
    /// A JavaScript array of `items`.
    Array {
        items: Vec<Expr>,
    },
    Record {
        entries: Vec<RecordEntry>,
    },
    RecordField {
        object: Box<Expr>,
        field: String,
        index: usize,
    },
    /// Positionally typed elements; `frozen` marks the immutable product ABI.
    Tuple {
        items: Vec<Expr>,
        #[serde(default, skip_serializing_if = "is_false")]
        frozen: bool,
        #[serde(
            rename = "productModel",
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::translation::types::deserialize_tuple_product_model"
        )]
        product_model: Option<String>,
    },
    /// A constant projection from a positional product.
    TupleIndex {
        object: Box<Expr>,
        index: usize,
    },
    /// The elements of `left`, then those of `right`.
    Append {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// The element of `array` at `index`, a Number or an integer; a read
    /// outside the array aborts.
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
        #[serde(rename = "provenInBounds", default, skip_serializing_if = "is_false")]
        proven_in_bounds: bool,
    },
    /// An array read that retains JavaScript's undefined result outside bounds.
    OptionalIndex {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    /// Select a numeric array property from a present Number or absence.
    OptionalIndexNumber {
        arg: Box<Expr>,
    },
    /// Iterate a string by Unicode code points, preserving receiver evaluation.
    StringPoints {
        object: Box<Expr>,
    },
    /// Read entries, keys or values from an immutable insertion-order collection.
    CollectionItems {
        object: Box<Expr>,
        op: String,
    },
    /// The number of elements of `array`, as a Number or an integer.
    Length {
        array: Box<Expr>,
    },
    /// Allocate the representation selected by the source reference rules.
    ReferenceAllocate {
        value: Box<Expr>,
    },
    /// Observe the selected shared representation through its original owner.
    ReferenceRead {
        reference: Box<Expr>,
    },
    /// A `Math` or `Number` function of Numbers; `maxOf` and `minOf` fold
    /// `max` and `min` over one array.
    Math {
        op: String,
        args: Vec<Expr>,
    },
    /// No lines printed (see `output`).
    OutNil,
    /// The lines `tail` with `head` printed after them.
    OutCons {
        head: Box<Expr>,
        tail: Box<Expr>,
    },
}

impl Node {
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Lit { .. } => "lit",
            Self::Unit => "unit",
            Self::None { .. } => "none",
            Self::Some { .. } => "some",
            Self::FlattenOption { .. } => "flattenOption",
            Self::Coalesce { .. } => "coalesce",
            Self::OptionalField { .. } => "optionalField",
            Self::OptionalProject { .. } => "optionalProject",
            Self::NullishNumber { .. } => "nullishNumber",
            Self::OptionalText { .. } => "optionalText",
            Self::OptionalTypeOf { .. } => "optionalTypeOf",
            Self::Truthy { .. } => "truthy",
            Self::StrictEqual { .. } => "strictEqual",
            Self::UnwrapOption { .. } => "unwrapOption",
            Self::OptionalPredicate { .. } => "optionalPredicate",
            Self::Var { .. } => "var",
            Self::FunctionRef { .. } => "functionRef",
            Self::Apply { .. } => "apply",
            Self::Lambda { .. } => "lambda",
            Self::CellAllocate { .. } => "cellAllocate",
            Self::CellRead { .. } => "cellRead",
            Self::CellWrite { .. } => "cellWrite",
            Self::Call { .. } => "call",
            Self::Ctor { .. } => "ctor",
            Self::Unary { .. } => "unary",
            Self::Binary { .. } => "binary",
            Self::If { .. } => "if",
            Self::Let { .. } => "let",
            Self::Match { .. } => "match",
            Self::StringMap { .. } => "stringMap",
            Self::Method { .. } => "method",
            Self::StringTest { .. } => "stringTest",
            Self::ToString { .. } => "toString",
            Self::Cast { .. } => "cast",
            Self::Abort { .. } => "abort",
            Self::Print { .. } => "print",
            Self::Array { .. } => "array",
            Self::Record { .. } => "record",
            Self::RecordField { .. } => "recordField",
            Self::Tuple { .. } => "tuple",
            Self::TupleIndex { .. } => "tupleIndex",
            Self::Append { .. } => "append",
            Self::Index { .. } => "index",
            Self::OptionalIndex { .. } => "optionalIndex",
            Self::OptionalIndexNumber { .. } => "optionalIndexNumber",
            Self::StringPoints { .. } => "stringPoints",
            Self::CollectionItems { .. } => "collectionItems",
            Self::Length { .. } => "length",
            Self::ReferenceAllocate { .. } => "referenceAllocate",
            Self::ReferenceRead { .. } => "referenceRead",
            Self::Math { .. } => "math",
            Self::OutNil => "outNil",
            Self::OutCons { .. } => "outCons",
        }
    }
}
