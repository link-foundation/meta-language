//! Existing source-origin field transport of the shared type clearing plan.
//! Capture, ABI/lifetime/owner facts and private type-validation flags survive.
use super::SourceOrigin;
use crate::translation::rust_source_type_origin_rules as rules;
pub(super) fn sanitize_source_type_origin(origin:&mut SourceOrigin){
    let plan=rules::read_rust_source_type_origin_runtime_field_plan();let mut pending=vec![origin];
    while let Some(value)=pending.pop(){
        for at in (0..plan.len()).step_by(2){
            match (plan[at].as_str(),plan[at+1].as_str()){
                ("sourceBindingIdentity","delete")=>value.source_binding_identity=None,
                ("sourcePlaceBindingIdentity","delete")=>value.source_place_binding_identity=None,
                ("sourceLoanBindingIdentities","empty list")=>value.source_loan_binding_identities.clear(),
                ("sourcePlaceLoanBindingIdentities","empty list")=>value.source_place_loan_binding_identities.clear(),
                ("sourceExtendingTemporaryBindings","empty list")=>value.source_extending_temporary_bindings.clear(),
                ("sourceBorrowedPlace","false")=>value.source_borrowed_place=false,
                ("sourceOwnedResultProof","false")=>value.source_owned_result_proof=false,
                ("sourceStandardSumPatternMove","false")=>value.source_standard_sum_pattern_move=false,
                ("sourceStandardSumBorrowedField","false")=>value.source_standard_sum_borrowed_field=false,
                ("freshArrayValue","absent fresh array")=>value.fresh_array_value=None,
                _=>panic!("the shared source type origin clearing plan has no native structural field"),
            }
        }
        if let Some(fields)=value.fields.as_mut(){pending.extend(fields.iter_mut());}
        if let Some(parameters)=value.callable_parameters.as_mut(){pending.extend(parameters.iter_mut());}
        if let Some(result)=value.callable_result.as_deref_mut(){pending.push(result);}
        if let Some(sum)=value.standard_sum_source.as_mut(){pending.extend(sum.arguments.iter_mut());}
    }
}
pub(super) fn clone_source_type_origin(origin:&SourceOrigin)->SourceOrigin{let mut result=origin.clone();sanitize_source_type_origin(&mut result);result}

#[cfg(test)]
mod deferred_controls{
    use super::*;
    use super::super::{Span,Type};
    fn body_origin()->SourceOrigin{
        let mut origin=SourceOrigin::with_type(Type::Unit,&[],Vec::new());
        origin.source_binding_identity=Some("body binding".into());origin.source_place_binding_identity=Some("body place".into());origin.source_loan_binding_identities=vec!["loan".into()];origin.source_place_loan_binding_identities=vec!["place loan".into()];origin.source_extending_temporary_bindings=vec!["temporary".into()];origin.source_owned_result_proof=true;origin.source_borrowed_place=true;origin.source_standard_sum_pattern_move=true;origin.source_standard_sum_borrowed_field=true;origin.fresh_array_value=Some(Span::new(3,4));origin.source_function_pointer_header=true;origin.copy_closure_identity=Some(Span::new(1,2));origin.copy_closure_capture_count=Some(2);origin.source_lifetimes=vec![Some("static".into())];origin
    }
    #[test]
    fn callable_type_clone_removes_body_lineage_and_keeps_validation_facts(){
        let mut origin=body_origin();origin.fields=Some(vec![body_origin()]);origin.callable_parameters=Some(vec![body_origin()]);origin.callable_result=Some(Box::new(body_origin()));
        origin.standard_sum_source=Some(Box::new(super::super::standard_sum_source::StandardSumSource {owner:"std::option::Option".into(),arguments:vec![body_origin()],type_path:vec!["crate".into(),"Sum".into()],constructor_paths:std::collections::HashMap::new(),ordinal:0,provisional:true}));
        let original=origin.clone();let output=clone_source_type_origin(&origin);assert_eq!(origin,original);
        let values=std::iter::once(&output).chain(output.fields.iter().flatten()).chain(output.callable_parameters.iter().flatten()).chain(output.callable_result.as_deref()).chain(output.standard_sum_source.iter().flat_map(|sum|sum.arguments.iter()));
        for value in values{
            assert!(value.source_binding_identity.is_none()&&value.source_place_binding_identity.is_none());assert!(value.source_loan_binding_identities.is_empty()&&value.source_place_loan_binding_identities.is_empty()&&value.source_extending_temporary_bindings.is_empty());assert!(!value.source_owned_result_proof&&!value.source_borrowed_place&&!value.source_standard_sum_pattern_move&&!value.source_standard_sum_borrowed_field);assert!(value.fresh_array_value.is_none());
            assert!(value.proven&&value.copy_evidence.is_some()&&value.source_function_pointer_header);assert_eq!(value.copy_closure_identity,Some(Span::new(1,2)));assert_eq!(value.copy_closure_capture_count,Some(2));assert_eq!(value.source_lifetimes,vec![Some("static".into())]);
        }
        assert!(output.standard_sum_source.as_ref().unwrap().provisional);
    }
}
