//! Actual receiver/callable origins into source-generated method decisions.
//! Capture and match construction reuse the ordinary common surface nodes.
use super::{BOOL,Options,Result,RustParser,SArrayItem,SExpr,SNode,SourceOrigin,Span,Token,Type,UNIT,unsupported};
use crate::translation::surface::{SPattern,SPatternNode,SRow};
use crate::translation::source_signatures::SourceOwner;
use crate::translation::rust_standard_sum_method_rules as rules;
use crate::translation::rust_standard_sum_source_rules as sum_rules;
use crate::translation::rust_contextual_closure_rules as contextual;
use crate::translation::rust_panic_value_rules as panic_rules;

struct MethodCapture {name:String,value:SExpr,origin:SourceOrigin}
struct MethodCallback {value:SExpr,origin:SourceOrigin,result:SourceOrigin}
fn fail(reason:&str,span:Option<Span>)->super::TranslationError {unsupported("Rust standard sum method",reason,span)}

impl RustParser {
    fn standard_sum_method_callback(&mut self,path:&[String],parameter:Option<&SourceOrigin>,result:Option<&SourceOrigin>,span:Option<Span>)->Result<MethodCallback>{
        let parameters=parameter.into_iter().cloned().collect::<Vec<_>>();
        let mut expected=SourceOrigin::with_type(Type::Function {params:parameters.iter().map(|origin|origin.ty.clone().expect("actual callback parameter type")).collect(),ret:Box::new(result.and_then(|origin|origin.ty.clone()).unwrap_or(UNIT))},path,Vec::new());
        expected.copy_evidence=Some(());
        expected.callable_parameters=Some(parameters.clone());
        expected.callable_result=result.cloned().map(Box::new);
        let value=if result.is_some(){
            self.read_expected_copy_closure(path,Some(&expected))?
        }else if let Some(value)=self.read_primitive_function_value(path,Some(&expected))?{
            // The primitive item adapter consumes the actual parameter origin;
            // the placeholder result is not used to infer a mapper result.
            value
        }else{
            let mut closure=["|","||","move"].iter().any(|role|self.cursor.is(role));
            if !closure && self.cursor.is("(") {
                let mut count=64usize;
                loop {
                    let length=count.min(self.cursor.tokens.len()-self.cursor.index);
                    let syntax=(0..length).map(|index|["(","|","||","move"].into_iter().find(|role|self.cursor.is_at(role,index)).unwrap_or("").to_owned()).collect::<Vec<_>>();
                    let depth=contextual::find_rust_contextual_closure_parenthesis_count(&syntax);
                    if depth>0.0 {closure=true;break;}
                    if depth!=-2.0 || length<count {break;}
                    count=count.checked_mul(2).expect("actual source closure token window");
                }
            }
            if closure {self.read_expected_copy_closure_result_mode(path,Some(&expected),true)?}
            else {
                let previous=self.standard_sum_source.expected_origin.take();
                let value=self.expr(path,Options::default());
                self.standard_sum_source.expected_origin=previous;
                value?
            }
        };
        let origin=self.origins.origin(&value,path);
        let actual=origin.callable_parameters.as_deref().unwrap_or(&[]);
        let output=origin.callable_result.as_deref();
        let parameter_count=match origin.ty.as_ref(){Some(Type::Function {params,..})=>params.len() as f64,_=>-1.0};
        let error=rules::read_rust_standard_sum_method_callback_error(origin.ty.as_ref().map_or("",Type::kind),parameter_count,parameters.len() as f64,origin.wrappers.len() as f64,origin.direct_copy_value(),output.is_some_and(|output|output.direct_copy_value()&&self.origins.standard_sum_source_identity(output).is_some()));
        if !error.is_empty(){return Err(fail(&error,value.span.or(span)));}
        if actual.len()!=parameters.len(){return Err(fail("the callback must retain every actual source parameter origin",value.span.or(span)));}
        for (actual,expected) in actual.iter().zip(&parameters){
            self.origins.argument(actual,expected,value.span.or(span))?;
            let identity=self.origins.standard_sum_source_identity(actual);
            if identity.is_none() || identity!=self.origins.standard_sum_source_identity(expected){return Err(fail("the callback parameter must preserve its complete actual source type",value.span.or(span)));}
        }
        let output=output.expect("shared callback policy proved an actual result");
        if let Some(result)=result{
            self.origins.argument(output,result,value.span.or(span))?;
            let identity=self.origins.standard_sum_source_identity(output);
            if identity.is_none() || identity!=self.origins.standard_sum_source_identity(result){return Err(fail("the callback result must preserve its complete actual source type",value.span.or(span)));}
        }
        Ok(MethodCallback {value,origin:origin.clone(),result:output.clone()})
    }

    fn standard_sum_method_construct(&mut self,origin:&SourceOrigin,variant:&str,args:Vec<SExpr>,span:Option<Span>)->Result<SExpr>{
        let path=origin.standard_sum_source.as_deref().and_then(|sum|sum.constructor_paths.get(variant)).cloned().ok_or_else(||fail("the result constructor must belong to the actual source sum",span))?;
        let name=SExpr::new(SNode::Name {path},span);
        let node=if args.is_empty(){name}else{SExpr::new(SNode::App {func:Box::new(name),args},span)};
        self.origins.mark(&node,origin.clone());Ok(node)
    }
    fn standard_sum_method_local(&mut self,capture:&MethodCapture,span:Option<Span>)->SExpr{
        let node=SExpr::new(SNode::Name {path:vec![capture.name.clone()]},span);
        self.origins.mark(&node,capture.origin.clone());node
    }

    /// The postfix reader calls this before consuming the method's opening (.
    pub(super) fn read_standard_sum_source_method(&mut self,receiver:&SExpr,method:&Token,path:&[String])->Result<Option<SExpr>>{
        if !self.ordinary_completion_context(){return Ok(None);}
        let actual=self.origins.origin(receiver,path);
        let Some(source)=self.origins.standard_sum_source_origin(&actual) else {return Ok(None);};
        let sum=source.standard_sum_source.as_deref().expect("actual source sum");
        let arity=rules::read_rust_standard_sum_method_arity(&sum.owner,&method.value);
        if arity<0.0{return Ok(None);}
        let version_error=rules::read_rust_standard_sum_method_source_version_error(&sum.owner,&method.value,self.source_version_minor);
        if !version_error.is_empty(){return Err(fail(&version_error,Some(method.span())));}
        if !source.wrappers.is_empty() || !source.direct_copy_value(){return Err(fail("the receiver needs its actual direct immutable Copy standard sum",receiver.span));}
        let start=receiver.span.map_or(method.start,|span|span.start);
        let result_policy=rules::read_rust_standard_sum_method_result(&sum.owner,&method.value);
        self.cursor.expect("(",Some("standard sum method"))?;
        let mut captures=vec![MethodCapture {name:self.fresh_product_binding(),value:receiver.clone(),origin:source.clone()}];
        let mut callable=None;
        let callback_argument=rules::read_rust_standard_sum_method_callback_argument(&sum.owner,&method.value);
        let message_role=rules::read_rust_standard_sum_method_panic_message_role(&sum.owner,&method.value);
        if message_role=="supplied message"{
            let previous=self.standard_sum_source.expected_origin.take();
            let argument=self.expr(path,Options::default());self.standard_sum_source.expected_origin=previous;
            let argument=argument?;let origin=self.origins.origin(&argument,path);
            let error=panic_rules::read_rust_panic_message_source_error(&method.value,origin.ty.as_ref().map_or("",Type::kind),origin.text_source_model.unwrap_or(""),origin.wrappers.len() as f64,origin.proven,origin.wrappers.iter().all(|layer|layer=="reference"));
            if !error.is_empty(){return Err(fail(&error,argument.span));}
            captures.push(MethodCapture {name:self.fresh_product_binding(),value:argument,origin});
        }else if method.value=="unwrap_or"{
            let previous=std::mem::replace(&mut self.standard_sum_source.expected_origin,Some(sum.arguments[0].clone()));
            let argument=self.expr(path,Options::default());
            self.standard_sum_source.expected_origin=previous;
            let argument=argument?;
            let error=super::copy_closures::result_context_error(&argument,&sum.arguments[0]);
            if !error.is_empty(){return Err(fail(&error,argument.span));}
            self.origins.argument(&self.origins.origin(&argument,path),&sum.arguments[0],argument.span)?;
            captures.push(MethodCapture {name:self.fresh_product_binding(),value:argument,origin:sum.arguments[0].clone()});
        }else if callback_argument!=-2.0{
            let expected=self.standard_sum_source.expected_origin.as_ref().and_then(|origin|self.origins.standard_sum_source_origin(origin));
            let matching=expected.as_ref().filter(|origin|origin.standard_sum_source.as_deref().is_some_and(|target|target.owner==sum.owner));
            let result=if method.value=="unwrap_or_else"{Some(&sum.arguments[0])}else if method.value=="map"{matching.and_then(|origin|origin.standard_sum_source.as_deref()).and_then(|sum|sum.arguments.first())}else{matching};
            let parameter=if callback_argument<0.0{None}else{Some(&sum.arguments[callback_argument as usize])};
            let function=self.standard_sum_method_callback(path,parameter,result,Some(Span::new(start,method.end)))?;
            captures.push(MethodCapture {name:self.fresh_product_binding(),value:function.value.clone(),origin:function.origin.clone()});
            callable=Some(function);
        }
        if arity>0.0{self.cursor.eat(",");}
        let close=self.cursor.expect(")",Some("standard sum method"))?;
        let span=Some(Span::new(start,close.end));
        let result_origin=match result_policy.as_str(){
            "boolean"=>{let mut origin=SourceOrigin::with_type(BOOL,path,Vec::new());origin.source_owner=Some(SourceOwner {kind:"primitive".to_owned(),path:vec!["bool".to_owned()],arguments:Vec::new()});origin},
            "payload"=>sum.arguments[0].clone(),
            "mapped sum"=>{let mut arguments=vec![callable.as_ref().expect("shared callable method").result.clone()];arguments.extend(sum.arguments.iter().skip(1).cloned());self.instantiate_standard_sum_source(&sum.owner,arguments,span)?},
            "bound sum"=>{
                let result=self.origins.standard_sum_source_origin(&callable.as_ref().expect("shared callable method").result).filter(|origin|origin.wrappers.is_empty()&&origin.standard_sum_source.as_deref().is_some_and(|result|result.owner==sum.owner)).ok_or_else(||fail("and_then must return an actual direct standard sum of the same family",span))?;
                let target=result.standard_sum_source.as_deref().expect("actual result sum");
                if sum.arguments.len()>1 && self.origins.standard_sum_source_identity(&sum.arguments[1])!=self.origins.standard_sum_source_identity(&target.arguments[1]){return Err(fail("and_then must preserve the complete actual Result error type",span));}
                result
            },
            _=>return Err(fail("the shared method result policy is absent",span)),
        };
        let mut rows=Vec::new();
        for variant in sum_rules::read_rust_standard_sum_source_constructors(&sum.owner){
            let disposition=rules::read_rust_standard_sum_method_branch(&sum.owner,&variant,&method.value);
            let field=sum_rules::read_rust_standard_sum_source_field_argument(&sum.owner,&variant);
            let arity=sum_rules::read_rust_standard_sum_source_constructor_arity(&sum.owner,&variant);
            let binding=(arity==1.0).then(||self.fresh_product_binding());
            let payload=binding.as_ref().map(|binding|{let node=SExpr::new(SNode::Name {path:vec![binding.clone()]},span);self.origins.mark(&node,sum.arguments[field as usize].clone());node});
            let body=match disposition.as_str(){
                "true"|"false"=>SExpr::new(SNode::Bool {value:disposition=="true"},span),
                "payload"=>payload.clone().expect("shared payload branch"),
                "default"=>self.standard_sum_method_local(&captures[1],span),
                "panic"=>{
                    let message=if message_role=="supplied message"{self.standard_sum_method_local(&captures[1],span)}else{SExpr::new(SNode::Str {value:panic_rules::read_rust_option_panic_message(&method.value,"")},span)};
                    SExpr::new(SNode::Math {op:panic_rules::read_rust_panic_value_operation(),name:"Rust standard Option panic message".to_owned(),args:vec![SArrayItem::new(message,false)]},span)
                },
                "invoke fallback"=>{let function=self.standard_sum_method_local(&captures[1],span);SExpr::new(SNode::App {func:Box::new(function),args:if callback_argument<0.0{Vec::new()}else{vec![payload.clone().expect("shared error payload")]}},span)},
                "construct mapped"=>{let function=self.standard_sum_method_local(&captures[1],span);let value=SExpr::new(SNode::App {func:Box::new(function),args:vec![payload.clone().expect("shared success payload")]},span);self.standard_sum_method_construct(&result_origin,&variant,vec![value],span)?},
                "invoke mapper"=>{let function=self.standard_sum_method_local(&captures[1],span);SExpr::new(SNode::App {func:Box::new(function),args:vec![payload.clone().expect("shared success payload")]},span)},
                "forward failure"=>self.standard_sum_method_construct(&result_origin,&variant,payload.into_iter().collect(),span)?,
                _=>return Err(fail("the shared constructor branch policy is absent",span)),
            };
            let pattern=SPattern {node:SPatternNode::Ctor {path:sum.constructor_paths.get(&variant).cloned().ok_or_else(||fail("the source constructor must belong to the actual source sum",span))?,args:binding.into_iter().map(|name|SPattern {node:SPatternNode::Bind {name},span}).collect()},span};
            rows.push(SRow {patterns:vec![pattern],body,span});
        }
        let captured=self.standard_sum_method_local(&captures[0],span);
        let mut body=SExpr::new(SNode::Match {scrutinees:vec![captured],rows},span);
        for capture in captures.into_iter().rev(){body=SExpr::new(SNode::Let {name:capture.name,ty:capture.origin.ty.map(Into::into),value:Box::new(capture.value),body:Box::new(body)},span);}
        self.origins.mark(&body,result_origin);
        Ok(Some(body))
    }
}
