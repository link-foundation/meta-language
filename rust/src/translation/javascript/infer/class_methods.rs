//! Existing inferred terms/constraints transported to shared source ownership.
use super::{Inference,Result,Term,ROOT};
use crate::translation::javascript_class_source_rules::{self as rules,ml_string::Text};
impl Inference{
 pub(super) fn closed_class_method_target(&self,receiver:&Term,method:&str)->String{
  let Term::Record(record)=receiver else{return String::new()};
  let fields=record.borrow().fields.iter().map(|(name,_)|Text::from_text(name)).collect::<Vec<_>>();
  rules::read_java_script_closed_class_method_target(&self.source_class_methods.iter().map(|value|Text::from_text(value)).collect::<Vec<_>>(),&fields,&Text::from_text(method)).to_string()
 }
 pub(super) fn solve_closed_class_receivers(&mut self)->Result<bool>{
  let before=self.unification_changes;
  for row in self.source_class_methods.clone().chunks_exact(3){
   let Some(helper)=self.signature(&[ROOT.to_owned(),row[2].clone()])else{continue};
   let Some(parameter)=helper.params.first()else{continue};
   for signature in self.signatures.values().cloned().collect::<Vec<_>>(){
    let owner=self.resolve(&signature.ret);
    let actual=matches!(&owner,Term::Record(record)if record.borrow().fields.iter().any(|(name,_)|name==&row[0]));
    if actual{self.unify(parameter,&owner,None)?;break;}
   }
  }Ok(self.unification_changes!=before)
 }
}
