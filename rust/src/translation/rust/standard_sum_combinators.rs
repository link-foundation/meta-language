//! Actual standard source values into the shared eager/lazy combinator plans.
use super::{Options,Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,TokenKind,Type,unsupported,type_error};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::rust_standard_sum_combinator_rules as rules;
use crate::translation::rust_standard_sum_method_rules as callback_rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
use crate::translation::frontend_rules::ml_array::number_index;

fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust standard sum combinator",reason,span)}
fn count(value:usize)->f64{u32::try_from(value).map_or(f64::INFINITY,f64::from)}
struct CombinatorCapture{role:String,name:String,value:SExpr,origin:SourceOrigin}
struct CombinatorCallback{value:SExpr,origin:SourceOrigin,result:SourceOrigin}
impl RustParser{
    fn check_combinator_argument(&self,left:&SourceOrigin,right:&SourceOrigin,span:Option<Span>)->Result<()>{
        self.origins.argument(left,right,span)?;let identity=self.origins.standard_sum_source_identity(left);
        if identity.is_none()||identity!=self.origins.standard_sum_source_identity(right){return Err(fail("the retained source argument must preserve its complete actual type and owner",span));}Ok(())
    }
    fn read_combinator_right_operand(&mut self,left:&SourceOrigin,method:&str,path:&[String])->Result<Option<SExpr>>{
        let start=self.peek();if start.kind!=TokenKind::Identifier{return Ok(None);}
        let mut segments=vec![start.value.clone()];let mut offset=1;
        while self.cursor.is_at("::",offset)&&self.cursor.tokens.get(self.cursor.index+offset+1).is_some_and(|token|token.kind==TokenKind::Identifier){
            if segments.len()>=128{return Err(fail("the direct constructor path exceeds its finite source budget",Some(start.span())));}
            segments.push(self.cursor.tokens[self.cursor.index+offset+1].value.clone());offset+=2;
        }
        if !self.cursor.is_at("(",offset){return Ok(None);}
        let Some((owner,variant))=self.resolve_standard_sum_source_constructor(&segments,path)else{return Ok(None);};
        let source=left.standard_sum_source.as_deref().expect("actual source sum");if owner!=source.owner{return Ok(None);}
        let plan=rules::plan_rust_standard_sum_combinator_right_arguments(&owner,method,&variant);if plan.is_empty(){return Ok(None);}
        for _ in 0..offset{self.cursor.advance();}self.cursor.expect("(",Some("constrained standard constructor"))?;
        if self.cursor.is(")"){return Err(type_error("the actual standard result constructor needs one field",Some(start.span())));}
        let expected=self.standard_sum_source.expected_origin.take();let argument=self.expr(path,Options::default());self.standard_sum_source.expected_origin=expected;let argument=argument?;
        if self.cursor.eat(",").is_some()&&!self.cursor.is(")"){return Err(type_error("the actual standard result constructor needs one field",Some(start.span())));}
        let close=self.cursor.expect(")",Some("constrained standard constructor"))?;let span=Some(Span::new(start.start,close.end));
        if !self.cursor.is(")")&&!self.cursor.is(","){return Err(fail("partial result constraints apply only to the complete direct right operand; a surrounding operation needs its own complete source type",span));}
        let actual=self.origins.origin(&argument,path);if !matches!(self.origins.standard_sum_ownership_facts(&actual).profile.as_str(),"copy"|"owned")||actual.ty.is_none()&&actual.fields.is_none(){return Err(fail("the constrained constructor field needs its actual complete immutable Copy source type or supported owned source type",span));}
        let mut arguments=Vec::new();for role in plan{arguments.push(match role.as_str(){"argument"=>actual.clone(),"left value"=>source.arguments[0].clone(),"left error"=>source.arguments[1].clone(),_=>return Err(fail("the actual receiver constructor constraint is incomplete",span))});}
        let result=self.instantiate_standard_sum_source(&owner,arguments,span)?;let sum=result.standard_sum_source.as_deref().expect("actual constrained result");
        let expression=SExpr::new(SNode::App{func:Box::new(SExpr::new(SNode::Name{path:sum.constructor_paths[&variant].clone()},span)),args:vec![argument.clone()]},span);
        self.capture_standard_sum_constructor(&expression,&result,&[argument],path,span)?;Ok(Some(expression))
    }
    fn read_combinator_callback(&mut self,source:&SourceOrigin,method:&str,path:&[String],span:Option<Span>)->Result<CombinatorCallback>{
        let sum=source.standard_sum_source.as_deref().expect("actual callback receiver");let field=rules::read_rust_standard_sum_combinator_callback_argument(&sum.owner,method);
        let parameters=if field<0.0{Vec::new()}else{vec![sum.arguments[number_index(field).expect("actual fallback child")].clone()]};
        let value=if rules::accept_rust_standard_sum_combinator_receiver_context(&sum.owner,method){
            let mut expected=SourceOrigin::with_type(Type::Function{params:parameters.iter().map(|origin|origin.ty.clone().expect("actual source parameter")).collect(),ret:Box::new(source.ty.clone().expect("actual source result"))},path,Vec::new());expected.copy_evidence=Some(());expected.callable_parameters=Some(parameters.clone());expected.callable_result=Some(Box::new(source.clone()));self.read_expected_copy_closure(path,Some(&expected))?
        }else{self.read_parameter_only_copy_closure(path,&parameters)?};
        let origin=self.origins.origin(&value,path);let actual=origin.callable_parameters.as_deref().unwrap_or(&[]);let result=origin.callable_result.as_deref();
        let parameter_count=match origin.ty.as_ref(){Some(Type::Function{params,..})=>count(params.len()),_=>-1.0};
        let error=callback_rules::read_rust_standard_sum_method_callback_error(origin.ty.as_ref().map_or("",Type::kind),parameter_count,count(parameters.len()),count(origin.wrappers.len()),origin.direct_copy_value(),result.is_some_and(|result|result.direct_copy_value()&&self.origins.standard_sum_source_identity(result).is_some()));
        if !error.is_empty(){return Err(fail(&error,value.span.or(span)));}if actual.len()!=parameters.len(){return Err(fail("the fallback must retain every actual source parameter",value.span.or(span)));}
        for(actual,expected)in actual.iter().zip(&parameters){self.check_combinator_argument(actual,expected,value.span.or(span))?;}
        Ok(CombinatorCallback{value,origin:origin.clone(),result:result.cloned().expect("actual callback result")})
    }
    fn combinator_local(&mut self,captures:&[CombinatorCapture],role:&str,span:Option<Span>)->Result<SExpr>{
        let capture=captures.iter().find(|capture|capture.role==role).ok_or_else(||fail("the actual combinator capture is absent",span))?;
        let value=SExpr::new(SNode::Name{path:vec![capture.name.clone()]},span);self.origins.mark(&value,capture.origin.clone());Ok(value)
    }
    fn combinator_pattern(&mut self,origin:&SourceOrigin,variant:&str,span:Option<Span>)->(SPattern,Option<SExpr>){
        let sum=origin.standard_sum_source.as_deref().expect("actual pattern sum");let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,variant);let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,variant);let binding=(count==1.0).then(||self.fresh_product_binding());
        let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name{path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[number_index(field).expect("actual pattern field")].clone());value});
        (SPattern{node:SPatternNode::Ctor{path:sum.constructor_paths[variant].clone(),args:binding.into_iter().map(|name|SPattern{node:SPatternNode::Bind{name},span}).collect()},span},payload)
    }
    pub(super) fn read_standard_sum_source_combinator(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
        if !self.ordinary_completion_context(){return Ok(None);}let actual=self.origins.origin(receiver,path);let Some(source)=self.origins.standard_sum_source_origin(&actual)else{return Ok(None);};let left=source.standard_sum_source.as_deref().expect("actual source receiver");
        if rules::read_rust_standard_sum_combinator_arity(&left.owner,&method.value)<0.0{return Ok(None);}let version=rules::read_rust_standard_sum_combinator_version_error(&left.owner,&method.value,self.source_version_minor);if !version.is_empty(){return Err(fail(&version,Some(method.span())));}
        let left_role=self.origins.standard_sum_ownership_facts(&source).profile;
        if !source.wrappers.is_empty()||!rules::accept_rust_standard_sum_combinator_ownership(&left_role,&method.value){return Err(fail("the receiver needs its actual direct Copy or supported eager owned standard sum",receiver.span));}
        let start=receiver.span.map_or(method.start,|span|span.start);let initial=Some(Span::new(start,method.end));self.cursor.expect("(",Some("standard sum combinator"))?;
        self.origins.check_owned_origin(&actual,receiver.span)?;
        let receiver_origin=if left_role=="owned"{self.owned_standard_sum_method_capture(receiver,path)?}else{actual};
        let roles=rules::read_rust_standard_sum_combinator_capture_roles(&method.value);let mut captures=vec![CombinatorCapture{role:"receiver".to_owned(),name:self.fresh_product_binding(),value:receiver.clone(),origin:receiver_origin.clone()}];
        let right=if roles.get(1).is_some_and(|role|role=="callback"){
            let callback=self.read_combinator_callback(&source,&method.value,path,initial)?;let result=callback.result.clone();captures.push(CombinatorCapture{role:"callback".to_owned(),name:self.fresh_product_binding(),value:callback.value,origin:callback.origin});result
        }else{
            let value=if let Some(value)=self.read_combinator_right_operand(&source,&method.value,path)?{value}else{let expected=self.standard_sum_source.expected_origin.take();self.standard_sum_source.expected_origin=rules::accept_rust_standard_sum_combinator_receiver_context(&left.owner,&method.value).then(||source.clone());let value=self.expr(path,Options::default());self.standard_sum_source.expected_origin=expected;value?};
            let actual=self.origins.origin(&value,path);self.origins.check_owned_origin(&actual,value.span)?;
            let origin=if self.origins.standard_sum_ownership_facts(&actual).profile=="owned"{self.owned_standard_sum_method_capture(&value,path)?}else{actual};captures.push(CombinatorCapture{role:"argument".to_owned(),name:self.fresh_product_binding(),value,origin:origin.clone()});origin
        };
        self.cursor.eat(",");let close=self.cursor.expect(")",Some("standard sum combinator"))?;let span=Some(Span::new(start,close.end));
        let result=self.origins.standard_sum_source_origin(&right).filter(|result|result.wrappers.is_empty()&&rules::accept_rust_standard_sum_combinator_ownership(&self.origins.standard_sum_ownership_facts(result).profile,&method.value)&&result.standard_sum_source.as_deref().is_some_and(|sum|sum.owner==left.owner)).ok_or_else(||fail("the right operand or fallback needs its actual direct Copy or supported eager owned standard sum of the same family",span))?;
        let result_sum=result.standard_sum_source.as_deref().expect("actual output sum");for index in rules::read_rust_standard_sum_combinator_retained_arguments(&left.owner,&method.value){self.check_combinator_argument(&result_sum.arguments[number_index(index).expect("actual retained child")],&left.arguments[number_index(index).expect("actual retained child")],span)?;}
        let right_role=self.origins.standard_sum_ownership_facts(&result).profile;
        let left_moved=left_role=="copy"||self.origins.consume_owned_binding(&receiver_origin,receiver.span)?;
        let right_moved=right_role=="copy"||self.origins.consume_owned_binding(&right,captures.last().and_then(|capture|capture.value.span))?;
        let mut rows=Vec::new();let variants=sum_rules::read_rust_standard_sum_source_constructors(&left.owner);
        for variant in &variants{let others=if method.value=="xor"{variants.clone()}else{vec![String::new()]};for other in others{
            let(first,payload)=self.combinator_pattern(&source,variant,span);let second=(!other.is_empty()).then(||self.combinator_pattern(&result,&other,span));let branch=rules::read_rust_standard_sum_combinator_branch(&left.owner,variant,&method.value,&other);
            let body=match branch.as_str(){
                "left"=>{let target=&result_sum.constructor_paths[variant];SExpr::new(SNode::App{func:Box::new(SExpr::new(SNode::Name{path:target.clone()},span)),args:payload.clone().into_iter().collect()},span)},
                "right" if second.is_some()=>{let target=&result_sum.constructor_paths[&other];SExpr::new(SNode::App{func:Box::new(SExpr::new(SNode::Name{path:target.clone()},span)),args:second.as_ref().and_then(|(_,payload)|payload.clone()).into_iter().collect()},span)},
                "right"=>self.combinator_local(&captures,"argument",span)?,
                "callback"=>{let field=rules::read_rust_standard_sum_combinator_callback_argument(&left.owner,&method.value);SExpr::new(SNode::App{func:Box::new(self.combinator_local(&captures,"callback",span)?),args:if field<0.0{Vec::new()}else{vec![payload.expect("actual fallback payload")]}},span)},
                "construct success"|"construct failure"=>{let target=rules::read_rust_standard_sum_combinator_constructor(&left.owner,branch=="construct success");let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&left.owner,&target);SExpr::new(SNode::App{func:Box::new(SExpr::new(SNode::Name{path:result_sum.constructor_paths[&target].clone()},span)),args:if count==1.0{vec![payload.expect("actual preserved source payload")]}else{Vec::new()}},span)},_=>return Err(fail("the actual constructor pair has no represented combinator branch",span))};
            let mut patterns=vec![first];patterns.extend(second.map(|(pattern,_)|pattern));rows.push(SRow{patterns,body,span});
        }}
        let mut scrutinees=vec![self.combinator_local(&captures,"receiver",span)?];if method.value=="xor"{scrutinees.push(self.combinator_local(&captures,"argument",span)?);}
        let output=super::copy_closure_lifetimes::carry_value_union(self.owned_standard_sum_method_output(result,left_moved,right_moved),&[&receiver_origin,&right]);
        let mut body=SExpr::new(SNode::Match{scrutinees,rows},span);for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let{name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}self.origins.mark(&body,output);Ok(Some(body))
    }
}
