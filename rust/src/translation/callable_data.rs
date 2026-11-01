//! Actual type/declaration transport over the shared callable-data decisions.
use super::{types::Type,ir::DataDecl,diagnostics::{Result,unsupported}};
use super::callable_data_derivation_rules as rules;
use std::collections::HashSet;
use std::collections::HashMap;
pub(crate) struct CallableDataFacts{pub callable:bool,pub constructive:bool,pub acyclic:bool}
pub(crate) struct CallableDataRecursiveProfile{pub recursive:bool,pub witness:usize}
fn datatype_constructive_plan<'a,F>(ty:&Type,lookup:&F,mutual:bool)->Result<(Vec<String>,Vec<String>)> where F:Fn(&str)->Option<&'a DataDecl>{
    #[derive(Default)]
    struct Projection{kinds:Vec<String>,ends:Vec<f64>,edges:Vec<f64>,references:Vec<f64>,roots:Vec<f64>,names:Vec<String>,positions:HashMap<String,usize>,visits:usize}
    impl Projection{
        fn node(&mut self,kind:&str,children:Vec<usize>,reference:f64)->Result<usize>{let reason=rules::read_callable_data_traversal_error(0.0,(self.kinds.len()+1) as f64);if !reason.is_empty(){return Err(unsupported("callable data source profile",&reason,None));}let at=self.kinds.len();self.kinds.push(kind.into());self.edges.extend(children.into_iter().map(|child|child as f64));self.ends.push(self.edges.len() as f64);self.references.push(reference);Ok(at)}
        fn walk(&mut self,ty:&Type,depth:usize)->Result<usize>{
            self.visits+=1;let reason=rules::read_callable_data_traversal_error(depth as f64,self.visits as f64);if !reason.is_empty(){return Err(unsupported("callable data source profile",&reason,None));}
            if let Type::Data{name}=ty{let reference=if let Some(position)=self.positions.get(name){*position}else{let position=self.names.len();self.positions.insert(name.clone(),position);self.names.push(name.clone());self.roots.push(-1.0);position};return self.node("reference",Vec::new(),reference as f64);}
            let children=ty.children().iter().map(|child|self.walk(child,depth+1)).collect::<Result<Vec<_>>>()?;self.node(ty.kind(),children,-1.0)
        }
    }
    let mut projection=Projection::default();let root=projection.walk(ty,0)?;let mut at=0;
    while at<projection.names.len(){
        let data=lookup(&projection.names[at]).ok_or_else(||unsupported("callable data source profile","the actual callable graph needs every declaration",None))?;
        let mut constructors=Vec::new();for ctor in &data.ctors{let fields=ctor.fields.iter().map(|field|projection.walk(&field.ty,0)).collect::<Result<Vec<_>>>()?;constructors.push(projection.node("constructor",fields,-1.0)?);}
        let node=projection.node("data",constructors,-1.0)?;projection.roots[at]=node as f64;at+=1;
    }
    let plan=if mutual{rules::read_formal_datatype_constructive_plan(&projection.kinds,&projection.ends,&projection.edges,&projection.references,&projection.roots,root as f64)}else{rules::read_callable_data_recursive_plan(&projection.kinds,&projection.ends,&projection.edges,&projection.references,&projection.roots,root as f64)};
    if plan[0]=="error"{return Err(unsupported("callable data field",&plan[1],None));}
    Ok((projection.names,plan))
}
pub(crate) fn read_callable_data_recursive_profile<'a,F>(ty:&Type,lookup:&F)->Result<CallableDataRecursiveProfile> where F:Fn(&str)->Option<&'a DataDecl>{
    let (_,plan)=datatype_constructive_plan(ty,lookup,false)?;
    Ok(CallableDataRecursiveProfile{recursive:plan[2]=="true",witness:plan[4].parse().expect("shared actual data constructor witness")})
}
pub(crate) struct FormalDatatypeConstructiveProfile{pub witness:usize,pub rank:usize}
pub(crate) fn read_formal_datatype_constructive_profiles<'a,F>(ty:&Type,lookup:&F)->Result<HashMap<String,FormalDatatypeConstructiveProfile>> where F:Fn(&str)->Option<&'a DataDecl>{
 let (names,plan)=datatype_constructive_plan(ty,lookup,true)?;
 if plan.first().is_none_or(|tag|tag!="witnesses")||plan.len()!=1+2*names.len(){return Err(unsupported("formal mutual datatype","the actual complete graph needs each finite constructor witness",None));}
 names.into_iter().enumerate().map(|(index,name)|{let witness=plan[1+index*2].parse().map_err(|_|unsupported("formal mutual datatype","invalid shared constructor witness",None))?;let rank=plan[2+index*2].parse().map_err(|_|unsupported("formal mutual datatype","invalid shared constructor witness rank",None))?;Ok((name,FormalDatatypeConstructiveProfile{witness,rank}))}).collect()
}
fn node(kind:&str,children:Vec<CallableDataFacts>,acyclic:bool)->CallableDataFacts{
    CallableDataFacts {callable:rules::accept_callable_data_node(kind,&children.iter().map(|child|child.callable).collect::<Vec<_>>()),constructive:rules::accept_callable_data_constructive_node(kind,&children.iter().map(|child|child.constructive).collect::<Vec<_>>()),acyclic:acyclic&&children.iter().all(|child|child.acyclic)}
}
pub(crate) fn read_callable_data_facts<'a,F>(ty:&Type,lookup:&F)->Result<CallableDataFacts> where F:Fn(&str)->Option<&'a DataDecl>{
    fn walk<'a,F>(ty:&Type,lookup:&F,active:&mut HashSet<String>,visits:&mut usize,depth:usize)->Result<CallableDataFacts> where F:Fn(&str)->Option<&'a DataDecl>{
        *visits+=1;let reason=rules::read_callable_data_traversal_error(depth as f64,*visits as f64);if !reason.is_empty(){return Err(unsupported("callable data source profile",&reason,None));}
        let Type::Data{name}=ty else{return Ok(node(ty.kind(),ty.children().iter().map(|child|walk(child,lookup,active,visits,depth+1)).collect::<Result<Vec<_>>>()?,true));};
        if active.contains(name){return Ok(node("cycle",Vec::new(),false));}
        let Some(data)=lookup(name) else{return Ok(node("missing",Vec::new(),false));};active.insert(name.clone());
        let children=data.ctors.iter().map(|ctor|Ok(node("constructor",ctor.fields.iter().map(|field|walk(&field.ty,lookup,active,visits,depth+1)).collect::<Result<Vec<_>>>()?,true))).collect::<Result<Vec<_>>>();active.remove(name);
        Ok(node("data",children?,true))
    }
    let mut visits=0;walk(ty,lookup,&mut HashSet::new(),&mut visits,0)
}
#[cfg(test)]
mod deferred_controls{
    use super::*;
    #[test]
    fn actual_nominal_function_fields_require_acyclic_constructive_results(){
        let mut data:DataDecl=serde_json::from_value(serde_json::json!({"name":"Callback","fullName":"Callback","modulePath":[],"ctors":[{"name":"Value","fields":[{"name":"read","type":{"kind":"function","params":[{"kind":"bool"}],"ret":{"kind":"bool"}}}]}]})).unwrap();
        let ty=Type::Data{name:"Callback".into()};
        let facts=read_callable_data_facts(&ty,&|name|if name=="Callback"{Some(&data)}else{None}).unwrap();assert!(facts.callable&&facts.constructive&&facts.acyclic);
        data.ctors[0].fields[0].ty=Type::Function{params:vec![ty.clone()],ret:Box::new(Type::Bool)};
        let facts=read_callable_data_facts(&ty,&|name|if name=="Callback"{Some(&data)}else{None}).unwrap();assert!(facts.callable&&!facts.acyclic);assert!(!rules::read_callable_data_profile_error(facts.callable,facts.acyclic,facts.constructive).is_empty());
    }
}
