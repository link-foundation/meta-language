//! Reuse the existing datatype projection in checked namespace scopes.
use super::{Result, error, project_type};
use crate::translation::ir::{DataDecl, FnDecl, Program};
use crate::translation::surface::{SCtor, SData, SField, SItem};
use crate::translation::typed_program_link_rules::read_program_declaration_type_projection_field;
use crate::translation::types::Type;
use serde::Deserialize;
use serde_json::{Map, Value};

/// Interpret the shared declaration result roles using actual retained types.
pub(super) fn project_function_result_types(
    function: &FnDecl,
    program: &Program,
) -> Result<(Type, Option<Type>)> {
    #[derive(Deserialize)]
    struct ProjectedFunctionResultTypes {
        ret: Type,
        #[serde(default, rename = "semanticRet")]
        semantic_ret: Option<Type>,
    }
    let value = serde_json::to_value(function).map_err(|failure| error(failure.to_string()))?;
    let annotation = read_program_declaration_type_projection_field("fn", "annotation");
    let retained = value.get(&annotation);
    let project = |value: &Value| -> Result<Value> {
        let ty =
            serde_json::from_value(value.clone()).map_err(|failure| error(failure.to_string()))?;
        serde_json::to_value(project_type(&ty, program)?)
            .map_err(|failure| error(failure.to_string()))
    };
    let mut fields = Map::new();
    fields.insert(
        "ret".to_owned(),
        project(retained.unwrap_or_else(|| &value["ret"]))?,
    );
    if retained.is_some() {
        let result = read_program_declaration_type_projection_field("fn", "result");
        let semantic = read_program_declaration_type_projection_field("fn", "semantic");
        fields.insert(semantic, project(&value[result])?);
    }
    let projected: ProjectedFunctionResultTypes = serde_json::from_value(Value::Object(fields))
        .map_err(|failure| error(failure.to_string()))?;
    Ok((projected.ret, projected.semantic_ret))
}

pub(super) fn project_data(data: &DataDecl, program: &Program) -> Result<SItem> {
    Ok(SItem::Data(SData {
        name: data.name.clone(),
        ctors: data
            .ctors
            .iter()
            .map(|constructor| -> Result<SCtor> {
                Ok(SCtor {
                    name: constructor.name.clone(),
                    fields: constructor
                        .fields
                        .iter()
                        .map(|field| -> Result<SField> {
                            Ok(SField {
                                name: Some(field.name.clone()),
                                ty: project_type(&field.ty, program)?,
                                rocq_type: None,
                                span: None,
                            })
                        })
                        .collect::<Result<_>>()?,
                })
            })
            .collect::<Result<_>>()?,
        span: data.span,
        generated: data.generated,
        source_unit_clone_derivation: data.source_unit_clone_derivation.clone().map(Box::new),
        source_field_signatures: data.source_field_signatures.clone(),
    }))
}
