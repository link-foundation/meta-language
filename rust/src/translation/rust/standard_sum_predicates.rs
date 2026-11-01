//! Actual Copy callable/source facts into the shared predicate/mapping policy.
//! Captures retain argument order; constructor branches invoke only their own
//! callback. Unknown results use actual parameter-only closure source records.
use super::{BOOL,Options,Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,Type,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::source_signatures::SourceOwner;
use crate::translation::rust_standard_sum_predicate_rules as rules;
use crate::translation::rust_standard_sum_method_rules as callback_rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
use crate::translation::frontend_rules::ml_array::number_index;

fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust standard sum predicate",reason,span)}
fn count(value:usize)->f64{u32::try_from(value).map_or(f64::INFINITY,f64::from)}
struct PredicateCapture{role:String,name:String,value:SExpr,origin:SourceOrigin}
struct PredicateCallback{role:String,value:SExpr,origin:SourceOrigin,pending:bool,parameters:Vec<SourceOrigin>}
impl RustParser{
    fn predicate_complete_result(&self,origin:Option<&SourceOrigin>)->bool{origin.is_some_and(|origin|origin.direct_copy_value()&&self.origins.standard_sum_source_identity(origin).is_some())}
    fn predicate_same(&self,actual:&SourceOrigin,expected:&SourceOrigin,span:Option<Span>)->Result<()>{
        self.origins.argument(actual,expected,span)?;let identity=self.origins.standard_sum_source_identity(actual);
        if identity.is_none()||identity!=self.origins.standard_sum_source_identity(expected){return Err(fail("the callback or eager default must preserve its complete actual source result type",span));}Ok(())
    }
    fn read_predicate_callback(&mut self,role:&str,path:&[String],parameters:Vec<SourceOrigin>,result:Option<&SourceOrigin>,span:Option<Span>)->Result<PredicateCallback>{
        let value=if let Some(result)=result{
            let mut expected=SourceOrigin::with_type(Type::Function{params:parameters.iter().map(|origin|origin.ty.clone().expect("actual callback parameter")).collect(),ret:Box::new(result.ty.clone().expect("actual known callback result"))},path,Vec::new());
            expected.copy_evidence=Some(());expected.callable_parameters=Some(parameters.clone());expected.callable_result=Some(Box::new(result.clone()));
            self.read_expected_copy_closure(path,Some(&expected))?
        }else{self.read_parameter_only_copy_closure_deferred_result(path,&parameters)?};
        let origin=self.origins.origin(&value,path);let pending=matches!(&value.node,SNode::Lambda{ret:None,..})&&origin.ty.is_none();
        if pending{let error=rules::read_rust_standard_sum_predicate_deferred_error(value.node.kind(),true,origin.callable_parameters.as_ref().map_or(-1.0,|parameters|count(parameters.len())),count(parameters.len()),count(origin.wrappers.len()),value.span.is_some()&&origin.copy_closure_identity==value.span);if !error.is_empty(){return Err(fail(&error,value.span.or(span)));}}
        Ok(PredicateCallback{role:role.to_owned(),value,origin,pending,parameters})
    }
    fn finish_predicate_callback(&mut self,callback:&mut PredicateCallback,result:&SourceOrigin,path:&[String],span:Option<Span>)->Result<()>{
        if callback.pending{callback.value=self.finish_deferred_copy_closure(callback.value.clone(),result,path)?;callback.origin=self.origins.origin(&callback.value,path);callback.pending=false;}
        let origin=&callback.origin;let actual=origin.callable_parameters.as_deref().unwrap_or(&[]);let output=origin.callable_result.as_deref();
        let parameters=match origin.ty.as_ref(){Some(Type::Function{params,..})=>count(params.len()),_=>-1.0};
        let error=callback_rules::read_rust_standard_sum_method_callback_error(origin.ty.as_ref().map_or("",Type::kind),parameters,count(callback.parameters.len()),count(origin.wrappers.len()),origin.direct_copy_value(),self.predicate_complete_result(output));
        if !error.is_empty(){return Err(fail(&error,callback.value.span.or(span)));}
        if actual.len()!=callback.parameters.len(){return Err(fail("the callback must retain every actual by-value source parameter",callback.value.span.or(span)));}
        for(actual,expected)in actual.iter().zip(&callback.parameters){self.predicate_same(actual,expected,callback.value.span.or(span))?;}
        self.predicate_same(output.expect("actual completed callback result"),result,callback.value.span.or(span))
    }
    fn predicate_local(&mut self,captures:&[PredicateCapture],role:&str,span:Option<Span>)->Result<SExpr>{
        let capture=captures.iter().find(|capture|capture.role==role).ok_or_else(||fail("an actual method capture is absent",span))?;
        let value=SExpr::new(SNode::Name{path:vec![capture.name.clone()]},span);self.origins.mark(&value,capture.origin.clone());Ok(value)
    }
    pub(super) fn read_standard_sum_source_predicate(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
        if !self.ordinary_completion_context(){return Ok(None);}
        let Some(source)=self.origins.standard_sum_source_origin(&self.origins.origin(receiver,path))else{return Ok(None);};let sum=source.standard_sum_source.as_deref().expect("actual validated standard sum");
        let borrow=rules::read_rust_standard_sum_predicate_borrow_error(&sum.owner,&method.value);if !borrow.is_empty(){return Err(fail(&borrow,Some(method.span())));}
        let arity=rules::read_rust_standard_sum_predicate_arity(&sum.owner,&method.value);if arity<0.0{return Ok(None);}
        let version=rules::read_rust_standard_sum_predicate_version_error(&sum.owner,&method.value,self.source_version_minor);if !version.is_empty(){return Err(fail(&version,Some(method.span())));}
        if !source.wrappers.is_empty()||!source.direct_copy_value(){return Err(fail("the receiver needs its actual direct immutable Copy standard sum",receiver.span));}
        let start=receiver.span.map_or(method.start,|span|span.start);let initial=Some(Span::new(start,method.end));self.cursor.expect("(",Some("standard sum predicate"))?;
        let mut boolean=SourceOrigin::with_type(BOOL,path,Vec::new());boolean.source_owner=Some(SourceOwner{kind:"primitive".to_owned(),path:vec!["bool".to_owned()],arguments:Vec::new()});
        let roles=rules::read_rust_standard_sum_predicate_capture_roles(&method.value);let mut captures=vec![PredicateCapture{role:"receiver".to_owned(),name:self.fresh_product_binding(),value:receiver.clone(),origin:source.clone()}];
        let mut callbacks:Vec<PredicateCallback>=Vec::new();let mut default_value=None;let mut first_result=None;
        for(at,role)in roles.iter().enumerate().skip(1){
            if at>1{self.cursor.expect(",",Some("standard sum predicate arguments"))?;}
            if role=="default"{
                let expected=self.standard_sum_source.expected_origin.take();let value=self.expr(path,Options::default());self.standard_sum_source.expected_origin=expected;let value=value?;let origin=self.origins.origin(&value,path);
                if self.predicate_complete_result(Some(&origin)){first_result=Some(origin.clone());}default_value=Some(value.clone());captures.push(PredicateCapture{role:role.clone(),name:self.fresh_product_binding(),value,origin});
            }else{
                let field=rules::read_rust_standard_sum_predicate_callback_argument(&sum.owner,&method.value,role);
                let parameters=if field==-2.0{return Err(fail("the shared callback role is absent",initial));}else if field<0.0{Vec::new()}else{vec![sum.arguments[number_index(field).expect("actual source child index")].clone()]};
                let result=if role=="predicate"{Some(&boolean)}else if role=="mapper"{first_result.as_ref()}else{None};
                let callback=self.read_predicate_callback(role,path,parameters,result,initial)?;
                if role=="fallback"&&self.predicate_complete_result(callback.origin.callable_result.as_deref()){first_result=callback.origin.callable_result.as_deref().cloned();}
                captures.push(PredicateCapture{role:role.clone(),name:self.fresh_product_binding(),value:callback.value.clone(),origin:callback.origin.clone()});callbacks.push(callback);
            }
        }
        self.cursor.eat(",");let close=self.cursor.expect(")",Some("standard sum predicate"))?;let span=Some(Span::new(start,close.end));
        let result=if arity==1.0{boolean.clone()}else{
            let mapped=callbacks.iter().find(|callback|callback.role=="mapper").and_then(|callback|callback.origin.callable_result.as_deref());
            match rules::read_rust_standard_sum_predicate_result_source(self.predicate_complete_result(first_result.as_ref()),self.predicate_complete_result(mapped)).as_str(){"first result"=>first_result.clone().expect("actual first output"),"mapper result"=>mapped.cloned().expect("actual mapper output"),_=>return Err(fail("the actual default or callback must establish a complete immutable Copy result; unresolved siblings cannot acquire a guessed Unit type",span))}
        };
        for callback in &mut callbacks{let predicate=callback.role=="predicate";self.finish_predicate_callback(callback,if predicate{&boolean}else{&result},path,span)?;let capture=captures.iter_mut().find(|capture|capture.role==callback.role).expect("actual callback capture");capture.value=callback.value.clone();capture.origin=callback.origin.clone();}
        if let Some(value)=default_value{let error=super::copy_closures::result_context_error(&value,&result);if !error.is_empty(){return Err(fail(&error,value.span.or(span)));}let actual=self.origins.origin(&value,path);self.origins.argument(&actual,&result,value.span.or(span))?;if self.predicate_complete_result(Some(&actual)){self.predicate_same(&actual,&result,value.span.or(span))?;}captures.iter_mut().find(|capture|capture.role=="default").expect("actual eager default capture").origin=result.clone();}
        let mut rows=Vec::new();for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
            let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);let binding=(count==1.0).then(||self.fresh_product_binding());
            let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name{path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[number_index(field).expect("actual constructor field")].clone());value});
            let branch=rules::read_rust_standard_sum_predicate_branch(&sum.owner,&variant,&method.value);
            let body=match branch.as_str(){"true"|"false"=>SExpr::new(SNode::Bool{value:branch=="true"},span),"default"=>self.predicate_local(&captures,"default",span)?,"mapper"|"predicate"|"fallback"=>{let arguments=if rules::read_rust_standard_sum_predicate_callback_argument(&sum.owner,&method.value,&branch)<0.0{Vec::new()}else{vec![payload.expect("actual selected callback payload")]};SExpr::new(SNode::App{func:Box::new(self.predicate_local(&captures,&branch,span)?),args:arguments},span)},_=>return Err(fail("the actual standard constructor has no represented method branch",span))};
            let pattern=SPattern{node:SPatternNode::Ctor{path:sum.constructor_paths[&variant].clone(),args:binding.into_iter().map(|name|SPattern{node:SPatternNode::Bind{name},span}).collect()},span};rows.push(SRow{patterns:vec![pattern],body,span});
        }
        let local=self.predicate_local(&captures,"receiver",span)?;let mut body=SExpr::new(SNode::Match{scrutinees:vec![local],rows},span);
        for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let{name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}
        self.origins.mark(&body,result);Ok(Some(body))
    }
}
