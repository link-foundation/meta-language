//! Surface expression variants, reexported at their original public path.

use serde::{Deserialize, Serialize};

use super::{
    BinaryOp, ConstructorPrototypeDefinition, Flavor, Rounding, SArrayItem, SCase, SExpr,
    SRecordEntry, SRow, ShowStyle, SurfaceBindingTypeAnnotation, SurfaceRawMetadata, UnaryOp,
    binding_type_annotation, is_false, ordered_fields, raw_metadata,
};
use crate::translation::types::Type;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SNode {
    Num {
        value: String,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
        #[serde(default, skip_serializing_if = "is_false")]
        negative: bool,
        /// The 1 of `x++`, which takes the type of `x`.
        #[serde(default, skip_serializing_if = "is_false")]
        unit: bool,
    },
    Bool {
        value: bool,
    },
    Str {
        value: String,
    },
    Unit,
    /// An explicitly empty optional value with its original absence tag.
    None {
        absence: String,
    },
    /// Internal checked reconstruction of an explicitly present optional value.
    Some {
        value: Box<SExpr>,
        #[serde(rename = "type")]
        ty: Type,
    },
    /// Internal reconstruction of the represented JavaScript numeric conversion.
    NumberValue {
        arg: Box<SExpr>,
    },
    FlattenOption {
        value: Box<SExpr>,
    },
    Coalesce {
        left: Box<SExpr>,
        right: Box<SExpr>,
    },
    ArrayBindingRest {
        object: Box<SExpr>,
        start: serde_json::Value,
        #[serde(flatten)]
        extra: std::collections::BTreeMap<String, serde_json::Value>,
    },
    ArrayBindingDefault {
        value: Box<SExpr>,
        fallback: Box<SExpr>,
        #[serde(flatten)]
        extra: std::collections::BTreeMap<String, serde_json::Value>,
    },
    OptionalField {
        object: Box<SExpr>,
        field: String,
    },
    Name {
        path: Vec<String>,
    },
    FunctionValue {
        path: Vec<String>,
    },
    Lambda {
        params: Vec<super::SParam>,
        ret: Option<Type>,
        #[serde(default)]
        captures: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assigned_captures: Option<Vec<String>>,
        body: Box<SExpr>,
    },
    CellAllocate {
        value: Box<SExpr>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
    },
    CellRead {
        cell: Box<SExpr>,
    },
    CellWrite {
        cell: Box<SExpr>,
        value: Box<SExpr>,
        body: Box<SExpr>,
    },
    /// Lean's `.ctor`, resolved against the expected type.
    DotCtor {
        name: String,
    },
    App {
        #[serde(rename = "fn")]
        func: Box<SExpr>,
        args: Vec<SExpr>,
    },
    Field {
        object: Box<SExpr>,
        field: String,
    },
    #[serde(deserialize_with = "super::record_projection::read_record_projection")]
    Record {
        entries: Vec<SRecordEntry>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        record_projection: Option<serde_json::Value>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prototype_definition: Option<ConstructorPrototypeDefinition>,
    },
    TypeOf {
        arg: Box<SExpr>,
    },
    Unary {
        op: UnaryOp,
        arg: Box<SExpr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<SExpr>,
        right: Box<SExpr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rounding: Option<Rounding>,
    },
    If {
        cond: Box<SExpr>,
        then: Box<SExpr>,
        #[serde(rename = "else")]
        otherwise: Box<SExpr>,
    },
    Let {
        name: String,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "binding_type_annotation::read_present_binding_type_annotation"
        )]
        ty: Option<SurfaceBindingTypeAnnotation>,
        value: Box<SExpr>,
        body: Box<SExpr>,
    },
    Match {
        scrutinees: Vec<SExpr>,
        rows: Vec<SRow>,
    },
    /// A single-level match, as pattern compilation produces it.
    Match1 {
        scrutinee: Box<SExpr>,
        cases: Vec<SCase>,
    },
    StringMap {
        op: String,
        object: Box<SExpr>,
    },
    CopySlice {
        object: Box<SExpr>,
    },
    Method {
        object: Box<SExpr>,
        op: String,
        args: Vec<Option<SExpr>>,
    },
    StringTest {
        op: String,
        object: Box<SExpr>,
        search: Box<SExpr>,
    },
    ToString {
        arg: Box<SExpr>,
    },
    Show {
        arg: Box<SExpr>,
        style: ShowStyle,
    },
    Cast {
        arg: Box<SExpr>,
        to: Type,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Type>,
        flavor: Flavor,
    },
    Abort {
        message: String,
    },
    /// `console.log(expr)`, then `body`.
    Print {
        expr: Box<SExpr>,
        style: ShowStyle,
        body: Box<SExpr>,
    },
    /// A JavaScript `{ $: 'tag', field: value }` constructor object, fields in source order.
    CtorObject {
        tag: String,
        #[serde(with = "ordered_fields")]
        fields: Vec<(String, SExpr)>,
        #[serde(default, skip_serializing_if = "is_false")]
        frozen: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prototype_definition: Option<ConstructorPrototypeDefinition>,
    },
    /// A JavaScript array literal `[a, ...xs, b]`.
    Array {
        items: Vec<SArrayItem>,
        /// The element type inference fixed for a literal with no element of its own.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        element: Option<Type>,
    },
    Set {
        items: Box<SExpr>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
    },
    Map {
        entries: Box<SExpr>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
    },
    /// An explicitly admitted positional product. The flag retains its value ABI.
    Tuple {
        items: Vec<SExpr>,
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
    /// A constant positional projection, distinct from array indexing.
    TupleIndex {
        object: Box<SExpr>,
        index: usize,
    },
    /// `xs[i]`, a read of an array element.
    Index {
        object: Box<SExpr>,
        index: Box<SExpr>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "raw_metadata::read_present_surface_raw_metadata"
        )]
        optional: Option<SurfaceRawMetadata>,
        #[serde(
            rename = "provenInBounds",
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "raw_metadata::read_present_surface_raw_metadata"
        )]
        proven_in_bounds: Option<SurfaceRawMetadata>,
    },
    /// The actual iterator selected by a source for-of statement.
    ForOfIterable {
        object: Box<SExpr>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
    },
    ArrayFrom {
        args: Vec<SExpr>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        ty: Option<Type>,
    },
    StringPoints {
        object: Box<SExpr>,
    },
    ReferenceWrite {
        object: Box<SExpr>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        index: Option<Box<SExpr>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        field: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<Box<SExpr>>,
        op: String,
    },
    /// The length of an array: a Number, or for the counter of a for…of loop a `BigInt`.
    Length {
        object: Box<SExpr>,
        integer: bool,
    },
    /// A call of a Math or Number function, `Math.max(a, ...xs)`: `op` is
    /// the function and `name` the path the program called it by.
    Math {
        op: String,
        name: String,
        args: Vec<SArrayItem>,
    },
    // Frontend-internal forms; the checker rejects them if they reach it.
    Cons {
        head: Box<SExpr>,
        tail: Box<SExpr>,
    },
    Nil,
    List {
        items: Vec<SExpr>,
    },
    Tag {
        tag: String,
    },
    Default,
}

impl SNode {
    /// The `k` discriminant.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Num { .. } => "num",
            Self::Bool { .. } => "bool",
            Self::Str { .. } => "str",
            Self::Unit => "unit",
            Self::None { .. } => "none",
            Self::Some { .. } => "some",
            Self::NumberValue { .. } => "numberValue",
            Self::FlattenOption { .. } => "flattenOption",
            Self::Coalesce { .. } => "coalesce",
            Self::ArrayBindingRest { .. } => "arrayBindingRest",
            Self::ArrayBindingDefault { .. } => "arrayBindingDefault",
            Self::OptionalField { .. } => "optionalField",
            Self::Name { .. } => "name",
            Self::FunctionValue { .. } => "functionValue",
            Self::Lambda { .. } => "lambda",
            Self::CellAllocate { .. } => "cellAllocate",
            Self::CellRead { .. } => "cellRead",
            Self::CellWrite { .. } => "cellWrite",
            Self::DotCtor { .. } => "dotCtor",
            Self::App { .. } => "app",
            Self::Field { .. } => "field",
            Self::Record { .. } => "record",
            Self::TypeOf { .. } => "typeOf",
            Self::Unary { .. } => "unary",
            Self::Binary { .. } => "binary",
            Self::If { .. } => "if",
            Self::Let { .. } => "let",
            Self::Match { .. } => "match",
            Self::Match1 { .. } => "match1",
            Self::StringMap { .. } => "stringMap",
            Self::CopySlice { .. } => "copySlice",
            Self::Method { .. } => "method",
            Self::StringTest { .. } => "stringTest",
            Self::ToString { .. } => "toString",
            Self::Show { .. } => "show",
            Self::Cast { .. } => "cast",
            Self::Abort { .. } => "abort",
            Self::Print { .. } => "print",
            Self::CtorObject { .. } => "ctorObject",
            Self::Array { .. } => "array",
            Self::Set { .. } => "set",
            Self::Map { .. } => "map",
            Self::Tuple { .. } => "tuple",
            Self::TupleIndex { .. } => "tupleIndex",
            Self::Index { .. } => "index",
            Self::ForOfIterable { .. } => "forOfIterable",
            Self::ArrayFrom { .. } => "arrayFrom",
            Self::StringPoints { .. } => "stringPoints",
            Self::ReferenceWrite { .. } => "referenceWrite",
            Self::Length { .. } => "length",
            Self::Math { .. } => "math",
            Self::Cons { .. } => "cons",
            Self::Nil => "nil",
            Self::List { .. } => "list",
            Self::Tag { .. } => "tag",
            Self::Default => "default",
        }
    }
}
