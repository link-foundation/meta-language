//! Actual nested source origins projected into source-generated ctor plans.
use super::{Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::rust_standard_sum_nested_rules as rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust nested standard sum",reason,span)}

impl RustParser{
    fn standard_sum_nested_result(&mut self,source:&SourceOrigin,inner:&SourceOrigin,profile:&str,span:Option<Span>)->Result<SourceOrigin>{
        let outer=source.standard_sum_source.as_deref().expect("actual outer sum");let nested=inner.standard_sum_source.as_deref().expect("actual inner sum");let mut stack=Vec::new();
        for term in rules::plan_rust_standard_sum_nested_result(profile){
            match term.as_str(){
                "inner"=>stack.push(inner.clone()),"inner success"=>stack.push(nested.arguments[0].clone()),"inner error"=>stack.push(nested.arguments[1].clone()),"outer error"=>stack.push(outer.arguments[1].clone()),
                _=>{
                    let count=sum_rules::read_rust_standard_sum_source_argument_count(&term);if count<0.0||(stack.len() as f64)<count{return Err(fail("the shared nested result plan lost an actual type argument",span));}
                    let arguments=stack.split_off(stack.len()-count as usize);stack.push(self.instantiate_standard_sum_source(&term,arguments,span)?);
                }
            }
        }
        if stack.len()!=1{return Err(fail("the shared nested result plan needs one complete actual sum",span));}Ok(stack.pop().expect("one actual result"))
    }

    fn standard_sum_nested_branch(&mut self,result:&SourceOrigin,plan:&[String],payload:Option<&SExpr>,span:Option<Span>)->Result<SExpr>{
        let mut stack=Vec::new();
        for term in plan{
            if term=="payload"{stack.push(payload.cloned().ok_or_else(||fail("the nested branch has no actual payload",span))?);continue;}
            let position=rules::read_rust_standard_sum_nested_construction_position(term);let variant=rules::read_rust_standard_sum_nested_construction_variant(term);
            let actual=(if position == -1.0{Some(result.clone())}else if position==0.0{result.standard_sum_source.as_deref().and_then(|sum|sum.arguments.first()).and_then(|origin|self.origins.standard_sum_source_origin(origin))}else{None}).ok_or_else(||fail("the shared nested construction needs its actual target sum owner",span))?;
            let sum=actual.standard_sum_source.as_deref().expect("actual target sum");let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);
            let path=sum.constructor_paths.get(&variant).cloned().ok_or_else(||fail("the shared nested constructor is absent from its actual datatype",span))?;
            if count<0.0||(stack.len() as f64)<count{return Err(fail("the shared nested construction lost its actual constructor fields",span));}
            let args=stack.split_off(stack.len()-count as usize);let name=SExpr::new(SNode::Name {path},span);let value=if args.is_empty(){name}else{SExpr::new(SNode::App {func:Box::new(name),args},span)};
            self.origins.mark(&value,actual);stack.push(value);
        }
        if stack.len()!=1{return Err(fail("the shared nested branch needs one complete result",span));}Ok(stack.pop().expect("one actual branch"))
    }

    fn standard_sum_nested_match(&mut self,origin:&SourceOrigin,scrutinee:&SExpr,inner:&SourceOrigin,result:&SourceOrigin,profile:&str,is_inner:bool,span:Option<Span>)->Result<SExpr>{
        let sum=origin.standard_sum_source.as_deref().expect("actual matched sum");let mut rows=Vec::new();
        for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
            let argument=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);let binding=(count==1.0).then(||self.fresh_product_binding());
            let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name {path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[argument as usize].clone());value});
            let plan=rules::plan_rust_standard_sum_nested_branch(profile,&sum.owner,&variant,is_inner);if plan.is_empty(){return Err(fail("the shared nested constructor branch is absent",span));}
            let body=if plan.len()==1&&plan[0]=="match inner"{self.standard_sum_nested_match(inner,payload.as_ref().ok_or_else(||fail("the outer constructor has no actual inner payload",span))?,inner,result,profile,true,span)?}else{self.standard_sum_nested_branch(result,&plan,payload.as_ref(),span)?};
            let path=sum.constructor_paths.get(&variant).cloned().ok_or_else(||fail("the actual matched constructor is absent",span))?;
            rows.push(SRow {patterns:vec![SPattern {node:SPatternNode::Ctor {path,args:binding.into_iter().map(|name|SPattern {node:SPatternNode::Bind {name},span}).collect()},span}],body,span});
        }
        let value=SExpr::new(SNode::Match {scrutinees:vec![scrutinee.clone()],rows},span);self.origins.mark(&value,result.clone());Ok(value)
    }

    pub(super) fn read_standard_sum_source_nested(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
        if !self.ordinary_completion_context(){return Ok(None);}
        let Some(source)=self.origins.standard_sum_source_origin(&self.origins.origin(receiver,path))else{return Ok(None);};let outer=source.standard_sum_source.as_deref().expect("actual outer sum");
        if rules::read_rust_standard_sum_nested_arity(&outer.owner,&method.value)<0.0{return Ok(None);}
        let error=rules::read_rust_standard_sum_nested_source_version_error(&outer.owner,&method.value,self.source_version_minor);if !error.is_empty(){return Err(fail(&error,Some(method.span())));}
        let inner=outer.arguments.first().and_then(|origin|self.origins.standard_sum_source_origin(origin)).filter(|origin|origin.wrappers.is_empty()&&origin.direct_copy_value());
        if !source.wrappers.is_empty()||!source.direct_copy_value()||inner.is_none(){return Err(fail("nested operations require actual complete direct Copy inner and outer standard sums",receiver.span));}
        let inner=inner.expect("actual admitted inner");let profile=rules::read_rust_standard_sum_nested_profile(&outer.owner,&method.value,&inner.standard_sum_source.as_deref().expect("actual inner sum").owner);
        if profile.is_empty(){return Err(fail("the actual inner standard sum does not match this nested operation",receiver.span));}
        self.cursor.expect("(",Some("nested standard sum"))?;let close=self.cursor.expect(")",Some("nested standard sum"))?;let span=Some(Span::new(receiver.span.map_or(method.start,|span|span.start),close.end));
        let result=self.standard_sum_nested_result(&source,&inner,&profile,span)?;let capture=self.fresh_product_binding();let local=SExpr::new(SNode::Name {path:vec![capture.clone()]},span);self.origins.mark(&local,source.clone());
        let matched=self.standard_sum_nested_match(&source,&local,&inner,&result,&profile,false,span)?;
        let body=SExpr::new(SNode::Let {name:capture,ty:source.ty.clone().map(Into::into),value:Box::new(receiver.clone()),body:Box::new(matched)},span);self.origins.mark(&body,result);Ok(Some(body))
    }
}
