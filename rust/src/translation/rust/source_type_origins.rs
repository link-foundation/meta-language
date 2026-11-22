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
                ("sourceLexicalBindingIdentity","delete")=>value.source_lexical_binding_identity=None,
                ("sourceRecordStateCollectingSite","delete")=>value.source_record_state_collecting_site=None,
                ("sourceValueBorrowBindings","empty list")=>value.source_value_borrow_bindings.clear(),
                ("sourceValueFieldBorrowBindings","delete")=>value.source_value_field_borrow_bindings=None,
                ("sourcePlaceBindingIdentity","delete")=>value.source_place_binding_identity=None,
                ("sourceLoanBindingIdentities","empty list")=>value.source_loan_binding_identities.clear(),
                ("sourcePlaceLoanBindingIdentities","empty list")=>value.source_place_loan_binding_identities.clear(),
                ("sourceExtendingTemporaryBindings","empty list")=>value.source_extending_temporary_bindings.clear(),
                ("sourceBorrowedPlace","false")=>value.source_borrowed_place=false,
                ("sourceOwnedResultProof","false")=>value.source_owned_result_proof=false,
                ("sourceStandardSumPatternMove","false")=>value.source_standard_sum_pattern_move=false,
                ("sourceStandardSumBorrowedField","false")=>value.source_standard_sum_borrowed_field=false,
                ("sourceNominalPatternMove","false")=>value.source_nominal_pattern_move=false,
                ("sourceNominalBorrowedField","false")=>value.source_nominal_borrowed_field=false,
                ("sourceBoxLayers","empty list")=>value.source_box_layers.clear(),
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

include!("../../../tests/internal/translation/rust/source_type_origins.rs");
