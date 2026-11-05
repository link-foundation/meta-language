//! Deferred structural transport controls; not a source ownership certificate.
use super::*;
fn declared(path:&[&str])->SourceOwner{SourceOwner{kind:"declaration".into(),path:path.iter().map(|value|(*value).to_owned()).collect(),arguments:Vec::new()}}
fn library(path:&[&str],child:SourceOwner)->SourceOwner{SourceOwner{kind:"library".into(),path:path.iter().map(|value|(*value).to_owned()).collect(),arguments:vec![child]}}
#[test]
fn canonical_type_and_owners_do_not_upgrade_actual_value_facts(){
 let mut table=SourceOrigins::new(Some(HashMap::from([("Later".into(),SourceDeclaration::Data),("domain.Later".into(),SourceDeclaration::Data)])),"");
 table.nominal_copy_owners.insert("domain.Later".into());
 let mut origin=SourceOrigin::unknown();
 origin.ty=Some(Type::Array{element:Box::new(Type::Named{path:vec!["Later".into()],span:None}),reference:false});
 origin.path=vec!["domain".into()];origin.wrappers=vec!["box".into()];origin.uncertain=true;
 origin.source_owner=Some(library(&["std","boxed","Box"],library(&["std","vec","Vec"],declared(&["Later"]))));
 origin.source_binding_identity=Some("existing source binding".into());origin.source_value_borrow_bindings=vec!["existing source loan".into()];
 let result=table.standard_sum_qualified_origin(&origin).unwrap();
 assert_eq!(result.ty,Some(Type::Array{element:Box::new(Type::Named{path:vec!["crate".into(),"domain".into(),"Later".into()],span:None}),reference:false}));
 let owner=result.source_owner.as_ref().unwrap();
 assert_eq!(owner.path,vec!["std","boxed","Box"]);assert_eq!(owner.arguments[0].path,vec!["std","vec","Vec"]);assert_eq!(owner.arguments[0].arguments[0].path,vec!["domain","Later"]);
 assert_eq!(result.path,origin.path);assert_eq!(result.wrappers,origin.wrappers);assert_eq!(result.proven,origin.proven);assert_eq!(result.copy_evidence,origin.copy_evidence);assert_eq!(result.diverging,origin.diverging);assert_eq!(result.uncertain,origin.uncertain);assert_eq!(result.source_binding_identity,origin.source_binding_identity);assert_eq!(result.source_value_borrow_bindings,origin.source_value_borrow_bindings);
}
