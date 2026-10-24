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
    Ok(Specialization {
        library,
        ty,
        entry,
        constructor: constructor.to_owned(),
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
