//! Structural surface transport driven by the generated shared expression
//! roles. Frames defer continuations until actual successful values, retaining
//! the JavaScript adapter's operand and fresh-binding order.
use serde::Serialize;
use serde_json::{Value,json};
use super::{Result,RustParser,SExpr,SourceOrigin,Type,unsupported};
use super::standard_sum_completions::PRIVATE_TRY;
use crate::translation::rust_expression_completion_rules as rules;

#[derive(Clone)]
pub(super) struct CompletionContinuation { pub(super) hole:String,pub(super) expression:SExpr }
fn encode<T:Serialize>(value:&T)->Result<Value>{serde_json::to_value(value).map_err(|_|unsupported("Rust expression completion","surface encoding failed",None))}
fn decode(value:Value)->Result<SExpr>{serde_json::from_value(value).map_err(|_|unsupported("Rust expression completion","surface reconstruction failed",None))}
fn replaced(value:&Value,hole:&str,replacement:&Value,budget:&mut usize)->Result<Value>{
    *budget+=1;if *budget>rules::read_rust_expression_completion_limit("inspection") as usize{return Err(unsupported("Rust expression completion","continuation substitution exceeds its finite budget",None));}
    if value.get("k").and_then(Value::as_str)==Some("name")&&value.get("path")==Some(&json!([hole])){return Ok(replacement.clone());}
    Ok(match value{Value::Array(values)=>Value::Array(values.iter().map(|value|replaced(value,hole,replacement,budget)).collect::<Result<_>>()?),Value::Object(values)=>Value::Object(values.iter().map(|(key,value)|Ok((key.clone(),replaced(value,hole,replacement,budget)?))).collect::<Result<_>>()?),_=>value.clone()})
}
impl CompletionContinuation {
    pub(super) fn apply(&self,value:&SExpr)->Result<SExpr>{decode(replaced(&encode(&self.expression)?,&self.hole,&encode(value)?,&mut 0)?)}
}
fn marker(value:&Value)->bool{value.get("k").and_then(Value::as_str)==Some("math")&&value.get("op").and_then(Value::as_str)==Some(PRIVATE_TRY)}
fn contains(value:&Value,budget:&mut usize)->Result<bool>{
    let mut pending=vec![value];while let Some(value)=pending.pop(){if !value.is_object()&&!value.is_array(){continue;}*budget+=1;if *budget>rules::read_rust_expression_completion_limit("inspection") as usize{return Err(unsupported("Rust expression completion","source inspection exceeds its finite budget",None));}if marker(value){return Ok(true);}match value{Value::Array(values)=>pending.extend(values),Value::Object(values)=>pending.extend(values.values()),_=>{}}}Ok(false)
}
pub(super) fn has_expression_completion(expression:&SExpr)->Result<bool>{contains(&encode(expression)?,&mut 0)}
fn known_type(origin:&SourceOrigin)->Option<Type>{origin.ty.clone().or_else(||origin.fields.as_ref().and_then(|fields|fields.iter().map(known_type).collect::<Option<Vec<_>>>()).map(|elements|Type::Tuple{elements}))}
#[derive(Clone)]
struct Slot{role:String,index:Option<usize>,wrapper:bool,value:Value}
#[derive(Clone)]
enum Continuation {
    Template(CompletionContinuation),
    Ordered{node:Value,slots:Vec<Slot>,at:usize,later:bool,next:Box<Self>,depth:usize},
    Propagate{ordinal:usize,next:Box<Self>,depth:usize},
    Conditional{node:Value,next:Box<Self>,depth:usize},
    ShortCircuit{node:Value,next:Box<Self>,depth:usize},
    Body{node:Value,role:String,next:Box<Self>,depth:usize},
    MatchRows{next:Box<Self>,packet:Option<(usize,Failure)>,depth:usize},
    PacketSuccess{node:Value,next:Box<Self>},
    PacketCondition{node:Value,levels:usize,next:Box<Self>,failed:Failure,depth:usize},
    PacketBody{node:Value,levels:usize,next:Box<Self>,failed:Failure,depth:usize},
}
#[derive(Clone)]
enum Failure {
    Expression{node:Value,next:Box<Continuation>,depth:usize},
    Packet{node:Value,levels:usize,next:Box<Continuation>,failed:Box<Self>,depth:usize},
}
struct Transport<'a>{parser:&'a mut RustParser,path:&'a [String],source_result:&'a SourceOrigin,returned:CompletionContinuation,work:usize,inspection:usize,residual_origins:Vec<SourceOrigin>}
impl Transport<'_>{
    fn charge(&mut self,depth:usize)->Result<()>{self.work+=1;if self.work>rules::read_rust_expression_completion_limit("work") as usize||depth>rules::read_rust_expression_completion_limit("depth") as usize{return Err(unsupported("Rust expression completion","continuation construction exceeds its finite node/depth budget",None));}Ok(())}
    fn has(&mut self,value:&Value)->Result<bool>{contains(value,&mut self.inspection)}
    fn failure(&mut self,failed:Failure)->Result<SExpr>{match failed{Failure::Expression{node,next,depth}=>self.visit(node,*next,depth),Failure::Packet{node,levels,next,failed,depth}=>self.packet(node,levels,*next,*failed,depth)}}
    fn apply(&mut self,next:Continuation,value:Value)->Result<SExpr>{
        match next{
            Continuation::Template(next)=>next.apply(&decode(value)?),
            Continuation::Ordered{mut node,slots,at,later,next,depth}=>{
                let slot=&slots[at];
                let capture=later&&!rules::accept_rust_expression_completion_inert_leaf(value.get("k").and_then(Value::as_str).unwrap_or(""));
                let (actual,binding)=if capture{
                    let expression=decode(value.clone())?;let origin=self.parser.origins.origin(&expression,self.path);let ty=known_type(&origin).filter(|_|origin.proven).ok_or_else(||unsupported("Rust ordered completion capture","an earlier evaluated child needs its complete source type before a later propagation",expression.span))?;
                    let name=self.parser.fresh_product_binding();let mut reference=json!({"k":"name","path":[name]});if let Some(span)=value.get("span"){reference["span"]=span.clone();}
                    (reference,Some((name,ty)))
                }else{(value.clone(),None)};
                if let Some(index)=slot.index{if slot.wrapper{node[&slot.role][index]["value"]=actual;}else{node[&slot.role][index]=actual;}}else{node[&slot.role]=actual;}
                let body=self.ordered_step(node,slots,at+1,*next,depth)?;
                if let Some((name,ty))=binding{let mut result=json!({"k":"let","name":name,"type":ty,"value":value,"body":encode(&body)?});if let Some(span)=result["value"].get("span").cloned(){result["span"]=span;}decode(result)}else{Ok(body)}
            },
            Continuation::Propagate{ordinal,next,depth}=>{
                let attempt=self.parser.standard_sum_try_attempts.get(ordinal).cloned().ok_or_else(||unsupported("Rust question-mark completion","unknown actual source attempt",None))?;
                let continued=self.visit(encode(&attempt.payload)?,*next,depth+1)?;
                let (expression,origins)=self.parser.lower_standard_sum_try_completion(ordinal,decode(value)?,self.path,self.source_result,continued,&self.returned)?;self.residual_origins.extend(origins);Ok(expression)
            },
            Continuation::Conditional{mut node,next,depth}=>{
                node["cond"]=value;node["then"]=encode(&self.visit(node["then"].clone(),(*next).clone(),depth+1)?)?;node["else"]=encode(&self.visit(node["else"].clone(),*next,depth+1)?)?;decode(node)
            },
            Continuation::ShortCircuit{node,next,depth}=>{
                let short=rules::read_rust_expression_completion_short_circuit_value(node["op"].as_str().unwrap_or(""));let branch=self.visit(node["right"].clone(),(*next).clone(),depth+1)?;
                let mut literal=json!({"k":"bool","value":short});if let Some(span)=node.get("span"){literal["span"]=span.clone();}let done=self.apply(*next,literal)?;
                let mut result=json!({"k":"if","cond":value,"then":encode(if short{&done}else{&branch})?,"else":encode(if short{&branch}else{&done})?});if let Some(span)=node.get("span"){result["span"]=span.clone();}decode(result)
            },
            Continuation::Body{mut node,role,next,depth}=>{node[&role]=value;node["body"]=encode(&self.visit(node["body"].clone(),*next,depth+1)?)?;decode(node)},
            Continuation::MatchRows{next,packet,depth}=>{
                let mut node=value;let rows=if node["k"]=="match1"{"cases"}else{"rows"};if let Some(entries)=node.get_mut(rows).and_then(Value::as_array_mut){for row in entries{row["body"]=encode(&if let Some((levels,failed))=&packet{self.packet(row["body"].clone(),*levels,(*next).clone(),failed.clone(),depth+1)?}else{self.visit(row["body"].clone(),(*next).clone(),depth+1)?})?;}}decode(node)
            },
            Continuation::PacketSuccess{mut node,next}=>{node["args"][0]["value"]=value;self.apply(*next,node)},
            Continuation::PacketCondition{mut node,levels,next,failed,depth}=>{node["cond"]=value;node["then"]=encode(&self.packet(node["then"].clone(),levels,(*next).clone(),failed.clone(),depth+1)?)?;node["else"]=encode(&self.packet(node["else"].clone(),levels,*next,failed,depth+1)?)?;decode(node)},
            Continuation::PacketBody{mut node,levels,next,failed,depth}=>{node["value"]=value;node["body"]=encode(&self.packet(node["body"].clone(),levels,*next,failed,depth+1)?)?;decode(node)},
        }
    }
    fn ordered(&mut self,node:Value,roles:Vec<String>,next:Continuation,depth:usize)->Result<SExpr>{
        let mut slots=Vec::new();for role in roles{if let Some(Value::Array(values))=node.get(&role){for(index,value)in values.iter().enumerate(){let wrapper=value.get("k").is_none()&&value.get("value").is_some_and(Value::is_object);slots.push(Slot{role:role.clone(),index:Some(index),wrapper,value:if wrapper{value["value"].clone()}else{value.clone()}});}}else if let Some(value)=node.get(&role).filter(|value|value.is_object()){slots.push(Slot{role,index:None,wrapper:false,value:value.clone()});}}
        self.ordered_step(node,slots,0,next,depth)
    }
    fn ordered_step(&mut self,node:Value,slots:Vec<Slot>,at:usize,next:Continuation,depth:usize)->Result<SExpr>{
        if at==slots.len(){return self.apply(next,node);}let mut later=false;for entry in &slots[at+1..]{if self.has(&entry.value)?{later=true;break;}}
        let value=slots[at].value.clone();self.visit(value,Continuation::Ordered{node,slots,at,later,next:Box::new(next),depth},depth+1)
    }
    fn visit(&mut self,node:Value,next:Continuation,depth:usize)->Result<SExpr>{
        self.charge(depth)?;if !self.has(&node)?{return self.apply(next,node);}
        let kind=if marker(&node){PRIVATE_TRY}else{node.get("k").and_then(Value::as_str).unwrap_or("")};let op=node.get("op").and_then(Value::as_str).unwrap_or("");let form=rules::read_rust_expression_completion_form(kind,op);
        match form.as_str(){
            "propagate"=>{let ordinal=node["name"].as_str().and_then(|name|name.parse::<usize>().ok()).ok_or_else(||unsupported("Rust question-mark completion","invalid actual attempt identity",None))?;let operand=node.get("args").and_then(Value::as_array).filter(|args|args.len()==1).and_then(|args|args[0].get("value")).cloned().ok_or_else(||unsupported("Rust question-mark completion","missing complete operand",None))?;self.visit(operand,Continuation::Propagate{ordinal,next:Box::new(next),depth},depth+1)},
            "ordered"=>self.ordered(node.clone(),rules::read_rust_expression_completion_child_roles(kind),next,depth),
            "conditional"=>self.visit(node["cond"].clone(),Continuation::Conditional{node,next:Box::new(next),depth},depth+1),
            "short circuit"=>self.visit(node["left"].clone(),Continuation::ShortCircuit{node,next:Box::new(next),depth},depth+1),
            "binding"|"print"=>{let role=if form=="binding"{"value"}else{"expr"};self.visit(node[role].clone(),Continuation::Body{node,role:role.to_owned(),next:Box::new(next),depth},depth+1)},
            "match"=>{let role=if kind=="match1"{"scrutinee"}else{"scrutinees"};self.ordered(node,vec![role.to_owned()],Continuation::MatchRows{next:Box::new(next),packet:None,depth},depth)},
            "packet"=>{let operation=rules::read_rust_expression_completion_packet_operation(op);if operation=="result"{self.packet(node["args"][0]["value"].clone(),1,next.clone(),Failure::Expression{node:node["args"][1]["value"].clone(),next:Box::new(next),depth:depth+1},depth+1)}else if operation=="success"{self.visit(node["args"][0]["value"].clone(),Continuation::PacketSuccess{node,next:Box::new(next)},depth+1)}else{self.apply(next,node)}},
            _=>self.apply(next,node),
        }
    }
    fn packet(&mut self,node:Value,levels:usize,next:Continuation,failed:Failure,depth:usize)->Result<SExpr>{
        self.charge(depth)?;let kind=node.get("k").and_then(Value::as_str).unwrap_or("");let operation=if kind=="math"{rules::read_rust_expression_completion_packet_operation(node["op"].as_str().unwrap_or(""))}else{String::new()};
        match operation.as_str(){
            "success"=>return if levels==1{self.visit(node["args"][0]["value"].clone(),next,depth+1)}else{self.packet(node["args"][0]["value"].clone(),levels-1,next,failed,depth+1)},
            "failure"=>return self.failure(failed),
            "result"=>return self.packet(node["args"][0]["value"].clone(),levels+1,next.clone(),Failure::Packet{node:node["args"][1]["value"].clone(),levels,next:Box::new(next),failed:Box::new(failed),depth:depth+1},depth+1),
            _=>{},
        }
        if kind=="if"{return self.visit(node["cond"].clone(),Continuation::PacketCondition{node,levels,next:Box::new(next),failed,depth},depth+1);}
        if kind=="let"{return self.visit(node["value"].clone(),Continuation::PacketBody{node,levels,next:Box::new(next),failed,depth},depth+1);}
        if kind=="match"||kind=="match1"{let role=if kind=="match1"{"scrutinee"}else{"scrutinees"};return self.ordered(node,vec![role.to_owned()],Continuation::MatchRows{next:Box::new(next),packet:Some((levels,failed)),depth},depth);}
        Err(unsupported("Rust conditional packet completion","the actual packet producer needs its complete source continuation interpretation",None))
    }
}
impl RustParser{
    pub(super) fn lower_expression_completions(&mut self,expression:SExpr,path:&[String],source_result:&SourceOrigin,continued:CompletionContinuation,returned:CompletionContinuation)->Result<(SExpr,Vec<SourceOrigin>)>{
        let value=encode(&expression)?;let mut transport=Transport{parser:self,path,source_result,returned,work:0,inspection:0,residual_origins:Vec::new()};let expression=transport.visit(value,Continuation::Template(continued),0)?;Ok((expression,transport.residual_origins))
    }
    pub(super) fn refuse_standard_sum_private_completions<T:Serialize>(&self,value:&T)->Result<()>{
        let value=encode(value)?;if contains(&value,&mut 0)?{return Err(unsupported("Rust question-mark completion","this position requires its complete ordered expression interpretation and actual nearest callable ownership",None));}
        let mut pending=vec![&value];let mut budget=0;while let Some(value)=pending.pop(){budget+=1;if budget>rules::read_rust_expression_completion_limit("inspection") as usize{return Err(unsupported("Rust expression completion","final admission exceeds its finite budget",None));}if value.get("k").and_then(Value::as_str)==Some("name")&&value.get("path").and_then(Value::as_array).is_some_and(|path|path.iter().any(|part|part.as_str().is_some_and(|part|part.starts_with("#rust-ordinary-")))){return Err(unsupported("Rust expression completion","a private continuation hole escaped surface admission",None));}match value{Value::Array(values)=>pending.extend(values),Value::Object(values)=>pending.extend(values.values()),_=>{}}}Ok(())
    }
}
