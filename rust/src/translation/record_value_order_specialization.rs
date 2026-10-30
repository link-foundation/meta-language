//! Transport one declared opaque payload slot through the ordinary checker.
//! The wrapper remains present, including for Unit and nullable actual payloads.
use super::diagnostics::{Result, type_error};
use super::ir::{Decl, Program};
use super::surface::{SItem, SModule, SProgram};
use super::types::{Type, array, data};
use serde_json::{Value, json};

pub(super) struct Specialization {
    pub(super) library: Program,
    pub(super) ty: Type,
    pub(super) entry: String,
    pub(super) constructor: String,
    pub(super) pair_ty: Type,
    pub(super) pair_entry: String,
    pub(super) pair_constructor: String,
}
fn failure(message: &str) -> super::diagnostics::TranslationError {
    type_error(message, None)
}
pub(super) fn specialize(element: &Type, namespace: &str) -> Result<Specialization> {
    let template =
        crate::read_structured_data_lino(include_str!("../data/record-value-order-template.lino"))
            .map_err(|error| failure(&error))?;
    if template["version"] != 1
        || template["model"]
            != "ordinary record value ordering with one opaque payload type parameter 1"
    {
        return Err(failure(
            "record value ordering needs its complete closed source template",
        ));
    }
    let mut surface: SProgram = serde_json::from_value(template["surface"].clone())
        .map_err(|error| failure(&error.to_string()))?;
    if surface.language != super::Language::JavaScript
        || !surface.externals.is_empty()
        || surface
            .main
            .as_ref()
            .is_some_and(|main| !main.effects.is_empty())
    {
        return Err(failure(
            "record value ordering needs its complete closed source template",
        ));
    }
    let parameter = &template["parameter"];
    let owner_name = parameter["data"].as_str().ok_or_else(|| {
        failure("record value ordering needs its one actual opaque payload owner")
    })?;
    let constructor = parameter["constructor"].as_str().ok_or_else(|| {
        failure("record value ordering needs its one actual opaque payload owner")
    })?;
    let field = parameter["field"].as_str().ok_or_else(|| {
        failure("record value ordering needs its one actual opaque payload owner")
    })?;
    let mut owners = 0;
    for item in &mut surface.items {
        if let SItem::Data(owner) = item
            && owner.name == owner_name
        {
            owners += 1;
            if owner.ctors.len() != 1
                || owner.ctors[0].name != constructor
                || owner.ctors[0].fields.len() != 1
                || owner.ctors[0].fields[0].name != field
            {
                return Err(failure(
                    "record value ordering needs its one actual opaque payload owner",
                ));
            }
            owner.ctors[0].fields[0].ty = element.clone();
        }
    }
    if owners != 1 {
        return Err(failure(
            "record value ordering needs its one actual opaque payload owner",
        ));
    }
    surface.items = vec![SItem::Module(SModule {
        name: namespace.to_owned(),
        items: surface.items,
        span: None,
    })];
    surface.main = None;
    let library = super::check::check_program(&surface)?;
    let ty = data(format!("{namespace}.{owner_name}"));
    let entry = format!(
        "{namespace}.{}",
        template["entry"]
            .as_str()
            .ok_or_else(|| failure("record value ordering has no actual source entry"))?
    );
    let shape = library.declaration(&format!("{namespace}.{owner_name}"));
    let function = library.declaration(&entry);
    let shape_valid = matches!(shape,Some(Decl::Data(shape))if shape.ctors.len()==1&&shape.ctors[0].name==constructor&&shape.ctors[0].fields.len()==1&&shape.ctors[0].fields[0].ty.same(element));
    let function_valid = matches!(function,Some(Decl::Fn(function))if function.params.len()==2&&function.params[0].ty.same(&array(ty.clone()))&&function.params[1].ty.same(&array(Type::Float))&&function.ret.same(&array(ty.clone())));
    if !shape_valid || !function_valid {
        return Err(failure(
            "record value specialization changed its owned source signature",
        ));
    }
    let pair = &template["pair"];
    let pair_name = pair["data"]
        .as_str()
        .ok_or_else(|| failure("own entry pairing changed its owned source signature"))?;
    let pair_constructor = pair["constructor"]
        .as_str()
        .ok_or_else(|| failure("own entry pairing changed its owned source signature"))?;
    let pair_entry_name = pair["entry"]
        .as_str()
        .ok_or_else(|| failure("own entry pairing changed its owned source signature"))?;
    let pair_ty = data(format!("{namespace}.{pair_name}"));
    let pair_entry = format!("{namespace}.{pair_entry_name}");
    let pair_owner = library.declaration(&format!("{namespace}.{pair_name}"));
    let pair_function = library.declaration(&pair_entry);
    let pair_owner_valid = matches!(pair_owner,Some(Decl::Data(shape))if shape.ctors.len()==1&&shape.ctors[0].name==pair_constructor&&shape.ctors[0].fields.len()==2&&shape.ctors[0].fields[0].name=="key"&&shape.ctors[0].fields[0].ty.same(&Type::String)&&shape.ctors[0].fields[1].name=="value"&&shape.ctors[0].fields[1].ty.same(&ty));
    let pair_function_valid = matches!(pair_function,Some(Decl::Fn(function))if function.params.len()==2&&function.params[0].ty.same(&array(Type::String))&&function.params[1].ty.same(&array(ty.clone()))&&function.ret.same(&array(pair_ty.clone())));
    if !pair_owner_valid || !pair_function_valid {
        return Err(failure(
            "own entry pairing changed its owned source signature",
        ));
    }
    Ok(Specialization {
        library,
        ty,
        entry,
        constructor: constructor.to_owned(),
        pair_ty,
        pair_entry,
        pair_constructor: pair_constructor.to_owned(),
    })
}

pub(super) fn lower(
    node: &Value,
    args: &[Value],
    specialization: &Specialization,
    prefix: &str,
    ordinal: &mut usize,
) -> Result<Value> {
    if args.len() != 2 {
        return Err(failure(
            "record value ordering needs every actual owned operand",
        ));
    }
    let values_type = args[0]["type"].clone();
    let element = values_type["element"].clone();
    let positions = json!({"kind":"array","element":{"kind":"float"}});
    let wrapped = json!({"kind":"array","element":specialization.ty});
    let mut fresh = |role: &str| {
        *ordinal += 1;
        format!("{prefix}{role}{}", *ordinal)
    };
    let capture_name = fresh("recordValueOperands");
    let capture_type = json!({"kind":"record","fields":[{"name":"fields","type":values_type},{"name":"order","type":positions}]});
    let capture = json!({"k":"var","name":capture_name,"type":capture_type});
    let fields =
        json!({"k":"recordField","object":capture,"field":"fields","index":0,"type":values_type});
    let order =
        json!({"k":"recordField","object":capture,"field":"order","index":1,"type":positions});
    let parameter_name = fresh("recordValue");
    let parameter = json!({"k":"var","name":parameter_name,"type":element});
    let wrapper = json!({"k":"ctor","data":match &specialization.ty{Type::Data{name}=>name,_=>unreachable!("actual opaque wrapper owner")},"ctor":specialization.constructor,"args":[parameter],"type":specialization.ty});
    let wrap = json!({"k":"lambda","params":[{"name":parameter_name,"type":element}],"body":wrapper,"type":{"kind":"function","params":[element],"ret":specialization.ty}});
    let values = json!({"k":"method","op":"map","object":fields,"args":[wrap],"type":wrapped});
    let ordered =
        json!({"k":"call","fn":specialization.entry,"args":[values,order],"type":wrapped});
    let wrapper_name = fresh("recordValueWrapper");
    let payload_name = fresh("recordValuePayload");
    let payload = json!({"k":"var","name":payload_name,"type":element});
    let body = json!({"k":"match","scrutinee":{"k":"var","name":wrapper_name,"type":specialization.ty},"cases":[{"pattern":{"k":"ctor","data":match &specialization.ty{Type::Data{name}=>name,_=>unreachable!("actual opaque wrapper owner")},"ctor":specialization.constructor,"binds":[payload_name]},"body":payload}],"type":element});
    let unwrap = json!({"k":"lambda","params":[{"name":wrapper_name,"type":specialization.ty}],"body":body,"type":{"kind":"function","params":[specialization.ty],"ret":element}});
    let result =
        json!({"k":"method","op":"map","object":ordered,"args":[unwrap],"type":values_type});
    let mut output = json!({"k":"let","name":capture_name,"value":{"k":"record","entries":[{"name":"fields","value":args[0]},{"name":"order","value":args[1]}],"type":capture_type},"body":result,"type":node["type"]});
    if let Some(span) = node.get("span").filter(|span| !span.is_null()) {
        output["span"] = span.clone();
    }
    Ok(output)
}

/// Adapt original declared constructors; source owns pairing admission/order.
pub(super) fn lower_entries(
    node: &Value,
    args: &[Value],
    specialization: &Specialization,
    prefix: &str,
    ordinal: &mut usize,
) -> Result<Value> {
    if args.len() != 2 {
        return Err(failure(
            "own entry pairing needs every actual owned operand",
        ));
    }
    let values_type = args[1]["type"].clone();
    let element = values_type["element"].clone();
    let keys_type = json!({"kind":"array","element":{"kind":"string"}});
    let text = json!({"kind":"string"});
    let wrapped = json!({"kind":"array","element":specialization.ty});
    let pair_type = json!({"kind":"tuple","elements":[text,element]});
    let mut fresh = |role: &str| {
        *ordinal += 1;
        format!("{prefix}{role}{}", *ordinal)
    };
    let capture_name = fresh("entryOperands");
    let capture_type = json!({"kind":"record","fields":[{"name":"keys","type":keys_type},{"name":"values","type":values_type}]});
    let capture = json!({"k":"var","name":capture_name,"type":capture_type});
    let keys =
        json!({"k":"recordField","object":capture,"field":"keys","index":0,"type":keys_type});
    let fields =
        json!({"k":"recordField","object":capture,"field":"values","index":1,"type":values_type});
    let parameter_name = fresh("entryValue");
    let parameter = json!({"k":"var","name":parameter_name,"type":element});
    let data = match &specialization.ty {
        Type::Data { name } => name,
        _ => unreachable!("actual opaque payload owner"),
    };
    let pair_data = match &specialization.pair_ty {
        Type::Data { name } => name,
        _ => unreachable!("actual source entry owner"),
    };
    let wrapper = json!({"k":"ctor","data":data,"ctor":specialization.constructor,"args":[parameter],"type":specialization.ty});
    let wrap = json!({"k":"lambda","params":[{"name":parameter_name,"type":element}],"body":wrapper,"type":{"kind":"function","params":[element],"ret":specialization.ty}});
    let values = json!({"k":"method","op":"map","object":fields,"args":[wrap],"type":wrapped});
    let paired_type = json!({"kind":"array","element":specialization.pair_ty});
    let paired =
        json!({"k":"call","fn":specialization.pair_entry,"args":[keys,values],"type":paired_type});
    let entry_name = fresh("ownEntry");
    let key_name = fresh("entryKey");
    let wrapper_name = fresh("entryWrapper");
    let payload_name = fresh("entryPayload");
    let key = json!({"k":"var","name":key_name,"type":text});
    let payload = json!({"k":"var","name":payload_name,"type":element});
    let project = json!({"k":"match","scrutinee":{"k":"var","name":wrapper_name,"type":specialization.ty},"cases":[{"pattern":{"k":"ctor","data":data,"ctor":specialization.constructor,"binds":[payload_name]},"body":{"k":"tuple","items":[key,payload],"type":pair_type}}],"type":pair_type});
    let body = json!({"k":"match","scrutinee":{"k":"var","name":entry_name,"type":specialization.pair_ty},"cases":[{"pattern":{"k":"ctor","data":pair_data,"ctor":specialization.pair_constructor,"binds":[key_name,wrapper_name]},"body":project}],"type":pair_type});
    let unwrap = json!({"k":"lambda","params":[{"name":entry_name,"type":specialization.pair_ty}],"body":body,"type":{"kind":"function","params":[specialization.pair_ty],"ret":pair_type}});
    let mut output = json!({"k":"let","name":capture_name,"value":{"k":"record","entries":[{"name":"keys","value":args[0]},{"name":"values","value":args[1]}],"type":capture_type},"body":{"k":"method","op":"map","object":paired,"args":[unwrap],"type":node["type"]},"type":node["type"]});
    if let Some(span) = node.get("span").filter(|span| !span.is_null()) {
        output["span"] = span.clone();
    }
    Ok(output)
}
