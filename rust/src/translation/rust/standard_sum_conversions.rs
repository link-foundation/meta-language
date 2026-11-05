//! Actual source receivers/callable origins into source-generated conversion plans.
use super::{Options,Result,RustParser,SExpr,SNode,SourceOrigin,Span,Token,Type,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::rust_standard_sum_conversion_rules as rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;

struct ConversionCapture{name:String,value:SExpr,origin:SourceOrigin}
struct ConversionCallback{value:SExpr,origin:SourceOrigin,result:SourceOrigin}
fn fail(reason:&str,span:Option<Span>)->super::TranslationError{unsupported("Rust standard sum conversion",reason,span)}

impl RustParser{
    fn standard_sum_conversion_callback(&mut self,path:&[String],parameter:Option<&SourceOrigin>,span:Option<Span>)->Result<ConversionCallback>{
        let parameters=parameter.into_iter().cloned().collect::<Vec<_>>();
        let value=self.read_parameter_only_copy_closure(path,&parameters)?;
        let origin=self.origins.origin(&value,path);let actual=origin.callable_parameters.as_deref().unwrap_or_default();
        let output=origin.callable_result.as_deref();
        let count=match origin.ty.as_ref(){Some(Type::Function {params,..})=>params.len() as f64,_=>-1.0};
        let error=rules::read_rust_standard_sum_conversion_callback_error(origin.ty.as_ref().map_or("",Type::kind),count,parameters.len() as f64,origin.wrappers.len() as f64,origin.direct_copy_value(),output.is_some_and(|output|output.direct_copy_value()&&self.origins.standard_sum_source_identity(output).is_some()));
        if !error.is_empty(){return Err(fail(&error,value.span.or(span)));}
        if actual.len()!=parameters.len(){return Err(fail("the conversion callable must retain every actual parameter origin",value.span.or(span)));}
        for (actual,expected) in actual.iter().zip(&parameters){
            self.origins.argument(actual,expected,value.span.or(span))?;
            let identity=self.origins.standard_sum_source_identity(actual);if identity.is_none()||identity!=self.origins.standard_sum_source_identity(expected){return Err(fail("the conversion callback parameter must preserve its complete actual source type",value.span.or(span)));}
        }
        let output=output.expect("shared callback rule proved a complete result");
        Ok(ConversionCallback{value,origin:origin.clone(),result:output.clone()})
    }

    fn standard_sum_conversion_local(&mut self,capture:&ConversionCapture,span:Option<Span>)->SExpr{
        let value=SExpr::new(SNode::Name {path:vec![capture.name.clone()]},span);self.origins.mark(&value,capture.origin.clone());value
    }

    pub(super) fn read_standard_sum_source_conversion(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
        if !self.ordinary_completion_context(){return Ok(None);}
        let Some(source)=self.origins.standard_sum_source_origin(&self.origins.origin(receiver,path))else{return Ok(None);};
        let sum=source.standard_sum_source.as_deref().expect("actual source sum");let arity=rules::read_rust_standard_sum_conversion_arity(&sum.owner,&method.value);
        if arity<0.0{return Ok(None);}
        let version_error=rules::read_rust_standard_sum_conversion_source_version_error(&sum.owner,&method.value,self.source_version_minor);
        if !version_error.is_empty(){return Err(fail(&version_error,Some(method.span())));}
        if !source.wrappers.is_empty()||!source.direct_copy_value(){return Err(fail("the conversion receiver needs its actual direct immutable Copy standard sum",receiver.span));}
        let owner=rules::read_rust_standard_sum_conversion_owner(&sum.owner,&method.value);let plan=rules::plan_rust_standard_sum_conversion_arguments(&sum.owner,&method.value);let callback_argument=rules::read_rust_standard_sum_conversion_callback_argument(&sum.owner,&method.value);
        let start=receiver.span.map_or(method.start,|span|span.start);
        self.cursor.expect("(",Some("standard sum conversion"))?;
        let mut captures=vec![ConversionCapture{name:self.fresh_product_binding(),value:receiver.clone(),origin:source.clone()}];
        let mut argument=None;let mut argument_origin=None;let mut callback=None;
        if arity>0.0&&callback_argument == -2.0{
            let previous=self.standard_sum_source.expected_origin.take();
            let parsed=self.expr(path,Options::default());self.standard_sum_source.expected_origin=previous;let value=parsed?;
            argument_origin=Some(self.origins.origin(&value,path));argument=Some(value);
        }else if callback_argument != -2.0{
            let parameter=if callback_argument<0.0{None}else{sum.arguments.get(callback_argument as usize)};
            let function=self.standard_sum_conversion_callback(path,parameter,Some(Span::new(start,method.end)))?;
            argument_origin=Some(function.result.clone());callback=Some(function);
        }
        if arity>0.0{self.cursor.eat(",");}let close=self.cursor.expect(")",Some("standard sum conversion"))?;let span=Some(Span::new(start,close.end));
        let arguments=plan.into_iter().map(|index|if index<0.0{argument_origin.clone()}else{sum.arguments.get(index as usize).cloned()}).collect::<Option<Vec<_>>>().ok_or_else(||fail("the conversion result needs every actual source argument",span))?;
        if arguments.iter().any(|origin|!origin.direct_copy_value()){return Err(fail("an owned conversion output requires its actual consuming transfer profile",span));}
        let result=self.instantiate_standard_sum_source(&owner,arguments,span)?;let output=result.standard_sum_source.as_deref().expect("actual result sum");
        if let Some(value)=argument{captures.push(ConversionCapture{name:self.fresh_product_binding(),value,origin:output.arguments[1].clone()});}
        if let Some(function)=callback{captures.push(ConversionCapture{name:self.fresh_product_binding(),value:function.value,origin:function.origin});}
        let mut rows=Vec::new();
        for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
            let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);let count=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);
            let binding=(count==1.0).then(||self.fresh_product_binding());
            let payload=binding.as_ref().map(|name|{let value=SExpr::new(SNode::Name {path:vec![name.clone()]},span);self.origins.mark(&value,sum.arguments[field as usize].clone());value});
            let role=rules::read_rust_standard_sum_conversion_value(&sum.owner,&variant,&method.value);let target=rules::read_rust_standard_sum_conversion_constructor(&sum.owner,&variant,&method.value);
            let values=match role.as_str(){
                "payload"=>vec![payload.clone().expect("shared payload branch")],"empty"=>Vec::new(),
                "argument"=>vec![self.standard_sum_conversion_local(&captures[1],span)],
                "invoke callback"=>{let function=self.standard_sum_conversion_local(&captures[1],span);vec![SExpr::new(SNode::App {func:Box::new(function),args:if callback_argument<0.0{Vec::new()}else{vec![payload.clone().expect("shared callback argument")]}},span)]},
                _=>return Err(fail("the shared conversion branch value is absent",span)),
            };
            let constructor=output.constructor_paths.get(&target).cloned().ok_or_else(||fail("the conversion constructor must belong to its actual result datatype",span))?;
            let name=SExpr::new(SNode::Name {path:constructor},span);let body=if values.is_empty(){name}else{SExpr::new(SNode::App {func:Box::new(name),args:values},span)};self.origins.mark(&body,result.clone());
            rows.push(SRow {patterns:vec![SPattern {node:SPatternNode::Ctor {path:sum.constructor_paths.get(&variant).cloned().expect("actual source constructor"),args:binding.into_iter().map(|name|SPattern {node:SPatternNode::Bind {name},span}).collect()},span}],body,span});
        }
        let captured=self.standard_sum_conversion_local(&captures[0],span);let mut body=SExpr::new(SNode::Match {scrutinees:vec![captured],rows},span);
        for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let {name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}
        self.origins.mark(&body,result);Ok(Some(body))
    }
}
