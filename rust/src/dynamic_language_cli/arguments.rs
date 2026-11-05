use super::{BTreeMap, rules};
pub(super) struct Arguments {
    pub command: String,
    pub sources: Vec<[String; 4]>,
    pub options: BTreeMap<String, String>,
    pub file: Option<String>,
}
pub(super) fn parse(args: &[String]) -> Result<Arguments, String> {
    let command = args.first().map_or("", String::as_str);
    if !rules::accept_dynamic_language_command(command) {
        return Err("unknown language operation".to_owned());
    }
    let mut result = Arguments {
        command: command.to_owned(),
        sources: Vec::new(),
        options: BTreeMap::new(),
        file: None,
    };
    let mut index = 1;
    while index < args.len() {
        let option = &args[index];
        if !option.starts_with("--") && command != "prepare" && result.file.is_none() {
            result.file = Some(option.clone());
            index += 1;
            continue;
        }
        let arity = rules::read_dynamic_language_argument_arity(command, option);
        let count = match arity {
            0.0 => 0,
            1.0 => 1,
            4.0 => 4,
            _ => {
                return Err(format!(
                    "unexpected or incomplete language argument {option}"
                ));
            }
        };
        if index + count >= args.len() {
            return Err(format!(
                "unexpected or incomplete language argument {option}"
            ));
        }
        if option == "--source" && count == 4 {
            result
                .sources
                .push(std::array::from_fn(|at| args[index + 1 + at].clone()));
        } else if result
            .options
            .insert(
                option.clone(),
                if count == 0 {
                    String::new()
                } else {
                    args[index + 1].clone()
                },
            )
            .is_some()
        {
            return Err(format!("duplicate language argument {option}"));
        }
        index += count + 1;
    }
    if command != "prepare" && result.file.is_none() {
        return Err("a language archive input is required".to_owned());
    }
    Ok(result)
}
