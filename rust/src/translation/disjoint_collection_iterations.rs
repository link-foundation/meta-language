//! Structural transport to the shared checked allocation/effect proof.
//! No native source-name alias decisions or replacement iterator semantics.
use super::{diagnostics::{Result,unsupported},ir::Program};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Clone,Copy)]
pub(crate) struct Proof {pub(crate) error:u8,pub(crate) array_callbacks:bool,pub(crate) collection_mappers:bool}
fn refused(error:u8)->Proof{Proof{error,array_callbacks:false,collection_mappers:false}}
fn position(value:f64)->Option<usize>{if value.is_finite()&&value>=0.0&&value.fract()==0.0{value.to_string().parse().ok()}else{None}}
fn number(value:usize)->f64{serde_json::Number::from(value).as_f64().expect("bounded graph position")}

pub(crate) fn read_program_proof(program:&Program,markers:&[String])->Proof{
 let Ok(value)=serde_json::to_value(program)else{return refused(1)};
 let facts=super::surface_reference_facts::create_surface_reference_facts(&value);
 let plan=super::disjoint_collection_projection_rules::project_disjoint_collection_iteration_facts(&facts.texts,&facts.starts,&facts.counts,&facts.labels,&facts.targets,facts.root,markers);
 if plan.first()!=Some(&0.0)||plan.len()<8{return refused(1);}
 let Some(header)=plan[1..8].iter().map(|value|position(*value)).collect::<Option<Vec<_>>>()else{return refused(1)};
 let [size,edge_count,function_count,parameter_count,main,snapshot_count,callback_count]=header[..]else{return refused(1)};
 let mut at=8;let mut next=||{let value=plan.get(at).copied();at+=1;value};
 let mut roles=Vec::new();let mut operations=Vec::new();let mut kinds=Vec::new();let mut owners=Vec::new();
 let text=|identity:f64|position(identity).and_then(|at|facts.texts.get(at)).map_or("",String::as_str);
 for _ in 0..size{
  let (Some(role),Some(operation),Some(kind),Some(owner))=(next(),next(),next(),next())else{return refused(1)};
  roles.push(super::disjoint_collection_projection_rules::read_disjoint_collection_projection_role(role));
  operations.push(super::disjoint_collection_projection_rules::read_disjoint_collection_projection_operation(role,operation,text(operation)));
  kinds.push(super::disjoint_collection_projection_rules::read_disjoint_collection_projection_kind(role,text(kind)));owners.push(owner);
 }
 let mut edge_rows=Vec::new();
 for _ in 0..edge_count{
  let (Some(source),Some(label),Some(auxiliary),Some(target))=(next(),next(),next(),next())else{return refused(1)};
  edge_rows.push((source,super::disjoint_collection_projection_rules::read_disjoint_collection_projection_edge(label,auxiliary,text(auxiliary)),target));
 }
 let mut functions=Vec::new();let mut bodies=Vec::new();
 for _ in 0..function_count{let (Some(root),Some(body))=(next(),next())else{return refused(1)};functions.push(root);bodies.push(body);}
 let mut parameter_rows=Vec::new();for _ in 0..parameter_count{let (Some(owner),Some(slot))=(next(),next())else{return refused(1)};parameter_rows.push((owner,slot));}
 let mut snapshots=Vec::new();for _ in 0..snapshot_count{let (Some(node),Some(source))=(next(),next())else{return refused(1)};snapshots.extend([node,source]);}
 let mut callbacks=Vec::new();for _ in 0..callback_count{let (Some(node),Some(source))=(next(),next())else{return refused(1)};callbacks.extend([node,source]);}
 let mut starts=Vec::new();let mut counts=Vec::new();let mut labels=Vec::new();let mut targets=Vec::new();
 for node in 0..size{starts.push(number(labels.len()));for(source,label,target)in &edge_rows{if *source==number(node){labels.push(label.clone());targets.push(*target);}}counts.push(number(labels.len())-starts[node]);}
 let mut parameter_starts=Vec::new();let mut parameter_counts=Vec::new();let mut parameters=Vec::new();
 for owner in &functions{let start=number(parameters.len());parameter_starts.push(start);for(actual,slot)in &parameter_rows{if owner==actual{parameters.push(*slot);}}parameter_counts.push(number(parameters.len())-start);}
 let result=super::disjoint_collection_iteration_rules::prove_disjoint_collection_iterations(&roles,&operations,&kinds,&owners,&starts,&counts,&labels,&targets,&functions,&parameter_starts,&parameter_counts,&parameters,&bodies,number(main),&snapshots,&callbacks);
 if result.first()!=Some(&0.0){return refused(if result.first()==Some(&2.0){2}else{1});}
 let Some(accepted)=result.get(1).and_then(|value|position(*value))else{return refused(1)};
 let Some(safe)=result.get(3+accepted..)else{return refused(1)};
 let mut proof=Proof{error:0,array_callbacks:true,collection_mappers:true};
 for pair in callbacks.chunks_exact(2){if !safe.contains(&pair[0]){if position(pair[1]).and_then(|at|kinds.get(at)).is_some_and(|kind|kind=="array"){proof.array_callbacks=false;}else{proof.collection_mappers=false;}}}
 proof
}

pub(crate) fn check_program_markers(program:&mut Program,markers:&[(String,String)])->Result<()>{
 if markers.is_empty(){return Ok(());}
 let names:Vec<_>=markers.iter().map(|(name,_)|name.clone()).collect();
 if read_program_proof(program,&names).error!=0{return Err(unsupported("closed collection iteration","the actual lowered loop needs bounded, known callable effects and disjoint collection allocations",None));}
 let mut value=serde_json::to_value(&*program).expect("checked program serialization");let mut found=BTreeSet::new();
 fn normalize(value:&mut Value,markers:&[(String,String)],found:&mut BTreeSet<String>)->Result<()>{
  match value{
   Value::Object(fields)=>{
    if fields.get("k").and_then(Value::as_str)==Some("collectionItems")&&let Some(op)=fields.get("op").and_then(Value::as_str)&&!super::disjoint_collection_iteration_rules::read_disjoint_collection_iteration_marker_role(op).is_empty(){
     let Some((name,operation))=markers.iter().find(|(name,_)|name==op)else{return Err(unsupported("closed collection iteration","an unregistered private collection marker cannot escape checking",None));};
     found.insert(name.clone());fields.insert("op".into(),Value::String(operation.clone()));
    }
    for (key,child) in fields{if !["type","domain","span","modulePath"].contains(&key.as_str()){normalize(child,markers,found)?;}}
   }
   Value::Array(values)=>for child in values{normalize(child,markers,found)?;},_=>{}
  }Ok(())
 }
 normalize(&mut value,markers,&mut found)?;
 if found.len()!=markers.len(){return Err(unsupported("closed collection iteration","every source marker needs an actual checked loop consumer",None));}
 *program=serde_json::from_value(value).expect("marker normalization preserves every checked program field");Ok(())
}
