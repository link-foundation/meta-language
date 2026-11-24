//! Actual inference-term transport for the complete collection construction plans.

use super::term_facts::TermFacts;
use super::{Env, Inference, Result, SExpr, SNode, Span, Term, Type, HashMap};
use crate::translation::collection_construction_rules as rules;
use crate::translation::collection_copy_rules as copy_rules;
use crate::translation::frontend_rules::ml_array::number_index;

fn endpoint(facts: &TermFacts, identity: f64) -> Term {
    facts
        .term(identity)
        .expect("the collection plan selects an actual inference term")
}
pub(super) struct CollectionConstructionInput{
    node:*const SExpr,kind:String,input:Term,array:Term,result:Term,snapshot:Term,span:Option<Span>,
}

pub(super) fn normalize_collection_copy_input(node:&mut SExpr,snapshot:&Type,types:&mut HashMap<*const SExpr,Type>)->(*const SExpr,*const SExpr){
    if copy_rules::read_collection_copy_source_normalization()!="arrayFrom"{unreachable!("the shared copy profile selects its existing eager source wrapper");}
    let child=match &mut node.node{SNode::Set{items,..}=>items,SNode::Map{entries,..}=>entries,_=>unreachable!("only proved collection constructors normalize")};
    let old_pointer=std::ptr::from_ref(child.as_ref());let original=std::mem::replace(child.as_mut(),SExpr::new(SNode::Unit,None));let span=original.span;
    *child=Box::new(SExpr::new(SNode::ArrayFrom{args:vec![original],ty:Some(snapshot.clone())},span));
    let original_pointer=if let SNode::ArrayFrom{args,..}=&child.node{std::ptr::from_ref(&args[0])}else{unreachable!("actual eager snapshot wrapper")};
    if let Some(original_type)=types.remove(&old_pointer){types.insert(original_pointer,original_type);}
    types.insert(std::ptr::from_ref(child.as_ref()),snapshot.clone());
    (old_pointer,original_pointer)
}

impl Inference {
    fn settle_collection_copy_input(&mut self,constraint:&CollectionConstructionInput,settled:bool)->Result<bool>{
        let actual=self.resolve(&constraint.input);let role=copy_rules::read_collection_copy_inference_role(&constraint.kind,super::optional_values::kind(&actual),settled);
        if role=="wait"{return Ok(false);}
        if role=="copy"{self.unify(&constraint.input,&constraint.result,constraint.span)?;self.collection_copies.insert(constraint.node,constraint.snapshot.clone());}
        else{self.unify(&constraint.input,&constraint.array,constraint.span)?;}Ok(true)
    }
    pub(super) fn solve_collection_copy_inputs(&mut self,settled:bool)->Result<bool>{
        let mut waiting=Vec::new();let mut progress=false;
        for constraint in std::mem::take(&mut self.collection_construction_inputs){if self.settle_collection_copy_input(&constraint,settled)?{progress=true;}else{waiting.push(constraint);}}
        self.collection_construction_inputs=waiting;Ok(progress)
    }
    pub(super) fn collection_construction(
        &mut self,
        node: &SExpr,
        items: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        let expected = expected.map(|term| match self.resolve(term) {
            Term::Known(ty) => self.declared(Some(&ty)),
            term => term,
        });
        let facts = TermFacts::new(&[expected]);
        let plan = rules::plan_collection_construction_inference(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            node.node.kind(),
            facts.position(0),
        );
        let contextual = number_index(plan[1]) == Some(1);
        let first = if contextual {
            endpoint(&facts, plan[2])
        } else {
            self.fresh(false)
        };
        let (element, result) = match number_index(plan[0]) {
            Some(1) => (
                first.clone(),
                Term::Set {
                    element: Box::new(first),
                    reference: false,
                },
            ),
            Some(2) => {
                let second = if contextual {
                    endpoint(&facts, plan[3])
                } else {
                    self.fresh(false)
                };
                (
                    Term::Tuple(vec![first.clone(), second.clone()]),
                    Term::Map {
                        key: Box::new(first),
                        value: Box::new(second),
                        reference: false,
                    },
                )
            }
            _ => unreachable!("only actual collection constructors reach this adapter"),
        };
        let expected_items = Term::Array(Box::new(element));
        let actual_items = self.expr_expected(items, env, Some(&expected_items))?;
        let constraint=CollectionConstructionInput{node:std::ptr::from_ref(node),kind:node.node.kind().into(),input:actual_items,array:expected_items.clone(),result:result.clone(),snapshot:expected_items,span:node.span};
        if !self.settle_collection_copy_input(&constraint,false)?{self.collection_construction_inputs.push(constraint);}
        self.collections
            .insert(std::ptr::from_ref(node), (result.clone(), node.span));
        Ok(result)
    }
}
