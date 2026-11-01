//! Actual standard-sum type/constructor/loan transport into shared ownership rules.
//! Existing String/Vec state transitions remain the only owned leaf model.
use std::collections::{HashMap,HashSet};
use super::{Result,RustParser,SExpr,SNode,SourceDeclaration,SourceOrigin,SourceOrigins,Span,Type,unsupported};
use crate::translation::rust_owned_binding_rules as owned_rules;
use crate::translation::rust_standard_sum_ownership_rules as rules;

#[derive(Clone,Default)]
pub(super) struct StandardSumOwnershipFacts{
    pub(super) profile:String,
    pub(super) owner:String,
    pub(super) roles:Vec<String>,
    pub(super) definition_name:String,
}

impl SourceOrigins{
    pub(super) fn standard_sum_ownership_facts(&self,origin:&SourceOrigin)->StandardSumOwnershipFacts{
        self.standard_sum_ownership_facts_bounded(origin,&mut HashSet::new(),&mut HashMap::new(),0,&mut 0)
    }
    fn standard_sum_ownership_facts_bounded(&self,origin:&SourceOrigin,active:&mut HashSet<String>,memo:&mut HashMap<String,StandardSumOwnershipFacts>,depth:usize,visits:&mut usize)->StandardSumOwnershipFacts{
        *visits+=1;if (*visits as f64)>rules::read_rust_standard_sum_ownership_node_limit()||!origin.proven||origin.ty.is_none()||!origin.wrappers.is_empty()||(depth as f64)>rules::read_rust_standard_sum_ownership_depth_limit(){return StandardSumOwnershipFacts::default();}
        let declared=match origin.ty.as_ref(){Some(Type::Named {path,..})=>self.lookup_type_owner(path,&origin.path),_=>None};
        let stored=declared.as_ref().filter(|entry|matches!(entry.declaration,SourceDeclaration::Data)).and_then(|entry|self.standard_sum_source_origins.get(&entry.full_name));
        if let Some((stored,sum))=stored.and_then(|origin|origin.standard_sum_source.as_deref().map(|sum|(origin,sum))){
            if !stored.proven||sum.provisional||!stored.wrappers.is_empty(){return StandardSumOwnershipFacts::default();}
            let name=declared.as_ref().expect("actual data definition").full_name.clone();if let Some(facts)=memo.get(&name){return facts.clone();}if !active.insert(name.clone()){return StandardSumOwnershipFacts::default();}
            let roles=sum.arguments.iter().map(|argument|self.standard_sum_ownership_facts_bounded(argument,active,memo,depth+1,visits).profile).collect::<Vec<_>>();active.remove(&name);
            let facts=StandardSumOwnershipFacts{profile:rules::read_rust_standard_sum_ownership_profile(&sum.owner,&roles),owner:sum.owner.clone(),roles,definition_name:name.clone()};memo.insert(name,facts.clone());return facts;
        }
        let ty=origin.ty.as_ref().expect("actual source type");
        if owned_rules::accept_rust_owned_binding_value(ty.kind(),origin.array_source_model.or(origin.text_source_model).unwrap_or(""),origin.wrappers.len() as f64,origin.proven){
            return StandardSumOwnershipFacts{profile:"owned".into(),owner:origin.expanded_source_owner.as_ref().or(origin.source_owner.as_ref()).map_or_else(String::new,|owner|owner.path.join("::")),roles:Vec::new(),definition_name:String::new()};
        }
        if let Some(declared)=declared.filter(|entry|matches!(entry.declaration,SourceDeclaration::Data)){
            return StandardSumOwnershipFacts{profile:if self.nominal_copy_owners.contains(&declared.full_name){"copy".into()}else{self.nominal_ownership_facts(origin).profile},owner:String::new(),roles:Vec::new(),definition_name:declared.full_name};
        }
        if let (Type::Tuple {..},Some(fields))=(ty,origin.fields.as_ref()){
            let roles=fields.iter().map(|field|self.standard_sum_ownership_facts_bounded(field,active,memo,depth+1,visits).profile).collect::<Vec<_>>();
            return StandardSumOwnershipFacts{profile:if roles.iter().all(|role|role=="copy")&&origin.direct_copy_value(){"copy".into()}else{String::new()},owner:String::new(),roles,definition_name:String::new()};
        }
        StandardSumOwnershipFacts{profile:if origin.direct_copy_value(){"copy".into()}else{String::new()},..StandardSumOwnershipFacts::default()}
    }
    pub(super) fn is_owned_standard_sum_value(&self,origin:&SourceOrigin)->bool{
        if !matches!(origin.ty,Some(Type::Named {..})){return false;}
        let facts=self.standard_sum_ownership_facts(origin);facts.profile=="owned"&&!facts.definition_name.is_empty()&&rules::read_rust_standard_sum_ownership_profile(&facts.owner,&facts.roles)=="owned"
    }
    pub(super) fn project_standard_sum_pattern_field(&self,parent:&SourceOrigin,field:&SourceOrigin,borrowed:bool)->SourceOrigin{
        // Transport loans, never a match scrutinee's temporary-extension grant.
        // Existing outer initializer extensions already own their drop scope.
        let mut value=super::copy_closure_lifetimes::project_value_field(parent,field.clone(),0);value.source_binding_identity=None;value.source_place_binding_identity=None;value.source_loan_binding_identities.clear();value.source_place_loan_binding_identities.clear();value.source_extending_temporary_bindings.clear();value.source_owned_result_proof=false;value.source_standard_sum_pattern_move=false;value.source_standard_sum_borrowed_field=false;
        let mut direct=parent.clone();direct.wrappers.clear();let parent_facts=self.standard_sum_ownership_facts(&direct);
        let lineage=parent.source_binding_identity.is_some()||parent.source_place_binding_identity.is_some()||!parent.source_loan_binding_identities.is_empty()||!parent.source_place_loan_binding_identities.is_empty();
        let role=rules::read_rust_standard_sum_owned_field_disposition(parent_facts.profile=="owned"&&!parent_facts.definition_name.is_empty()||borrowed&&lineage,borrowed,self.standard_sum_ownership_facts(field).profile=="owned");
        if borrowed{value.wrappers.insert(0,"reference".into());value.source_lifetimes.insert(0,parent.source_lifetimes.first().cloned().unwrap_or(None));value.source_value_borrow_bindings=owned_rules::merge_rust_owned_loan_bindings(&super::copy_closure_lifetimes::value_borrows(&value),&super::copy_closure_lifetimes::value_borrows(parent));}
        if role=="borrow"{
            value.source_standard_sum_borrowed_field=true;
            value.source_nominal_borrowed_field=parent.source_nominal_borrowed_field;
            value.source_binding_identity=parent.source_binding_identity.clone().or_else(||parent.source_place_binding_identity.clone());value.source_place_binding_identity=parent.source_place_binding_identity.clone().or_else(||parent.source_binding_identity.clone());
            value.source_loan_binding_identities=owned_rules::merge_rust_owned_loan_bindings(&owned_rules::project_rust_owned_loan_bindings(parent.source_binding_identity.as_deref().unwrap_or(""),&parent.source_loan_binding_identities),&owned_rules::project_rust_owned_loan_bindings(parent.source_place_binding_identity.as_deref().unwrap_or(""),&parent.source_place_loan_binding_identities));value.source_borrowed_place=true;
        }else if role=="move"{value.source_binding_identity=parent.source_binding_identity.clone();value.source_borrowed_place=parent.source_borrowed_place;value.source_standard_sum_pattern_move=true;value.source_nominal_pattern_move=parent.source_nominal_pattern_move;value.source_owned_result_proof=parent.source_owned_result_proof;}
        else{value.source_borrowed_place=parent.source_borrowed_place;}
        value
    }
    pub(super) fn validate_standard_sum_pattern_transfer(&self,origin:&SourceOrigin,span:Option<Span>)->Result<()>{
        let error=rules::read_rust_standard_sum_owned_pattern_transfer_error(origin.source_standard_sum_pattern_move||self.is_owned_standard_sum_value(origin),origin.source_binding_identity.is_some(),origin.source_owned_result_proof);
        if error.is_empty(){Ok(())}else{Err(unsupported("Rust owned standard sum",&error,span))}
    }
    pub(super) fn validate_standard_sum_borrowed_result(&self,origin:&SourceOrigin,span:Option<Span>)->Result<()>{
        if origin.source_standard_sum_borrowed_field||origin.source_nominal_borrowed_field{self.check_owned_origin(origin,span)?;}Ok(())
    }
    pub(super) fn finish_standard_sum_field_dereference(&self,mut origin:SourceOrigin,span:Option<Span>)->Result<SourceOrigin>{
        if !origin.source_standard_sum_borrowed_field&&!origin.source_nominal_borrowed_field{return Ok(origin);}
        self.validate_standard_sum_borrowed_result(&origin,span)?;
        if origin.wrappers.is_empty()&&self.standard_sum_ownership_facts(&origin).profile=="copy"{
            origin.source_binding_identity=None;origin.source_place_binding_identity=None;origin.source_loan_binding_identities.clear();origin.source_place_loan_binding_identities.clear();origin.source_extending_temporary_bindings.clear();origin.source_borrowed_place=false;origin.source_standard_sum_borrowed_field=false;origin.source_standard_sum_pattern_move=false;origin.source_nominal_borrowed_field=false;origin.source_nominal_pattern_move=false;origin.source_owned_result_proof=false;
        }
        Ok(origin)
    }
    pub(super) fn standard_sum_owned_call_result(&self,node:&SExpr,path:&[String],mut origin:SourceOrigin)->SourceOrigin{
        let facts=self.standard_sum_ownership_facts(&origin);let owned=facts.profile=="owned"&&!facts.definition_name.is_empty();
        let names=match &node.node{SNode::Name {path}=>Some(path),SNode::App {func,..}=>match &func.node{SNode::Name {path}=>Some(path),_=>None},_=>None};
        let function=names.and_then(|names|self.lookup_value_owner(names,path)).is_some_and(|entry|match entry.declaration{
            SourceDeclaration::Function {result,..}=>{let output=self.standard_sum_ownership_facts(&result);output.profile=="owned"&&output.definition_name==facts.definition_name},_=>false,
        });
        if rules::accept_rust_standard_sum_owned_call_result(owned,function,self.owned_standard_sum_recorded_call(node)){
            origin.source_binding_identity=None;origin.source_place_binding_identity=None;origin.source_loan_binding_identities.clear();origin.source_place_loan_binding_identities.clear();origin.source_extending_temporary_bindings.clear();origin.source_borrowed_place=false;origin.source_owned_result_proof=true;
        }
        origin
    }
}

impl RustParser{
    pub(super) fn capture_standard_sum_constructor(&mut self,node:&SExpr,origin:&SourceOrigin,args:&[SExpr],path:&[String],span:Option<Span>)->Result<()>{
        let produced=self.standard_sum_constructor_origin(node,origin,args,path,span)?;self.origins.mark(node,produced);Ok(())
    }
    pub(super) fn standard_sum_constructor_origin(&mut self,node:&SExpr,origin:&SourceOrigin,args:&[SExpr],path:&[String],span:Option<Span>)->Result<SourceOrigin>{
        if self.header_discovery{return Ok(origin.clone());}
        let facts=self.origins.standard_sum_ownership_facts(origin);
        if !["copy","owned"].contains(&facts.profile.as_str())||facts.definition_name.is_empty(){return Err(unsupported("Rust owned standard sum","the constructor needs its actual admitted standard datatype",span));}
        let names=match &node.node{SNode::Name {path}=>Some(path),SNode::App {func,..}=>match &func.node{SNode::Name {path}=>Some(path),_=>None},_=>None};
        let entry=names.and_then(|names|self.origins.lookup_value_owner(names,path));
        let (actual,fields)=match entry{Some(entry)=>match entry.declaration{SourceDeclaration::Constructor {owner,fields,..}=>(owner==facts.definition_name,fields),_=>(false,Vec::new())},None=>(false,Vec::new())};
        let nominal=self.origins.nominal_ownership_facts(origin);
        let error=if !nominal.profile.is_empty()&&nominal.owner==facts.definition_name{crate::translation::rust_nominal_ownership_rules::read_rust_nominal_constructor_error(actual,fields.len() as f64,args.len() as f64)}else{rules::read_rust_standard_sum_owned_constructor_error(actual,fields.len() as f64,args.len() as f64)};if !error.is_empty(){return Err(unsupported("Rust owned standard sum",&error,span));}
        let values=args.iter().map(|argument|self.origins.origin(argument,path)).collect::<Vec<_>>();
        for ((argument,field),value) in args.iter().zip(&fields).zip(&values){
            let owned=self.origins.standard_sum_ownership_facts(field).profile=="owned";let transferred=self.origins.owned_standard_sum_argument_transferred(argument);self.origins.argument(value,field,argument.span.or(span))?;
            let called=match &argument.node{SNode::App{func,..}=>match &func.node{SNode::Name{path:names}=>self.origins.lookup_value_owner(names,path),_=>None},_=>None};
            let call_producer=called.is_some_and(|entry|match entry.declaration{SourceDeclaration::Function{result,..}=>self.origins.standard_sum_ownership_facts(&result).profile=="owned"&&self.origins.owned_standard_sum_recorded_call(argument)&&self.origins.same_origin(value,&result),_=>false});
            let producer=value.source_owned_result_proof||value.fresh_array_value.is_some()||self.origins.owned_standard_sum_recorded_vector(argument)||call_producer;
            if !self.origins.collecting{let error=rules::read_rust_standard_sum_owned_field_production_error(owned,transferred,self.origins.available_owned_binding(value),producer);if !error.is_empty(){return Err(unsupported("Rust owned standard sum",&error,argument.span.or(span)));}}
            if !transferred{self.origins.prepare_owned_argument(argument,Some(field),path)?;}
        }
        let field_borrows=values.iter().map(super::copy_closure_lifetimes::value_borrows).collect::<Vec<_>>();let mut all=Vec::new();for field in &field_borrows{all=owned_rules::merge_rust_owned_loan_bindings(&all,field);}
        let mut value=origin.clone();value.source_lexical_binding_identity=None;value.source_value_borrow_bindings=all;value.source_value_field_borrow_bindings=Some(field_borrows);value.source_binding_identity=None;value.source_place_binding_identity=None;value.source_loan_binding_identities.clear();value.source_place_loan_binding_identities.clear();value.source_extending_temporary_bindings.clear();value.source_borrowed_place=false;value.source_owned_result_proof=facts.profile=="owned";
        Ok(value)
    }
}
