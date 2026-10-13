//! Reuse the existing datatype projection in checked namespace scopes.
use super::{Result, project_type};
use crate::translation::ir::{DataDecl, Program};
use crate::translation::surface::{SCtor, SData, SField, SItem};

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
    }))
}
