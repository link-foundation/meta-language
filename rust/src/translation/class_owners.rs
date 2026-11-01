//! Actual JSON/type transport for the generated persistent class-owner policy.
use super::{diagnostics::{Result,type_error},ir::Program,types::Type};
use serde_json::{Value,json};
use super::javascript_class_source_rules::ml_string::Text;
fn fail(message:&str)->super::diagnostics::TranslationError{type_error(format!("JavaScript class ownership: {message}"),None)}
fn position(value:f64)->Option<usize>{if value>=0.0&&value.is_finite()&&value.fract()==0.0{value.to_string().parse().ok()}else{None}}
pub(crate) fn owners_from_rows(rows:&[Vec<Text>])->Result<Value>{
 let mut owners=Vec::new();for row in rows{
  let plan=super::javascript_class_source_rules::read_java_script_closed_class_owner_row(row);
  if plan.first()!=Some(&0.0)||plan.len()!=5{return Err(fail("the source-owned class row is malformed"));}
  let shape=plan[1..].iter().map(|value|position(*value)).collect::<Option<Vec<_>>>().ok_or_else(||fail("the source-owned class row has invalid coordinates"))?;
  let fields=row.get(shape[0]..shape[0]+shape[1]).ok_or_else(||fail("the actual class fields are missing"))?.iter().map(ToString::to_string).collect::<Vec<_>>();
  let mut methods=Vec::new();for at in 0..shape[3]{let index=shape[2]+2*at;methods.push(json!({"name":row.get(index).ok_or_else(||fail("the actual method is missing"))?.to_string(),"helper":row.get(index+1).ok_or_else(||fail("the actual method helper is missing"))?.to_string()}));}
  owners.push(json!({"constructor":row[0].to_string(),"brand":row[1].to_string(),"representation":row[2].to_string(),"fields":fields,"methods":methods}));
 }Ok(json!({"schemaVersion":1,"language":"JavaScript","model":super::javascript_class_owner_rules::read_java_script_class_owner_model(),"owners":owners}))
}
pub(crate) fn source_record_owner(witness:Option<&Value>,fields:&[String])->Result<Option<String>>{
 let Some(witness)=witness else{return Ok(None)};
 let owners=witness.get("owners").and_then(Value::as_array).ok_or_else(||fail("complete actual class owner records are required"))?;
 let constructors=owners.iter().map(|owner|owner.get("constructor").and_then(Value::as_str).map(str::to_owned).ok_or_else(||fail("actual constructor names are required"))).collect::<Result<Vec<_>>>()?;
 let brands=owners.iter().map(|owner|owner.get("brand").and_then(Value::as_str).map(str::to_owned).ok_or_else(||fail("actual class brand identities are required"))).collect::<Result<Vec<_>>>()?;
 let owner=super::javascript_class_owner_rules::read_java_script_class_record_owner(&constructors,&brands,fields);
 if owner=="#ambiguous"{return Err(fail("a record cannot combine different actual class brands"));}Ok(if owner.is_empty(){None}else{Some(owner)})
}
pub(crate) fn validate_records(value:&Value)->Result<()>{
 let facts=super::surface_reference_facts::create_surface_reference_facts(value);
 let keys=facts.values.iter().map(|value|if value.get("kind").is_some(){serde_json::from_value::<Type>((*value).clone()).map(|ty|ty.key()).unwrap_or_default()}else{String::new()}).collect::<Vec<_>>();
 let error=super::javascript_class_owner_rules::read_java_script_class_owner_error(&facts.forms,&facts.texts,&facts.starts,&facts.counts,&facts.labels,&facts.targets,&keys,facts.root);
 if error.is_empty(){Ok(())}else{Err(fail(&error))}
}
pub(crate) fn validate_program(program:&Program)->Result<()>{validate_records(&serde_json::to_value(program).map_err(|error|fail(&error.to_string()))?)}
pub(crate) fn accept_type_observation(ty:&Type,witness:Option<&Value>,operation:&str)->bool{
 let Type::Record{source_class_owner:Some(owner),reference:true,..}=ty else{return false};
 let Some(owners)=witness.and_then(|value|value.get("owners")).and_then(Value::as_array)else{return false};
 owners.iter().any(|value|value.get("constructor").and_then(Value::as_str)==Some(owner.as_str())&&super::javascript_class_owner_rules::accept_java_script_class_owner_observation(value.get("representation").and_then(Value::as_str).unwrap_or(""),if operation=="comparison"{"identity"}else{operation}))
}
