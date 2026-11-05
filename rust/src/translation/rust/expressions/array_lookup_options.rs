//! Actual scalar source facts into the shared owned lookup disposition and
//! existing standard Option/array term adapters. Immediate consumers stay exact.
use super::{Result,RustParser,SExpr,SNode,Span,Token,TokenKind,Type,rust_fixed_type,type_error,unsupported};
use super::array_lookups::{project,selected_lookup,lookup_dimensions,lookup_result_origin,source_owners};
use crate::translation::rust_array_lookup_rules as lookup;
use crate::translation::rust_array_lookup_option_rules as rules;

impl RustParser{
    pub(super) fn read_array_lookup_option(&mut self,receiver:&SExpr,method:&Token,args:&[SExpr],range:Option<Span>,path:&[String])->Result<Option<SExpr>>{
        let copied=self.cursor.is(".")&&self.cursor.tokens.get(self.cursor.index+1).is_some_and(|token|token.kind==TokenKind::Identifier&&token.value=="copied")&&self.cursor.is_at("(",2)&&self.cursor.is_at(")",3);
        let next=if self.cursor.is_at(".",4){self.cursor.tokens.get(self.cursor.index+5).filter(|token|token.kind==TokenKind::Identifier).map_or("",|token|token.value.as_str())}else{""};
        if rules::read_rust_array_lookup_option_disposition(&method.value,self.ordinary_completion_context(),copied,next)!="owned scalar option"{return Ok(None);}
        let arity=lookup::read_rust_array_lookup_arity(&method.value,true);
        if u32::try_from(args.len()).map_or(f64::INFINITY,f64::from)!=arity{return Err(type_error(format!("{} needs {arity} actual array lookup arguments",method.value),range));}
        self.cursor.expect(".",Some("copied lookup"))?;self.cursor.identifier(Some("copied lookup"))?;self.cursor.expect("(",Some("copied lookup"))?;let close=self.cursor.expect(")",Some("copied lookup"))?;
        let range=Some(Span::new(receiver.span.or(range).map_or(method.start,|span|span.start),close.end));
        let actual=self.origins.origin(receiver,path);let source=actual.static_byte_array_read_view().unwrap_or_else(||actual.clone());
        let element=match &source.ty{Some(Type::Array{element,..})=>Some(element.as_ref().clone()),_=>None};
        let error=lookup::read_rust_array_lookup_receiver_error(source.ty.as_ref().map_or("",Type::kind),element.as_ref().map_or("",Type::kind),source.array_source_model.unwrap_or(""),source.proven,source.wrappers.iter().all(|layer|layer=="reference"||layer=="box"),source.array_element_copy_evidence,true);
        if !error.is_empty(){return Err(unsupported("Rust array lookup receiver",&error,range));}
        let element=element.ok_or_else(||unsupported("Rust array lookup result","a copied value needs its complete actual scalar element type",range))?;
        if arity>0.0&&!self.usize_index_expression(&args[0],path)?.is_empty(){return Err(unsupported("Rust array lookup index","the actual index needs its complete direct usize source meaning",args[0].span.or(range)));}
        self.origins.check_owned_origin(&actual,range)?;
        let owner=source_owners::scalar_owner(&source);let(bits,signed)=match &element{Type::Fixed{bits,signed}=>(bits.to_string(),*signed),_=>(String::new(),false)};
        let error=lookup::read_rust_array_lookup_element_owner_error(element.kind(),&bits,signed,&owner);if !error.is_empty(){return Err(unsupported("Rust array lookup result",&error,range));}
        let payload=lookup_result_origin(&element,&owner,false,path);
        let result=self.instantiate_standard_sum_source(&rules::read_rust_array_lookup_option_owner(),vec![payload],range)?;
        let sum=result.standard_sum_source.as_ref().expect("actual instantiated standard Option");
        let usize_type=rust_fixed_type("usize").expect("existing complete usize source type");let mut captures=Vec::new();
        for role in rules::read_rust_array_lookup_option_capture_roles(&method.value){
            let(value,ty)=match role.as_str(){"receiver"=>(receiver.clone(),source.ty.clone().expect("complete admitted source array")),"index"=>(args[0].clone(),usize_type.clone()),_=>return Err(type_error("unknown owned array lookup capture role",range))};
            captures.push((role,self.fresh_product_binding(),value,ty));
        }
        let local=|role:&str|->Result<SExpr>{let capture=captures.iter().find(|capture|capture.0==role).ok_or_else(||type_error("an owned lookup plan lost its actual capture",range))?;Ok(SExpr::new(SNode::Name{path:vec![capture.1.clone()]},capture.2.span.or(range)))};
        let mut values=lookup_dimensions(local("receiver")?,&usize_type,range);if arity>0.0{values.insert("index".to_owned(),local("index")?);}
        let condition=project(&lookup::plan_rust_array_lookup_condition(&method.value),&values,range)?;
        let selected=selected_lookup(local("receiver")?,project(&lookup::plan_rust_array_lookup_index(&method.value),&values,range)?,range);
        let construct=|present:bool,args:Vec<SExpr>|SExpr::new(SNode::App{func:Box::new(SExpr::new(SNode::Name{path:sum.constructor_paths[&rules::read_rust_array_lookup_option_variant(present)].clone()},range)),args},range);
        let mut body=SExpr::new(SNode::If{cond:Box::new(condition),then:Box::new(construct(true,vec![selected])),otherwise:Box::new(construct(false,Vec::new()))},range);
        for(_,name,value,ty)in captures.into_iter().rev(){body=SExpr::new(SNode::Let{name,ty:Some(ty.into()),value:Box::new(value),body:Box::new(body)},range);}
        self.origins.mark(&body,result);Ok(Some(body))
    }
}
