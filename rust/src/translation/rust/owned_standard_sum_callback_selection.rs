//! Read-only token/catalog context selecting an owning callable result. The
//! actual callback reader still admits the complete signature and source body.
use super::{RustParser,SourceDeclaration,SourceOrigin,TokenKind,Type};
use crate::translation::rust_owned_standard_sum_callback_rules as rules;
impl RustParser{
 fn owned_callback_selection_output(&self,origin:Option<&SourceOrigin>)->bool{origin.is_some_and(|origin|self.origins.standard_sum_ownership_facts(origin).profile=="owned"&&self.origins.standard_sum_source_identity(origin).is_some())}
 pub(super) fn selects_owned_standard_sum_callback(&self,path:&[String])->bool{
  let tokens=self.cursor.tokens.iter().skip(self.cursor.index).take(rules::read_rust_owned_standard_sum_callback_selection_token_limit() as usize).collect::<Vec<_>>();let values=tokens.iter().map(|token|token.value.clone()).collect::<Vec<_>>();let kinds=tokens.iter().map(|token|if token.kind==TokenKind::Identifier{"identifier".to_owned()}else{String::new()}).collect::<Vec<_>>();let plan=rules::plan_rust_owned_standard_sum_callback_selection(&values,&kinds);if plan.len()!=3{return false;}
  let names=(plan[1] as usize..plan[2] as usize).step_by(2).map(|at|tokens[at].value.clone()).collect::<Vec<_>>();
  if plan[0]==3.0{
   let resolved=self.resolve(&names,path);let owner=self.named_source_type_owner(&names,Vec::new());
   let roots=[names.clone(),resolved.clone(),vec![names[0].clone()],vec![resolved[0].clone()]];
   let alias=roots.iter().any(|head|crate::translation::rust_type_alias_rules::read_rust_type_alias_lookup_keys(head,path).iter().any(|key|self.source_type_aliases.declarations.contains_key(key)));
   let shadow=names.len()>1&&self.origins.lookup_type_owner(&names[..1],path).is_some()||resolved.len()>1&&self.origins.lookup_type_owner(&resolved[..1],path).is_some();
   return rules::accept_rust_owned_standard_sum_callback_written_selection(&owner.kind,&owner.path.join("::"),alias,shadow);
  }
  if names.len()==1{if let Some(callable)=self.origins.scope.get(&names[0]){return plan[0]==1.0&&callable.direct_copy_value()&&callable.wrappers.is_empty()&&matches!(callable.ty,Some(Type::Function{..}))&&self.owned_callback_selection_output(callable.callable_result.as_deref());}}
  let entry=self.origins.lookup_value_owner(&self.source_value_path(&names,path),path);
  match entry.map(|entry|entry.declaration){
   Some(SourceDeclaration::Function{result,..})=>if plan[0]==2.0{result.direct_copy_value()&&result.wrappers.is_empty()&&matches!(result.ty,Some(Type::Function{..}))&&self.owned_callback_selection_output(result.callable_result.as_deref())}else{self.owned_callback_selection_output(Some(&result))},
   Some(SourceDeclaration::Constant{result,..})=>plan[0]==1.0&&result.direct_copy_value()&&result.wrappers.is_empty()&&matches!(result.ty,Some(Type::Function{..}))&&self.owned_callback_selection_output(result.callable_result.as_deref()),
   _=>false,
  }
 }
}
