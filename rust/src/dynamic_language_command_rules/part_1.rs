// Generated from complete js/src/dynamic-language-command-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_dynamic_language_command(command: &str) -> bool {
    (((((((((command == "prepare") || (command == "extend")) || (command == "parse"))
        || (command == "list"))
        || (command == "grammar"))
        || (command == "discover"))
        || (command == "refine"))
        || (command == "install"))
        || (command == "infer-attribute"))
        || (command == "translate-attributes")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_dynamic_language_argument_arity(command: &str, option: &str) -> f64 {
    if (self::accept_dynamic_language_command(command)) {
        if command == "infer-attribute" {
            if option == "--grammar" { 1f64 } else { -1f64 }
        } else if command == "translate-attributes" {
            if (option == "--input-binary") || (option == "--input-link-cli-store") {
                0f64
            } else if (((((option == "--source") || (option == "--input"))
                || (option == "--target"))
                || (option == "--selections"))
                || (option == "--start-rule"))
                || (!(self::read_dynamic_language_resource_field(option)).is_empty())
            {
                1f64
            } else {
                -1f64
            }
        } else if (option == "--binary") || (option == "--link-cli-store") {
            if (command == "list") || (command == "grammar") {
                -1f64
            } else {
                0f64
            }
        } else if (option == "--input-binary") || (option == "--input-link-cli-store") {
            if (command == "prepare") || (command == "discover") {
                -1f64
            } else {
                0f64
            }
        } else if command == "discover" {
            if ((option == "--identifier") || (option == "--language")) || (option == "--edition") {
                1f64
            } else {
                -1f64
            }
        } else if command == "refine" {
            if (option == "--examples") || (option == "--identifier") {
                1f64
            } else {
                -1f64
            }
        } else if command == "install" {
            if (((option == "--base") || (option == "--base-encoding"))
                || (option == "--merge-with"))
                || (option == "--identifier")
            {
                1f64
            } else {
                -1f64
            }
        } else if option == "--source" {
            if (command == "prepare") || (command == "extend") {
                4f64
            } else {
                1f64
            }
        } else if option == "--configuration" {
            if (command == "prepare") || (command == "extend") {
                1f64
            } else {
                -1f64
            }
        } else if option == "--replace" {
            if command == "extend" { 0f64 } else { -1f64 }
        } else if ((((((((option == "--input") || (option == "--identifier"))
            || (option == "--projection"))
            || (option == "--start-rule"))
            || (option == "--step-limit"))
            || (option == "--memory-limit"))
            || (option == "--max-depth"))
            || (option == "--memo-limit"))
            || (option == "--max-repairs")
        {
            if command == "parse" { 1f64 } else { -1f64 }
        } else if (((option == "--error-recovery") || (option == "--accept-recovery"))
            || (option == "--reject-ambiguity"))
            || (option == "--replace-snapshot")
        {
            if command == "parse" { 0f64 } else { -1f64 }
        } else {
            -1f64
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_dynamic_language_projection(projection: &str) -> String {
    if (projection == "cst") || (projection == "concrete syntax") {
        String::from("concrete syntax")
    } else if (projection == "ast") || (projection == "abstract syntax") {
        String::from("abstract syntax")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_dynamic_language_configuration_field(field: &str) -> bool {
    ((((((field == "records") || (field == "formalDefinitions")) || (field == "equivalences"))
        || (field == "nodeLimit"))
        || (field == "referenceLimit"))
        || (field == "byteLimit"))
        || (field == "depthLimit")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_dynamic_language_resource_field(option: &str) -> String {
    if option == "--step-limit" {
        String::from("stepLimit")
    } else if option == "--memory-limit" {
        String::from("memoryLimit")
    } else if option == "--max-depth" {
        String::from("maxDepth")
    } else if option == "--memo-limit" {
        String::from("memoLimit")
    } else if option == "--max-repairs" {
        String::from("maxRepairs")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_dynamic_language_base_encoding(encoding: &str) -> bool {
    ((encoding == "text") || (encoding == "binary")) || (encoding == "link-cli-store")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_dynamic_language_command_usage() -> String {
    String::from(
        "usage: meta-language language prepare --source <id> <language> <edition> <grammar.lino> ... [--configuration <links-file>] [--binary]\n       meta-language language extend <archive> --source <id> <language> <edition> <grammar.lino> ... [--replace] [--configuration <links-file>] [--input-binary] [--binary]\n       meta-language language parse <archive> --source <id> --input <file> [--projection cst|ast] [--identifier <name>] [--replace-snapshot] [--start-rule <rule>]\n         [--step-limit <n>] [--memory-limit <n>] [--max-depth <n>] [--memo-limit <n>] [--max-repairs <n>]\n         [--error-recovery] [--accept-recovery] [--reject-ambiguity] [--input-binary] [--binary]\n       meta-language language list|grammar <archive> [--source <id>] [--input-binary]\n       meta-language language discover <examples.lino> [--identifier <id>] [--language <name>] [--edition <edition>] [--binary]\n       meta-language language refine <checkpoint> --examples <examples.lino> [--identifier <id>] [--input-binary] [--binary]\n       meta-language language install <checkpoint> [--input-binary] [--binary]\n         [--base <archive> --merge-with <source> --identifier <merged> [--base-encoding text|binary|link-cli-store]]\n       meta-language language infer-attribute <observations.lino> --grammar <grammar.lino>\n       meta-language language translate-attributes <archive> --source <id> --input <file> --target JavaScript|Rust --selections <selections.lino> [--input-binary]\nUse --link-cli-store or --input-link-cli-store in place of the corresponding binary option for native link-cli stores.\nParse writes the complete extended archive; CST is the default. AST retains its original CST. Configuration is structured Links data.\n",
    )
}
