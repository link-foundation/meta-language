// Generated from the shared typed-program-links schema through
// js/scripts/generate-typed-program-link-rules.mjs. Do not edit by hand.
/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_container_kind(term: &str) -> String {
    if term == "program record" {
        String::from("record")
    } else if term == "ordered program list" {
        String::from("array")
    } else if term == "program mapping" {
        String::from("map")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_container_fields(kind: &str, has_fields: bool) -> bool {
    if kind == "record" {
        true
    } else if kind == "array" || kind == "map" {
        !has_fields
    } else {
        false
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_scalar_kind(term: &str) -> String {
    if term == "string value" {
        String::from("string")
    } else if term == "number value" {
        String::from("number")
    } else if term == "boolean value" {
        String::from("boolean")
    } else if term == "integer value" {
        String::from("bigint")
    } else if term == "null value" {
        String::from("null")
    } else if term == "undefined value" {
        String::from("undefined")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_reference_role(term: &str) -> String {
    if term == "binding reference" {
        String::from("binding")
    } else if term == "declaration reference" {
        String::from("declaration")
    } else if term == "data type declaration reference" {
        String::from("datatype")
    } else if term == "constructor declaration reference" {
        String::from("constructor")
    } else if term == "datatype declaration reference" {
        String::from("datatype")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_pattern_binding_form(kind: &str) -> String {
    if kind == "wild" {
        String::from("none")
    } else if kind == "bind" {
        String::from("single")
    } else if kind == "natZero" {
        String::from("none")
    } else if kind == "natSucc" {
        String::from("single")
    } else if kind == "ctor" {
        String::from("fields")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_expression_operand_fields(kind: &str) -> String {
    if ((kind == "call" || kind == "ctor") || kind == "math") || kind == "recur" {
        String::from("args")
    } else if kind == "apply" {
        String::from("fn args")
    } else if kind == "method" {
        String::from("object args")
    } else if kind == "cellAllocate" {
        String::from("value")
    } else if kind == "cellRead" {
        String::from("cell")
    } else if kind == "cellWrite" {
        String::from("cell value body")
    } else if kind == "referenceAllocate" {
        String::from("value")
    } else if kind == "referenceRead" {
        String::from("reference")
    } else if kind == "recordField" || kind == "tupleIndex" {
        String::from("object")
    } else if kind == "tuple" || kind == "array" {
        String::from("items")
    } else if kind == "some" || kind == "flattenOption" {
        String::from("value")
    } else if (kind == "nullishNumber" || kind == "optionalIndexNumber") || kind == "truthy" {
        String::from("arg")
    } else if kind == "strictEqual" {
        String::from("left right")
    } else if ((kind == "optionalTypeOf" || kind == "optionalText") || kind == "unwrapOption")
        || kind == "optionalPredicate"
    {
        String::from("arg")
    } else if kind == "coalesce" {
        String::from("left right")
    } else if kind == "optionalField" || kind == "optionalProject" {
        String::from("object")
    } else if kind == "optionalIndex" {
        String::from("array index")
    } else if kind == "stringPoints" {
        String::from("object")
    } else if kind == "set" {
        String::from("items")
    } else if kind == "map" {
        String::from("entries")
    } else if kind == "collectionSize" {
        String::from("object")
    } else if kind == "collectionRead" {
        String::from("object key")
    } else if kind == "collectionItems" {
        String::from("object")
    } else if kind == "unary" {
        String::from("arg")
    } else if kind == "binary" {
        String::from("left right")
    } else if kind == "if" {
        String::from("cond then else")
    } else if kind == "print" {
        String::from("text body")
    } else if kind == "toString" {
        String::from("arg")
    } else if kind == "stringMap" {
        String::from("string")
    } else if kind == "stringTest" {
        String::from("string search")
    } else if kind == "cast" {
        String::from("arg")
    } else if kind == "append" {
        String::from("left right")
    } else if kind == "index" {
        String::from("array index")
    } else if kind == "length" {
        String::from("array")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_reference_source_field(
    label: &str,
    construct_kind: &str,
    type_kind: &str,
) -> String {
    if (label == "binding reference" && construct_kind == "var") && (type_kind.is_empty()) {
        String::from("name")
    } else if ((label == "declaration reference" && construct_kind == "call")
        && (type_kind.is_empty()))
        || ((label == "declaration reference" && construct_kind == "functionRef")
            && (type_kind.is_empty()))
    {
        String::from("fn")
    } else if (label == "data type declaration reference" && (construct_kind.is_empty()))
        && type_kind == "data"
    {
        String::from("name")
    } else if (label == "data type declaration reference" && construct_kind == "ctor")
        && (type_kind.is_empty())
    {
        String::from("data")
    } else if (label == "constructor declaration reference" && construct_kind == "ctor")
        && (type_kind.is_empty())
    {
        String::from("ctor")
    } else if (label == "datatype declaration reference" && (construct_kind.is_empty()))
        && type_kind == "data"
    {
        String::from("name")
    } else if (label == "datatype declaration reference" && construct_kind == "ctor")
        && (type_kind.is_empty())
    {
        String::from("data")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_reference_target_field(label: &str) -> String {
    if label == "binding reference" {
        String::from("name")
    } else if label == "declaration reference" || label == "data type declaration reference" {
        String::from("fullName")
    } else if label == "constructor declaration reference" {
        String::from("name")
    } else if label == "datatype declaration reference" {
        String::from("fullName")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_neutral_field_default(context: &str, field: &str) -> String {
    if context == "program" && (field == "imports" || field == "externals") {
        String::from("empty-array")
    } else if context == "program" && (field == "outputThreaded" || field == "abortsThreaded") {
        String::from("false")
    } else if context == "program" && field == "main" {
        String::from("null")
    } else if context == "main" && field == "sequentialAsync" {
        String::from("false")
    } else if context == "main" && field == "span" {
        String::from("null")
    } else if context == "declaration:fn" && (field == "mutual" || field == "generated") {
        String::from("false")
    } else if context == "declaration:fn" && (field == "decreasing" || field == "span") {
        String::from("null")
    } else if context == "declaration:data" && (field == "generated" || field == "output") {
        String::from("false")
    } else if context == "declaration:data" && field == "span" {
        String::from("null")
    } else if (field == "frozen" && (context == "expression:ctor" || context == "expression:tuple"))
        || (context == "expression:toString" && field == "console")
    {
        String::from("false")
    } else if context == "parameter" && (field == "guard" || field == "default") {
        String::from("null")
    } else if context == "effect:let" && field == "constant" {
        String::from("false")
    } else if field == "span"
        && ((((((((((((((((((((((((((context == "effect:let"
            || context == "declaration:module")
            || context == "effect:print")
            || context == "expression:lit")
            || context == "expression:unit")
            || context == "expression:var")
            || context == "expression:call")
            || context == "expression:ctor")
            || context == "expression:match")
            || context == "expression:unary")
            || context == "expression:binary")
            || context == "expression:if")
            || context == "expression:let")
            || context == "expression:toString")
            || context == "expression:stringMap")
            || context == "expression:stringTest")
            || context == "expression:cast")
            || context == "expression:abort")
            || context == "expression:print")
            || context == "expression:array")
            || context == "expression:append")
            || context == "expression:index")
            || context == "expression:length")
            || context == "expression:math")
            || context == "expression:tuple")
            || context == "expression:tupleIndex")
            || context == "expression:none")
    {
        String::from("null")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_expression_field(kind: &str, field: &str) -> bool {
    if kind == "tuple" {
        ((((field == "k" || field == "type") || field == "span") || field == "items")
            || field == "frozen")
            || field == "productModel"
    } else if kind == "tupleIndex" {
        (((field == "k" || field == "type") || field == "span") || field == "object")
            || field == "index"
    } else {
        false
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_tuple_product_model(model: &str, frozen: bool) -> bool {
    model == "immutable value product" && frozen
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn read_program_expression_projection(term: &str) -> String {
    if term == "lit" {
        String::from("lit")
    } else if term == "lit:bool" {
        String::from("lit:bool")
    } else if term == "lit:string" {
        String::from("lit:string")
    } else if term == "lit:nat" {
        String::from("lit:nat")
    } else if term == "lit:int" {
        String::from("lit:int")
    } else if term == "lit:fixed" {
        String::from("lit:fixed")
    } else if term == "lit:float" {
        String::from("lit:float")
    } else if term == "unit" {
        String::from("unit")
    } else if term == "var" {
        String::from("var")
    } else if term == "call" {
        String::from("call")
    } else if term == "ctor" {
        String::from("ctor")
    } else if term == "ctor:true" {
        String::from("ctor:true")
    } else if term == "match" {
        String::from("match")
    } else if term == "unary" {
        String::from("unary")
    } else if term == "binary" {
        String::from("binary")
    } else if term == "if" {
        String::from("if")
    } else if term == "let" {
        String::from("let")
    } else if term == "toString" {
        String::from("toString")
    } else if term == "toString:true" {
        String::from("toString:true")
    } else if term == "stringMap" {
        String::from("stringMap")
    } else if term == "stringTest" {
        String::from("stringTest")
    } else if term == "cast" {
        String::from("cast")
    } else if term == "abort" {
        String::from("abort")
    } else if term == "print" {
        String::from("print")
    } else if term == "array" {
        String::from("array")
    } else if term == "append" {
        String::from("append")
    } else if term == "index" {
        String::from("index")
    } else if term == "length" {
        String::from("length")
    } else if term == "length:string" {
        String::from("length:string")
    } else if term == "math" {
        String::from("math")
    } else if term == "tuple" {
        String::from("tuple")
    } else if term == "tupleIndex" {
        String::from("tupleIndex")
    } else if term == "none" {
        String::from("none")
    } else {
        String::new()
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_expression_operation(kind: &str, operation: &str) -> bool {
    if kind == "math" {
        (((((((((((((operation == "abs" || operation == "floor") || operation == "ceil")
            || operation == "trunc")
            || operation == "round")
            || operation == "sign")
            || operation == "sqrt")
            || operation == "max")
            || operation == "min")
            || operation == "maxOf")
            || operation == "minOf")
            || operation == "isInteger")
            || operation == "isSafeInteger")
            || operation == "isFinite")
            || operation == "isNaN"
    } else if kind == "stringMap" {
        ((((((operation == "toLowerCase" || operation == "toUpperCase") || operation == "trim")
            || operation == "trimStart")
            || operation == "trimEnd")
            || operation == "rustTrim")
            || operation == "rustTrimStart")
            || operation == "rustTrimEnd"
    } else if kind == "stringTest" {
        (operation == "startsWith" || operation == "endsWith") || operation == "includes"
    } else if kind == "method" {
        (((((((((((((((((((operation == "split" || operation == "slice")
            || operation == "indexOf")
            || operation == "lastIndexOf")
            || operation == "replace")
            || operation == "replaceAll")
            || operation == "repeat")
            || operation == "padStart")
            || operation == "padEnd")
            || operation == "codePointAt")
            || operation == "map")
            || operation == "filter")
            || operation == "find")
            || operation == "findIndex")
            || operation == "some")
            || operation == "every")
            || operation == "flatMap")
            || operation == "forEach")
            || operation == "reduce")
            || operation == "join")
            || operation == "includes"
    } else if kind == "collectionRead" {
        operation == "get" || operation == "has"
    } else if kind == "collectionItems" {
        (operation == "keys" || operation == "values") || operation == "entries"
    } else if kind == "optionalPredicate" {
        ((operation == "isInteger" || operation == "isSafeInteger") || operation == "isFinite")
            || operation == "isNaN"
    } else {
        false
    }
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_reference_field(label: &str) -> bool {
    (((label == "binding reference" || label == "declaration reference")
        || label == "data type declaration reference")
        || label == "constructor declaration reference")
        || label == "datatype declaration reference"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn is_unsafe_program_field(name: &str) -> bool {
    (name == "__proto__" || name == "constructor") || name == "prototype"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_schema_root(term: &str, revision: &str, arity: &str) -> bool {
    (term == "typed program" && revision == "1") && arity == "1"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_machine_integer_width(width: &str) -> bool {
    (((width == "8" || width == "16") || width == "32") || width == "64") || width == "128"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_optional_absence(absence: &str) -> bool {
    (absence == "null" || absence == "undefined") || absence == "both"
}

/// Read the corresponding decision of the shared typed program links schema.
#[must_use]
pub fn accept_program_division_rounding(rounding: &str) -> bool {
    (rounding == "trunc" || rounding == "euclid") || rounding == "floor"
}
