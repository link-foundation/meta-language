//! Runtime source-value lease inventory. Decisions come from shared JS rules.
use std::collections::{HashMap,HashSet};
#[derive(Default)]
pub(super) struct MutableRecordLoans {
 pub(super) roots:HashMap<String,String>,
 pub(super) lexicals:HashMap<String,String>,
 pub(super) loans:HashMap<String,ValueLoan>,
 pub(super) active:HashMap<String,HashSet<String>>,
 pub(super) nodes:usize,
 pub(super) text:usize,
}
pub(super) struct ValueLoan {pub(super) root:String,pub(super) dependencies:Vec<String>,pub(super) lexical:bool}
