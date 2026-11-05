use super::{
    BTreeMap, DynamicMetaLanguageOptions, DynamicParseOptions, MetaLanguageArchiveLimits, Value,
    read_structured_data_lino, rules,
};

pub(super) fn read<T>(
    path: Option<&String>,
    read_text: &mut T,
) -> Result<DynamicMetaLanguageOptions, String>
where
    T: FnMut(&str) -> Result<String, String>,
{
    let mut options = DynamicMetaLanguageOptions::default();
    let Some(path) = path else {
        return Ok(options);
    };
    let value = read_structured_data_lino(&read_text(path)?)?;
    let fields = value
        .as_object()
        .ok_or("unknown language configuration field")?;
    if fields
        .keys()
        .any(|key| !rules::accept_dynamic_language_configuration_field(key))
    {
        return Err("unknown language configuration field".to_owned());
    }
    if let Some(records) = fields.get("records") {
        if !records.is_array() {
            return Err("records must be an array".to_owned());
        }
        options.concepts.records = Some(records.clone());
    }
    if let Some(definitions) = fields.get("formalDefinitions") {
        if !definitions.is_object() {
            return Err("formal definitions must be a record".to_owned());
        }
        options.concepts.formal_definitions = Some(definitions.clone());
    }
    if let Some(equivalences) = fields.get("equivalences") {
        let error = "equivalences must be pairs of source identities";
        for pair in equivalences.as_array().ok_or(error)? {
            let pair = pair
                .as_array()
                .filter(|pair| pair.len() == 2)
                .ok_or(error)?;
            options.concepts.equivalences.push([
                pair[0].as_str().ok_or(error)?.to_owned(),
                pair[1].as_str().ok_or(error)?.to_owned(),
            ]);
        }
    }
    let mut limits = MetaLanguageArchiveLimits::default();
    for (field, target) in [
        ("nodeLimit", &mut limits.node_limit),
        ("referenceLimit", &mut limits.reference_limit),
        ("byteLimit", &mut limits.byte_limit),
    ] {
        if let Some(value) = fields.get(field) {
            *target = positive(value)?;
        }
    }
    options.limits = Some(limits);
    if let Some(value) = fields.get("depthLimit") {
        options.depth_limit = Some(positive(value)?);
    }
    Ok(options)
}
fn positive(value: &Value) -> Result<usize, String> {
    let number = value
        .as_f64()
        .ok_or("dynamic meta-language: invalid resource limit")?;
    if !number.is_finite()
        || number < 1.0
        || number > 9_007_199_254_740_991.0
        || number.fract() != 0.0
    {
        return Err("dynamic meta-language: invalid resource limit".to_owned());
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let count = number as u64;
    usize::try_from(count).map_err(|_| "dynamic meta-language: invalid resource limit".to_owned())
}
pub(super) fn parse_options(
    options: &BTreeMap<String, String>,
) -> Result<DynamicParseOptions, String> {
    let mut result = DynamicParseOptions::default();
    result.projection = rules::read_dynamic_language_projection(
        options.get("--projection").map_or("cst", String::as_str),
    );
    if result.projection.is_empty() {
        return Err("unknown syntax projection".to_owned());
    }
    for (option, value) in options {
        let field = rules::read_dynamic_language_resource_field(option);
        if field.is_empty() {
            continue;
        }
        if value.is_empty()
            || value.len() > 1 && value.starts_with('0')
            || !value.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err("a parser limit needs a nonnegative safe integer".to_owned());
        }
        let count = value
            .parse::<u64>()
            .ok()
            .filter(|value| *value <= 9_007_199_254_740_991)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or("a parser limit needs a nonnegative safe integer")?;
        match field.as_str() {
            "stepLimit" => result.parser.step_limit = Some(count),
            "memoryLimit" => result.parser.memory_limit = Some(count),
            "maxDepth" => result.parser.max_depth = Some(count),
            "memoLimit" => result.parser.memo_limit = Some(count),
            "maxRepairs" => result.parser.max_repairs = Some(count),
            _ => return Err("unknown parser resource field".to_owned()),
        }
    }
    result.parser.start_rule = options.get("--start-rule").cloned();
    result.identifier = options.get("--identifier").cloned();
    result.replace_snapshot = options.contains_key("--replace-snapshot");
    if options.contains_key("--error-recovery") {
        result.parser.error_recovery = Some(true);
    }
    if options.contains_key("--accept-recovery") {
        result.parser.accept_recovery = Some(true);
    }
    if options.contains_key("--reject-ambiguity") {
        result.parser.reject_ambiguity = Some(true);
    }
    Ok(result)
}
