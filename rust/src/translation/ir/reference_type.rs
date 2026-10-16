//! Retain the generated reference-result field without changing ordinary Type
//! defaults. The existing checked type still supplies its actual Boolean value.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use super::{Expr, Node, Span, Type};
use crate::translation::surface_reference_analysis_rules::read_surface_reference_result_field;

struct ReferenceResultType<'a> {
    ty: &'a Type,
    field: String,
}

impl Serialize for ReferenceResultType<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.field.is_empty() {
            return self.ty.serialize(serializer);
        }
        let mut value =
            serde_json::to_value(self.ty).map_err(<S::Error as serde::ser::Error>::custom)?;
        value
            .as_object_mut()
            .ok_or_else(|| {
                <S::Error as serde::ser::Error>::custom("a checked result type is not a record")
            })?
            .insert(self.field.clone(), Value::Bool(self.ty.is_reference()));
        value.serialize(serializer)
    }
}

impl Serialize for Expr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct ExpressionWire<'a> {
            #[serde(flatten)]
            node: &'a Node,
            #[serde(rename = "type")]
            ty: ReferenceResultType<'a>,
            #[serde(rename = "referenceOperation", skip_serializing_if = "Option::is_none")]
            reference_operation: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            span: Option<Span>,
        }
        ExpressionWire {
            node: &self.node,
            reference_operation: {
                let operation =
                    crate::translation::primitive_value_rules::read_scalar_cell_operation(
                        self.kind(),
                    );
                (!operation.is_empty()).then_some(operation)
            },
            ty: ReferenceResultType {
                ty: &self.ty,
                field: read_surface_reference_result_field(self.kind()),
            },
            span: self.span,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Expr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct ExpressionWire {
            #[serde(flatten)]
            node: Node,
            #[serde(rename = "type")]
            ty: Value,
            #[serde(default, rename = "referenceOperation")]
            reference_operation: Option<String>,
            #[serde(default)]
            span: Option<Span>,
        }
        let wire = ExpressionWire::deserialize(deserializer)?;
        let operation =
            crate::translation::primitive_value_rules::read_scalar_cell_operation(wire.node.kind());
        if !operation.is_empty() && wire.reference_operation.as_deref() != Some(operation.as_str())
        {
            return Err(<D::Error as serde::de::Error>::custom(
                "a checked scalar cell must retain its actual linked operation",
            ));
        }
        let field = read_surface_reference_result_field(wire.node.kind());
        if !field.is_empty() && !wire.ty.get(&field).is_some_and(Value::is_boolean) {
            return Err(<D::Error as serde::de::Error>::custom(
                "a checked reference result must retain its explicit Boolean type field",
            ));
        }
        Ok(Self {
            node: wire.node,
            ty: serde_json::from_value(wire.ty).map_err(<D::Error as serde::de::Error>::custom)?,
            span: wire.span,
        })
    }
}
