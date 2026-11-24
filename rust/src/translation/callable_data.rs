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
#[derive(Clone, Copy)]
struct CallableDataSubtree {
    facts: CallableDataFacts,
    height: usize,
}

fn node(
    kind: &str,
    children: Vec<CallableDataSubtree>,
    child_offset: usize,
    acyclic: bool,
) -> CallableDataSubtree {
    CallableDataSubtree {
        facts: CallableDataFacts {
            callable: rules::accept_callable_data_node(
                kind,
                &children
                    .iter()
                    .map(|child| child.facts.callable)
                    .collect::<Vec<_>>(),
            ),
            constructive: rules::accept_callable_data_constructive_node(
                kind,
                &children
                    .iter()
                    .map(|child| child.facts.constructive)
                    .collect::<Vec<_>>(),
            ),
            acyclic: acyclic && children.iter().all(|child| child.facts.acyclic),
        },
        height: children
            .iter()
            .map(|child| child.height + child_offset)
            .max()
            .unwrap_or(0),
    }
}

pub(crate) fn read_callable_data_facts<'a, F>(ty: &Type, lookup: &F) -> Result<CallableDataFacts>
where
    F: Fn(&str) -> Option<&'a DataDecl>,
{
    struct Traversal<'lookup, F> {
        lookup: &'lookup F,
        active: HashSet<String>,
        completed: HashMap<String, CallableDataSubtree>,
        visits: usize,
    }
    impl<'a, F: Fn(&str) -> Option<&'a DataDecl>> Traversal<'_, F> {
        fn check_bound(&self, depth: usize) -> Result<()> {
            let reason =
                rules::read_callable_data_traversal_error(depth as f64, self.visits as f64);
            if !reason.is_empty() {
                return Err(unsupported("callable data source profile", &reason, None));
            }
            Ok(())
        }

        fn walk(&mut self, ty: &Type, depth: usize) -> Result<CallableDataSubtree> {
            self.visits += 1;
            self.check_bound(depth)?;
            let Type::Data { name } = ty else {
                let children = ty
                    .children()
                    .iter()
                    .map(|child| self.walk(child, depth + 1))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(node(ty.kind(), children, 1, true));
            };
            if self.active.contains(name) {
                return Ok(node("cycle", Vec::new(), 0, false));
            }
            if let Some(cached) = self.completed.get(name) {
                self.check_bound(depth + cached.height)?;
                return Ok(*cached);
            }
            let Some(data) = (self.lookup)(name) else {
                return Ok(node("missing", Vec::new(), 0, false));
            };
            self.active.insert(name.clone());
            let mut children = Vec::new();
            for ctor in &data.ctors {
                let fields = ctor
                    .fields
                    .iter()
                    .map(|field| self.walk(&field.ty, depth + 1))
                    .collect::<Result<Vec<_>>>()?;
                children.push(node("constructor", fields, 1, true));
            }
            self.active.remove(name);
            let subtree = node("data", children, 0, true);
            if subtree.facts.acyclic {
                self.completed.insert(name.clone(), subtree);
            }
            Ok(subtree)
        }
    }
    let mut traversal = Traversal {
        lookup,
        active: HashSet::new(),
        completed: HashMap::new(),
        visits: 0,
    };
    Ok(traversal.walk(ty, 0)?.facts)
}
include!("../../tests/internal/translation/callable_data.rs");
